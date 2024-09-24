use leptos::*;
use leptos_meta::*;

/// 提供登陆页
#[component]
pub fn RegistrationPage() -> impl IntoView {
    let (username, set_username) = create_signal("".to_string());
    let (password, set_password) = create_signal("".to_string());
    let (confirm_password, set_confirm_password) = create_signal("".to_string());
    let (fullname, set_fullname) = create_signal("".to_string());
    let (email, set_email) = create_signal("".to_string());
    let (mobile_no, set_mobile_no) = create_signal("".to_string());
    let (mobile_verify_code, set_mobile_verify_code) = create_signal("".to_string());

    let input_username: NodeRef<html::Input> = create_node_ref();
    let input_password: NodeRef<html::Input> = create_node_ref();
    let input_confirm_password: NodeRef<html::Input> = create_node_ref();
    let input_fullname: NodeRef<html::Input> = create_node_ref();
    let input_email: NodeRef<html::Input> = create_node_ref();
    let input_mobile_no: NodeRef<html::Input> = create_node_ref();
    let input_mobile_verify_code: NodeRef<html::Input> = create_node_ref();

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        // stop the page from reloading!
        ev.prevent_default();

        let username_value = input_username
            .get()
            .expect("<input> should be mounted")
            .value();

        let password_value = input_password
            .get()
            .expect("<input> should be mounted")
            .value();

        let confirm_password_value = input_confirm_password
            .get()
            .expect("<input> should be mounted")
            .value();

        let fullname_value = input_fullname
            .get()
            .expect("<input> should be mounted")
            .value();

        let email_value = input_email
            .get()
            .expect("<input> should be mounted")
            .value();

        let mobile_no_value = input_mobile_no
            .get()
            .expect("<input> should be mounted")
            .value();

        let mobile_verify_code_value = input_mobile_verify_code
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
                                        value=username
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
                                        value=password
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
                                        value=confirm_password
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
                                        value=fullname
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
                                        value=email
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
                                        value=mobile_no
                                        node_ref=input_mobile_no
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
                                        value=mobile_verify_code
                                        node_ref=input_mobile_verify_code
                                    />
                                    <button class="registration" style="margin-left:15px">
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
                <div style="padding-top:100px;magin-top:50px;" >
                    <a href="/">
                        "返回主页"
                    </a>
                </div>
            </div>
        </div>
    }
}
