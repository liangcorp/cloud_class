use cfg_if::cfg_if;
use leptos::*;
use leptos_meta::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InputRegistrationInfo {
    username: String,
    password: String,
    confirm_password: String,
    fullname: String,
    email: String,
    mobile_num: String,
    mobile_verify_code: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum InputRegistrationErrorKind {
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

impl Default for InputRegistrationInfo {
    fn default() -> InputRegistrationInfo {
        InputRegistrationInfo {
            username: "".to_string(),
            password: "".to_string(),
            confirm_password: "".to_string(),
            fullname: "".to_string(),
            email: "".to_string(),
            mobile_num: "".to_string(),
            mobile_verify_code: "".to_string(),
        }
    }
}

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use crate::state::AppState;

        fn is_valid_password(input_password: &str) -> bool {
            if input_password.len() < 8 || input_password.len() > 100 {
                return false;
            }

            true
        }

        fn is_password_match(input_password: &str, input_confirm: &str) -> bool {
            input_password == input_confirm
        }

        fn verify_input_content(input_reg: InputRegistrationInfo) -> Option<InputRegistrationErrorKind> {
            //  取得软件状态
            let state = match use_context::<AppState>() {
                Some(s) => s,
                None => return Some(InputRegistrationErrorKind::UnknowError),
            };

            let validation_regex = state.validation_regex;

            if !validation_regex.get_username_regex().is_match(&input_reg.username) {
                return Some(InputRegistrationErrorKind::InvalidUsername);
            } else if !is_valid_password(&input_reg.password) {
                return Some(InputRegistrationErrorKind::InvalidPassword);
            } else if !is_password_match(&input_reg.password, &input_reg.confirm_password) {
                return Some(InputRegistrationErrorKind::PasswordNotMatch);
            } else if !validation_regex.get_email_regex().is_match(&input_reg.email) {
                return Some(InputRegistrationErrorKind::InvalidEmailAddress);
            } else if !validation_regex.get_mobile_num_regex().is_match(&input_reg.mobile_num) {
                return Some(InputRegistrationErrorKind::InvalidMobileNumber);
            } else if !validation_regex.get_mobile_code_regex().is_match(&input_reg.mobile_verify_code) {
                return Some(InputRegistrationErrorKind::InvalidMobileVerifyCode);
            }
            None
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
) -> Result<Option<InputRegistrationErrorKind>, ServerFnError> {
    match verify_input_content(input_reg) {
        Some(error) => Ok(Some(error)),
        None => Ok(None),
    }
}

/// 提供注册页
#[component]
pub fn RegistrationPage() -> impl IntoView {
    let (reg_error_message, set_reg_error_message) = create_signal("".to_string());
    let (is_show, set_is_show) = create_signal(false);

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
                        set_is_show.set(false);
                        set_reg_error_message.set("".to_string());
                    },
                    Some(InputRegistrationErrorKind::InvalidPassword) => {
                        set_is_show.set(true);
                        set_reg_error_message.set("密码无效".to_string());
                    }
                    Some(InputRegistrationErrorKind::PasswordNotMatch) => {
                        set_is_show.set(true);
                        set_reg_error_message.set("确认密码不符".to_string());
                    }
                    Some(InputRegistrationErrorKind::InvalidMobileVerifyCode) => {
                        set_is_show.set(true);
                        set_reg_error_message.set("验证码无效".to_string());
                    }
                    Some(InputRegistrationErrorKind::MobileVerifyFailed) => {
                        set_is_show.set(true);
                        set_reg_error_message.set("验证码错误".to_string());
                    }
                    Some(InputRegistrationErrorKind::InvalidUsername) => {
                        set_is_show.set(true);
                        set_reg_error_message.set("用户名无效".to_string());
                    }
                    Some(InputRegistrationErrorKind::InvalidMobileNumber) => {
                        set_is_show.set(true);
                        set_reg_error_message.set("手机号无效".to_string());
                    }
                    Some(InputRegistrationErrorKind::InvalidEmailAddress) => {
                        set_is_show.set(true);
                        set_reg_error_message.set("邮件地址无效".to_string());
                    }
                    Some(InputRegistrationErrorKind::InvalidFullName) => {
                        set_is_show.set(true);
                        set_reg_error_message.set("姓名无效".to_string());
                    }
                    Some(InputRegistrationErrorKind::UnknowError) => {
                        set_is_show.set(true);
                        set_reg_error_message.set("系统问题请稍后再试".to_string());
                    }
                },
                Err(_) => {
                    set_reg_error_message.set("系统问题请稍后再试".to_string());
                    set_is_show.set(false)
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
                            <tr class:display=move || is_show.get()>
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
