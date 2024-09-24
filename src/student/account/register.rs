use leptos::*;
use leptos_meta::*;

/// 提供登陆页
#[component]
pub fn RegistrationPage() -> impl IntoView {

    view! {
        <Title text="学员注册" />

        <div class="full-height">
            <div class="register-div" align="center">
            <img src="images/registration/registration_logo.png" style="height:50px;width:50px;" />
            <hr style="width:100%" />
                // <form on:submit=on_submit>
                <form style="margin-top:40px">
                    <table>
                        // <tr style:display=move || auth_success.get() style="color:red">
                        //     <td>
                        //         <h4>"用户名或者密码不正确"</h4>
                        //     </td>
                        // </tr>
                        <tr>
                            <td>"用户名"</td>
                            <td style="padding-left:10px">
                                <input
                                    placeholder="请输入用户名"
                                    class="login-form"
                                    style="width:100%"
                                    type="text"
                                    // value=username
                                    // node_ref=input_username
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
                                    // value=password
                                    // node_ref=input_password
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
                                    // value=cnfrm_password
                                    // node_ref=input_cnfrm_password
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
                                    // value=fullname
                                    // node_ref=input_fullname
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
                                    // value=email
                                    // node_ref=input_email
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
                                    // value=mobile_no
                                    // node_ref=input_mobile_no
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
                                    // value=mobl_verify
                                    // node_ref=input_mobl_verify
                                />
                                <button class="registration" style="margin-left:15px">"获取验证码"</button>
                            </td>
                        </tr>
                        <tr >
                            <td style="padding-top:15px;"><input class="submit-button" type="submit" value="注册"/></td>
                            <td style="padding-top:15px;padding-left:10px;">
                                <a href="/login" class="login-switch">"返回登陆"</a>
                            </td>
                        </tr>
                    </table>
                </form>
            </div>
        </div>
    }
}
