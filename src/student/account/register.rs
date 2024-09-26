use cfg_if::cfg_if;
use leptos::*;
use leptos_meta::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InputRegistrationInfo {
    username: String,
    fullname: String,
    not_valid_password: bool,
    not_match_password: bool,
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
        pub enum RegistrationInputErrors {
            InvalidUsername,
            InvalidPassword,
            PasswordNotMatch,
            InvalidFullName,
            InvalidEmailAddress,
            InvalidMobileNumber,
            InvalidMobileVerifyCode,
            MobileVerifyFailed,
            UnknowError,
        }

        impl fmt::Display for RegistrationInputErrors {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                match self {
                    RegistrationInputErrors::InvalidUsername => write!(f, "用户名无效"),
                    RegistrationInputErrors::InvalidPassword => write!(f, "密码无效"),
                    RegistrationInputErrors::PasswordNotMatch => write!(f, "确认密码不符"),
                    RegistrationInputErrors::InvalidFullName => write!(f, "姓名无效"),
                    RegistrationInputErrors::InvalidEmailAddress => write!(f, "电子邮件无效"),
                    RegistrationInputErrors::InvalidMobileNumber => write!(f, "手机号无效"),
                    RegistrationInputErrors::InvalidMobileVerifyCode => write!(f, "验证码无效"),
                    RegistrationInputErrors::MobileVerifyFailed => write!(f, "验证码错误"),
                    RegistrationInputErrors::UnknowError => write!(f, "系统问题请稍后再试"),
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

        fn is_valid_fullname(input_full_name: &str) -> bool {
            //  whitespace and '.' are allowed in full name
            //  user input must be sanitized before inserting into database
            if input_full_name.is_empty()
                || input_full_name.len() > 60
                || input_full_name.chars().any(|c| !c.is_alphabetic() && c != '.' && c != ' ') {
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

        fn verify_input_content(input_reg: InputRegistrationInfo) -> Option<String> {
            //  取得软件状态
            let state = match use_context::<AppState>() {
                Some(s) => s,
                None => return Some(RegistrationInputErrors::UnknowError.to_string()),
            };

            let validation_regex = state.validation_regex;

            if !is_valid_username(&validation_regex, &input_reg.username) {
                return Some(RegistrationInputErrors::InvalidUsername.to_string());
            } else if input_reg.not_valid_password {
                return Some(RegistrationInputErrors::InvalidPassword.to_string());
            } else if input_reg.not_match_password {
                return Some(RegistrationInputErrors::PasswordNotMatch.to_string());
            } else if !is_valid_fullname(&input_reg.fullname) {
                return Some(RegistrationInputErrors::InvalidFullName.to_string());
            } else if !is_valid_email(&validation_regex, &input_reg.email)  {
                return Some(RegistrationInputErrors::InvalidEmailAddress.to_string());
            } else if !validation_regex.get_mobile_num_regex().is_match(&input_reg.mobile_num) {
                return Some(RegistrationInputErrors::InvalidMobileNumber.to_string());
            } else if !validation_regex.get_mobile_code_regex().is_match(&input_reg.mobile_verify_code) {
                return Some(RegistrationInputErrors::InvalidMobileVerifyCode.to_string());
            }

            None
        }

        fn create_user(input_reg: InputRegistrationInfo) -> Result<(), ServerFnError> {
            Ok(())
        }
    }
}

#[server]
pub async fn send_mobile_code(mobile_num: String) -> Result<(), ServerFnError> {
    use crate::utils::*;
    let num: String = format!(
        "{}",
        rapid::rapidhash(&uuid::get_random_token().into_bytes())
    );
    logging::log!("{}: {}", mobile_num, &num[..6]);
    // probably need redis caching
    Ok(())
}

#[server]
pub async fn commit_user(
    input_reg: InputRegistrationInfo,
) -> Result<Option<String>, ServerFnError> {
    match verify_input_content(input_reg) {
        Some(error) => Ok(Some(error)),
        None => Ok(None),
    }
}

/// 提供注册页
#[component]
pub fn RegistrationPage() -> impl IntoView {
    let (reg_error_message, set_reg_error_message) = create_signal("".to_string());
    let (is_not_valid, set_not_valid) = create_signal(false);

    let input_username: NodeRef<html::Input> = create_node_ref();
    let input_password: NodeRef<html::Input> = create_node_ref();
    let input_confirm_password: NodeRef<html::Input> = create_node_ref();
    let input_fullname: NodeRef<html::Input> = create_node_ref();
    let input_email: NodeRef<html::Input> = create_node_ref();
    let input_m_num: NodeRef<html::Input> = create_node_ref();
    let input_m_verify_code: NodeRef<html::Input> = create_node_ref();

    let on_click = move |_| {
        let m_num_value = input_m_num
            .get()
            .expect("<input> should be mounted")
            .value();

        spawn_local(async move {
            let _ = send_mobile_code(m_num_value).await;
        });
    };

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        // stop the page from reloading!
        ev.prevent_default();

        let password = input_password
            .get()
            .expect("<input> should be mounted")
            .value();

        let confirm_password = input_confirm_password
            .get()
            .expect("<input> should be mounted")
            .value();

        let input_reg_info = InputRegistrationInfo {
            username: input_username
                .get()
                .expect("<input> should be mounted")
                .value(),

            fullname: input_fullname
                .get()
                .expect("<input> should be mounted")
                .value(),

            //  not a valid password if it's empty,
            //  contains whitespace, less than 8 characters
            //  long and more than 100 characters
            not_valid_password: password.is_empty()
                || password.chars().any(|c| c.is_whitespace())
                || password.len() < 8
                || password.len() > 100,

            not_match_password: password != confirm_password,

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
                    set_not_valid.set(false)
                    set_reg_error_message.set("系统问题请稍后再试".to_string());
                }
            }
        })
    };

    view! {
        <Title text="学员注册" />

        <div class="full-height">
            <div class="register-div" align="center">
                <div>
                    <img src="images/registration/registration_logo.png" class="login-register" />
                    <hr class="login-register" />
                </div>
                <div>
                    <form on:submit=on_submit style="margin-top:40px">
                        <table>
                            // error message
                            <tr class:display=move || is_not_valid.get()>
                                <td></td>
                                <td>
                                    <p style="color:red">{reg_error_message}</p>
                                </td>
                            </tr>
                            // actual form
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
                                    <button
                                        on:click=on_click
                                        class="registration"
                                        style="margin-left:15px"
                                    >
                                        "获取验证码"
                                    </button>
                                </td>
                            </tr>
                        </table>
                        <table>
                            <tr>
                                <td style="padding-top:15px;">
                                    <input class="submit-button" type="submit" value="注册" />
                                </td>
                                <td style="padding-top:15px;padding-left:10px;">
                                    <a href="/login" class="login-switch">
                                        "返回登陆"
                                    </a>
                                </td>
                            </tr>
                        </table>
                    </form>
                </div>
                <div style="padding-top:100px;">
                    <a href="/">"返回主页"</a>
                </div>
            </div>
        </div>
    }
}
