pub mod mobile;
pub mod qr;
pub mod username;

use leptos::*;
use leptos_meta::*;

use mobile::MobileLoginLayer;
use qr::QRLayer;
use username::UsernameLoginLayer;

/// 提供登陆页
#[component]
pub fn LoginPage() -> impl IntoView {
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
                                <LoginLayer />
                            </td>
                            <td align="center">
                                <QRLayer />
                            </td>
                        </tr>
                    </table>
                </div>
                <div style="padding-top:100px;">
                    <a href="/">"返回主页"</a>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn LoginLayer() -> impl IntoView {
    let (show_layer, set_show_layer) = create_signal(true);
    view! {
        <div style="padding:20px;top:0px">
            <div style="display:inline-block; padding-right:20px">
                <a
                    href="#"
                    class="login-switch"
                    on:click=move |_| {
                        set_show_layer.update(|n| *n = true);
                    }
                >
                    "密码登录"
                </a>
            </div>
            <div style="display:inline-block;">
                <a
                    href="#"
                    class="login-switch"
                    on:click=move |_| {
                        set_show_layer.update(|n| *n = false);
                    }
                >
                    "短信登录"
                </a>
            </div>
        </div>

        <div class:display=move || !show_layer.get()>
            <UsernameLoginLayer />
        </div>

        <div class:display=move || show_layer.get()>
            <MobileLoginLayer />
        </div>
    }
}
