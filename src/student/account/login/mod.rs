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
                <div>
                    <img class="login-register" src="images/users/authentication_page_logo.jpg" />
                    <hr class="login-register" />
                </div>
                <div>
                    <table>
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
                </div>
                <div style="padding-top:100px;" >
                    <a href="/">
                        "返回主页"
                    </a>
                </div>
            </div>
        </div>
    }
}
