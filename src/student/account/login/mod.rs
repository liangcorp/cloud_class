pub mod username;
pub mod mobile;
pub mod qr;

use leptos::*;
use leptos_meta::*;

use username::UsernameLoginLayer;
use mobile::MobileLoginLayer;
use qr::QRLayer;

/// 提供登陆页
#[component]
pub fn LoginPage() -> impl IntoView {
    let (username_login, set_username_login) = create_signal("".to_string());
    let (mobile_login, set_mobile_login) = create_signal("none".to_string());

    view! {
        <Title text="浩天数智化教学" />

        <div class="full-height">
            <div class="login_div" align="center">
                <table>
                    <tr>
                        <td style="padding: 20px">
                            <hr width="350px" size="1" color="#BFBFBF" noshade />
                        </td>
                        <td>
                            <img src="images/logo1.png" />
                        </td>
                        <td style="padding: 20px">
                            <hr width="350px" size="1" color="#BFBFBF" noshade />
                        </td>
                    </tr>

                    <tr>
                        <td>
                            <div style="padding:20px;top:0px">
                                <table>
                                    <tr>
                                        <td>
                                            <a
                                                href="#"
                                                class="login_switch"
                                                on:click=move |_| {
                                                    set_username_login.update(|n| *n = String::from("inline"));
                                                    set_mobile_login.update(|n| *n = String::from("none"));
                                                }
                                            >
                                                密码登录
                                            </a>
                                        </td>
                                        <td>
                                            <a
                                                href="#"
                                                class="login_switch"
                                                on:click=move |_| {
                                                    set_username_login.update(|n| *n = String::from("none"));
                                                    set_mobile_login.update(|n| *n = String::from("inline"));
                                                }
                                            >
                                                短信登录
                                            </a>
                                        </td>
                                    </tr>
                                </table>
                            </div>

                            <div style:display=move || username_login.get()>
                                <UsernameLoginLayer />
                            </div>
                            <b>{move || username_login.get()}</b>

                            <div style:display=move || mobile_login.get()>
                                <MobileLoginLayer />
                            </div>
                        </td>
                        <td></td>
                        <td align="center">
                            <QRLayer />
                        </td>
                    </tr>
                </table>
                <br />
                <br />
                <br />
                <a href="/">返回主页</a>
            </div>
        </div>
    }
}
