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
    let (show_layer, set_show_layer) = create_signal(true);

    view! {
        <Title text="学员登陆" />

        <div class="full-height">
            <div class="login-div" align="center">
                <table>
                    <tr>
                        <td style="padding: 20px">
                            <hr width="350px" size="1" color="#BFBFBF" noshade />
                        </td>
                        <td>
                            <img
                                style="width:50px;height:auto"
                                src="images/users/authentication_page_logo.jpg"
                            />
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
                                                class="login-switch"
                                                on:click=move |_| {
                                                    set_show_layer.update(|n| *n = true);
                                                }
                                            >
                                                "密码登录"
                                            </a>
                                        </td>
                                        <td>
                                            <a
                                                href="#"
                                                class="login-switch"
                                                on:click=move |_| {
                                                    set_show_layer.update(|n| *n = false);
                                                }
                                            >
                                                "短信登录"
                                            </a>
                                        </td>
                                    </tr>
                                </table>
                            </div>

                            <div class:display=move || !show_layer.get()>
                                <UsernameLoginLayer />
                            </div>

                            <div class:display=move || show_layer.get()>
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
                <a href="/">"返回主页"</a>
            </div>
        </div>
    }
}
