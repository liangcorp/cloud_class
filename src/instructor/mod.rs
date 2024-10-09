mod account;

use leptos::*;
use leptos_meta::*;
use account::login::{username::UsernameLoginLayer, mobile::MobileLoginLayer, qr::QRLayer};

#[component]
pub fn InstructorPage() -> impl IntoView {
    use crate::header::HeaderSection;

    view! {
        <Title text="教师中心" />

        <HeaderSection />

        <div class="contents">
            <LoginPage />
        </div>
    }
}

/// 提供登陆页
#[component]
pub fn LoginPage() -> impl IntoView {
    view! {
        <div align="center">
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
            // <Await
            // future=|| get_public_key()
            // let:public_key
            // >
            // {match public_key {
            // Ok(ok_pub_key) => {
            // match ok_pub_key {
            // Some(some_pub_key) => {
            // view! { <UsernameLoginLayer pub_key=some_pub_key.clone() /> }
            // .into_view()
            // }
            // None => view! {}.into_view(),
            // }
            // }
            // Err(_) => view! {}.into_view(),
            // }}
            //
            // </Await>
            <UsernameLoginLayer />
        </div>

        <div class:display=move || show_layer.get()>
            <MobileLoginLayer />
        </div>
    }
}
