use leptos::*;
use leptos_meta::*;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct InputRegistrationInfo {
    username: String,
    password: String,
    confirm_password: String,
    fullname: String,
    email: String,
    mobile_num: String,
    mobile_verify_code: String,
}

enum InputRegistrationErrorKind {
    None,
    PasswordNotMatch,
    MobileVerifyFailed,
    InvalidMobileVerifyCode,
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

#[server]
pub async fn send_mobile_code(mobile_num: String) -> Result<(), ServerFnError> {
    use crate::utils::*;
    let num: String = format!("{}", rapid::rapidhash(&uuid::get_random_token().into_bytes()));
    logging::log!("{}: {}", mobile_num, &num[..6]);
    Ok(())
}

/// 提供注册页
#[component]
pub fn RegistrationPage() -> impl IntoView {
    let (reg_error, set_reg_error) = create_signal(InputRegistrationErrorKind::None);

    let input_username: NodeRef<html::Input> = create_node_ref();
    let input_password: NodeRef<html::Input> = create_node_ref();
    let input_confirm_password: NodeRef<html::Input> = create_node_ref();
    let input_fullname: NodeRef<html::Input> = create_node_ref();
    let input_email: NodeRef<html::Input> = create_node_ref();
    let input_m_number: NodeRef<html::Input> = create_node_ref();
    let input_m_verify_code: NodeRef<html::Input> = create_node_ref();

    let mut input_reg_info = InputRegistrationInfo::default();

    let on_click = move |_| {
        let m_num_value = input_m_number
            .get()
            .expect("<input> should be mounted")
            .value();

        spawn_local(
            async move {
                let _ = send_mobile_code(m_num_value).await;
            });
    };

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        // stop the page from reloading!
        ev.prevent_default();

        input_reg_info.username = input_username
            .get()
            .expect("<input> should be mounted")
            .value();

        input_reg_info.password = input_password
            .get()
            .expect("<input> should be mounted")
            .value();

        input_reg_info.confirm_password = input_confirm_password
            .get()
            .expect("<input> should be mounted")
            .value();

        input_reg_info.fullname = input_fullname
            .get()
            .expect("<input> should be mounted")
            .value();

        input_reg_info.email = input_email
            .get()
            .expect("<input> should be mounted")
            .value();

        input_reg_info.mobile_num = input_m_number
            .get()
            .expect("<input> should be mounted")
            .value();

        input_reg_info.mobile_verify_code = input_m_verify_code
            .get()
            .expect("<input> should be mounted")
            .value();
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
                            // <tr style:display=move || auth_success.get() style="color:red">
                            // <td>
                            // <h4>"用户名或者密码不正确"</h4>
                            // </td>
                            // </tr>
                            <tr>
                                <td>"用户名"</td>
                                <td style="padding-left:10px">
                                    <input
                                        placeholder="请输入用户名"
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
                                        placeholder="请输入密码"
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
                                        node_ref=input_m_number
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
