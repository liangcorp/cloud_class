use cfg_if::cfg_if;
use leptos::*;
use leptos_meta::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InputRegistrationInfo {
    username: String,
    fullname: String,
    password: String,
    confirm_password: String,
    email: String,
    mobile_num: String,
    mobile_verify_code: String,
}

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use std::fmt;
        use crate::state::AppState;
        use crate::utils::regex::InputValidationRegex;

        #[derive(Clone, Debug)]
        pub enum RegistrationError {
            InvalidUsername,
            InvalidPassword,
            PasswordNotMatch,
            InvalidFullName,
            InvalidEmailAddress,
            InvalidMobileNumber,
            InvalidMobileVerifyCode,
            MobileVerifyFailed,
            ExistingUsername,
            ExistingMobileNumber,
            ExistingEmailAddress,
            ErrorDuringUserCreation,
            UnknownError,
        }

        impl fmt::Display for RegistrationError {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                match self {
                    RegistrationError::InvalidUsername => write!(f, "用户名无效"),
                    RegistrationError::InvalidPassword => write!(f, "密码无效"),
                    RegistrationError::PasswordNotMatch => write!(f, "确认密码不符"),
                    RegistrationError::InvalidFullName => write!(f, "姓名无效"),
                    RegistrationError::InvalidEmailAddress => write!(f, "电子邮件无效"),
                    RegistrationError::InvalidMobileNumber => write!(f, "手机号无效"),
                    RegistrationError::InvalidMobileVerifyCode => write!(f, "验证码无效"),
                    RegistrationError::MobileVerifyFailed => write!(f, "验证码错误"),
                    RegistrationError::ExistingUsername => write!(f, "用户名以注册"),
                    RegistrationError::ExistingMobileNumber => write!(f, "手机号以注册"),
                    RegistrationError::ExistingEmailAddress => write!(f, "油箱以注册"),
                    RegistrationError::ErrorDuringUserCreation => write!(f, "用户注册失败"),
                    RegistrationError::UnknownError => write!(f, "系统问题请稍后再试"),
                }
            }
        }

        fn is_valid_username(validation_regex: &InputValidationRegex, input_username: &str) -> bool {
            //  between 5 - 30 charactors long without whitespace
            //  only allow alphabets and numbers
            //  can not start with number
            if input_username.chars().any(|c| c.is_whitespace())
                || !validation_regex.get_username_regex().is_match(input_username) {
                return false;
            }
            true
        }

        fn is_valid_password(input_password: &str) -> bool {
            //  not a valid password if it's empty,
            //  contains whitespace, less than 8 characters
            //  long and more than 100 characters
            if input_password.is_empty()
                || input_password.chars().any(|c| c.is_whitespace())
                || input_password.len() < 8
                || input_password.len() > 100 {
                    return false;
            }
            true
        }

        fn is_valid_fullname(input_fullname: &str) -> bool {
            //  whitespace and '.' are allowed in full name
            //  user input must be sanitized before inserting into database
            if input_fullname.is_empty()
                || input_fullname.len() > 60
                || input_fullname.chars().any(|c| !c.is_alphanumeric() && c != '.' && c != ' ') {
                return false;
            }
            true
        }

        fn is_valid_email(validation_regex: &InputValidationRegex, input_email: &str) -> bool {
            //  disallowed email address characters based on RFC 5322 standard
            //  domain names only allow big and small alphabets, numbers and '-'
            //  less than 256 characters long
            match input_email.chars().last() {
                Some(c) => if c == '.' || c == '-' { return false },
                None => return false,   // empty string
            }

            if !validation_regex.get_email_regex().is_match(input_email)
                || input_email.len() > 256 {
                return false;
            }

            let (mail_name, domain) = input_email.split_once('@').unwrap();

            if mail_name.contains('"')
                || validation_regex.get_email_forbidden_regex().is_match(mail_name)
                || domain.chars().any(|c| !c.is_ascii_alphanumeric() && c != '.' && c != '-') {
                return false;
            }

            true
        }

        fn verify_input_content(input_reg: &InputRegistrationInfo) -> Option<String> {
            //  取得软件状态
            let state = match use_context::<AppState>() {
                Some(s) => s,
                None => return Some(RegistrationError::UnknownError.to_string()),
            };

            let validation_regex = state.validation_regex;

            if !is_valid_username(&validation_regex, &input_reg.username) {
                return Some(RegistrationError::InvalidUsername.to_string());
            } else if !is_valid_password(&input_reg.password) {
                return Some(RegistrationError::InvalidPassword.to_string());
            } else if input_reg.password != input_reg.confirm_password {
                return Some(RegistrationError::PasswordNotMatch.to_string());
            } else if !is_valid_fullname(&input_reg.fullname) {
                return Some(RegistrationError::InvalidFullName.to_string());
            } else if !is_valid_email(&validation_regex, &input_reg.email)  {
                return Some(RegistrationError::InvalidEmailAddress.to_string());
            } else if !validation_regex.get_mobile_num_regex().is_match(&input_reg.mobile_num) {
                return Some(RegistrationError::InvalidMobileNumber.to_string());
            } else if !validation_regex.get_mobile_code_regex().is_match(&input_reg.mobile_verify_code) {
                return Some(RegistrationError::InvalidMobileVerifyCode.to_string());
            }

            None
        }

        fn create_salt_hash(password: &str) -> Result<(String, String), RegistrationError> {
            use crate::utils::crypto;

            let salt = crypto::get_salt();
            let password_hash = match crypto::get_parsed_hash(password, &salt) {
                Ok(ok_ph) => ok_ph,
                Err(_) => return Err(RegistrationError::ErrorDuringUserCreation),
            };

            Ok((salt, password_hash))
        }

        fn login_user(username: &str) -> Result<(), ServerFnError>{
            use crate::utils::uuid;
            use crate::session::{cookie::Cookie, cache::Cache};

            let session_token = uuid::get_session_token();

            Cookie::set_cookie(&session_token, false)?;
            Cache::set_cache(&session_token, username)?;

            //  改变网址到学生资料
            leptos_axum::redirect("/profile");
            Ok(())
        }
    }
}

#[server]
pub async fn send_mobile_code(mobile_num: String) -> Result<(), ServerFnError> {
    use crate::utils::{rapid, uuid};
    let num: String = format!(
        "{}",
        rapid::rapidhash(&uuid::get_random_token().into_bytes())
    );
    logging::log!("{}: {}", mobile_num, &num[..6]);
    // @TODO use an actual mobile service
    // @TODO redis caching
    Ok(())
}

#[server]
pub async fn commit_user(
    input_reg: InputRegistrationInfo,
) -> Result<Option<String>, ServerFnError> {
    use crate::state::AppState;
    use crate::utils::date;
    use sqlx::Error::Database;

    let registration_input_err = verify_input_content(&input_reg);

    if registration_input_err.is_none() {
        let (salt, password_hash) = match create_salt_hash(&input_reg.password) {
            Ok((ok_salt, ok_ph)) => (ok_salt, ok_ph),
            Err(_) => return Ok(None),
        };

        //  取得软件状态
        let state = match use_context::<AppState>() {
            Some(s) => s,
            None => {
                return Err(ServerFnError::Args(
                    "ERROR<user/account/register.rs>: during application state retrieval"
                        .to_string(),
                ))
            }
        };

        //  取得数据库信息
        let pool = state.pool;

        let sanitized_fullname = input_reg
            .fullname
            .chars()
            .map(|c| if c == ' ' { '_' } else { c })
            .collect::<String>();

        //  提取用户数据
        let sql_error = match sqlx::query("INSERT INTO students (username, salt, pw_hash, start_date, fullname, status, email, mobile)
            VALUES ($1, $2, $3, $4, $5, 'active', $6, $7);")
            .bind(&input_reg.username)
            .bind(&salt)
            .bind(&password_hash)
            .bind(date::get_current_date())
            .bind(&sanitized_fullname)
            .bind(&input_reg.email)
            .bind(&input_reg.mobile_num)
            .execute(&pool)
            .await {
                Ok(_) => {
                    match login_user(&input_reg.username) {
                        Ok(()) => return Ok(None),
                        Err(_) => return Ok(Some(RegistrationError::UnknownError.to_string())),
                    }
                },
                Err(sql_e) => sql_e,
            };

        match sql_error {
            Database(d_err) => match d_err.message() {
                "UNIQUE constraint failed: students.mobile" => {
                    return Ok(Some(RegistrationError::ExistingMobileNumber.to_string()))
                }
                "UNIQUE constraint failed: students.username" => {
                    return Ok(Some(RegistrationError::ExistingUsername.to_string()))
                }
                "UNIQUE constraint failed: students.email" => {
                    return Ok(Some(RegistrationError::ExistingEmailAddress.to_string()))
                }
                &_ => return Ok(Some(RegistrationError::UnknownError.to_string())),
            },
            _ => return Ok(Some(RegistrationError::UnknownError.to_string())),
        }
    }

    Ok(registration_input_err)
}

/// 提供注册页
#[component]
pub fn RegistrationPage() -> impl IntoView {
    view! {
        <Title text="学员注册" />

        <div class="full-height">
            <div class="register-div" align="center">
                <div>
                    <img src="images/registration/registration_logo.png" class="login-register" />
                    <hr class="login-register" />
                </div>
                <div>
                    <RegistrationForm />
                </div>
                <div style="padding-top:100px;">
                    <a href="/">"返回主页"</a>
                </div>
            </div>
        </div>
    }
}

/// @TODO ISLAND
#[component]
pub fn RegistrationForm() -> impl IntoView {
    let (reg_error_message, set_reg_error_message) = create_signal("".to_string());
    let (is_not_valid, set_not_valid) = create_signal(false);

    let input_username: NodeRef<html::Input> = create_node_ref();
    let input_password: NodeRef<html::Input> = create_node_ref();
    let input_confirm_password: NodeRef<html::Input> = create_node_ref();
    let input_fullname: NodeRef<html::Input> = create_node_ref();
    let input_email: NodeRef<html::Input> = create_node_ref();
    let input_m_num: NodeRef<html::Input> = create_node_ref();
    let input_m_verify_code: NodeRef<html::Input> = create_node_ref();

    let ignore_enter = move |ev: leptos::ev::KeyboardEvent| {
        if ev.code() == "Enter" {
            // stop the key action
            ev.prevent_default();
        }
    };

    let on_click = move |_| {
        let m_num_value = input_m_num
            .get()
            .expect("<input> should be mounted")
            .value();

        if m_num_value.len() != 11 || m_num_value.chars().any(|c| !c.is_numeric()) {
            set_not_valid.set(true);
            set_reg_error_message.set("手机号无效".to_string());
        } else {
            set_not_valid.set(false);
            set_reg_error_message.set("".to_string());

            spawn_local(async move {
                let _ = send_mobile_code(m_num_value).await;
            });
        }
    };

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        // stop the page from reloading!
        ev.prevent_default();

        let input_reg_info = InputRegistrationInfo {
            username: input_username
                .get()
                .expect("<input> should be mounted")
                .value(),

            password: input_password
                .get()
                .expect("<input> should be mounted")
                .value(),

            confirm_password: input_confirm_password
                .get()
                .expect("<input> should be mounted")
                .value(),

            fullname: input_fullname
                .get()
                .expect("<input> should be mounted")
                .value(),

            email: input_email
                .get()
                .expect("<input> should be mounted")
                .value(),

            mobile_num: input_m_num
                .get()
                .expect("<input> should be mounted")
                .value(),

            mobile_verify_code: input_m_verify_code
                .get()
                .expect("<input> should be mounted")
                .value(),
        };

        spawn_local(async move {
            match commit_user(input_reg_info).await {
                Ok(some_error) => match some_error {
                    None => {
                        set_not_valid.set(false);
                        set_reg_error_message.set("".to_string())
                    }
                    Some(error_message) => {
                        set_not_valid.set(true);
                        set_reg_error_message.set(error_message)
                    }
                },
                Err(_) => {
                    set_not_valid.set(false);
                    set_reg_error_message.set("系统问题请稍后再试".to_string())
                }
            }
        })
    };

    view! {
        <table>
            // error message
            <tr class:display=move || is_not_valid.get()>
                <td></td>
                <td>
                    <p style="color:red">{reg_error_message}</p>
                </td>
            </tr>
            // actual data form
            <tr>
                <td>"用户名"</td>
                <td style="padding-left:10px">
                    <input
                        placeholder="5-20位英文大小写字母加数字。必须英文字母开头"
                        class="login-form"
                        style="width:100%"
                        type="text"
                        node_ref=input_username
                    />
                </td>
            </tr>
            <tr>
                <td>"密码"</td>
                <td style="padding-left:10px">
                    <input
                        placeholder="密码必须在8位以上"
                        class="login-form"
                        style="width:100%"
                        type="password"
                        node_ref=input_password
                    />
                </td>
            </tr>
            <tr>
                <td>"确认密码"</td>
                <td style="padding-left:10px">
                    <input
                        placeholder="请确认密码"
                        class="login-form"
                        style="width:100%"
                        type="password"
                        node_ref=input_confirm_password
                    />
                </td>
            </tr>
            <tr>
                <td>"姓名"</td>
                <td style="padding-left:10px">
                    <input
                        placeholder="请输入姓名"
                        class="login-form"
                        style="width:100%"
                        type="text"
                        node_ref=input_fullname
                    />
                </td>
            </tr>
            <tr>
                <td>"邮件地址"</td>
                <td style="padding-left:10px">
                    <input
                        placeholder="请输入邮件地址"
                        class="login-form"
                        style="width:100%"
                        type="text"
                        node_ref=input_email
                    />
                </td>
            </tr>
            <tr>
                <td>"手机号"</td>
                <td style="padding-left:10px">
                    <input
                        placeholder="请输入手机号"
                        class="login-form"
                        style="width:100%"
                        type="text"
                        node_ref=input_m_num
                    />
                </td>
            </tr>
            <tr>
                <td></td>
                <td style="padding-left:10px">
                    <input
                        placeholder="请输入验证码"
                        class="login-form"
                        type="text"
                        node_ref=input_m_verify_code
                    />
                    <button on:click=on_click class="registration" style="margin-left:15px">
                        "获取验证码"
                    </button>
                </td>
            </tr>
        </table>
        <div style="display:inline-block;margin-left:auto;margin-right:10px;">
            <form on:submit=on_submit on:keydown=ignore_enter style="margin-top:40px">
                <input class="submit-button" type="submit" value="注册" />
            </form>
        </div>
        <div style="display:inline-block;margin-left:10px;margin-right:auto;">
            <a href="/login" class="login-switch">
                "返回登陆"
            </a>
        </div>
    }
}
