use leptos::*;
use leptos_meta::*;

/// 提供登陆页
#[component]
pub fn RegistrationPage() -> impl IntoView {

    view! {
        <Title text="学员注册" />

        <div class="full-height">
            <div class="register_div" align="center">
                // <form on:submit=on_submit>
                <form>
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
                                    class="login_form"
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
                                    class="login_form"
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
                                    class="login_form"
                                    style="width:100%"
                                    type="password"
                                    // value=cnfrm_password
                                    // node_ref=input_cnfrm_password
                                />
                            </td>
                        </tr>
                        <tr>
                            <td>"邮件地址"</td>
                            <td style="padding-left:10px">
                                <input
                                    placeholder="请输入邮件地址"
                                    class="login_form"
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
                                    class="login_form"
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
                                    class="login_form"
                                    type="text"
                                    // value=mobl_verify
                                    // node_ref=input_mobl_verify
                                />
                                <button style="margin-left:10px;padding:10px;font-size:22;background:#333333;font-weight:8;">"获取验证码"</button>
                            </td>
                        </tr>
                        <tr>
                            <td><button style="padding:10px;font-size:22;background:#333333;font-weight:8;">"注册"</button></td>
                            <td style="padding-left:10px">
                                <a href="/" class="header_menu">"返回主页"</a>
                            </td>
                        </tr>
                    </table>
                </form>
            </div>
        </div>
    }
}
