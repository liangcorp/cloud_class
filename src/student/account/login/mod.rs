pub mod mobile;
pub mod qr;
pub mod username;

use leptos::*;
use leptos_meta::*;

use mobile::MobileLoginLayer;
use qr::QRLayer;
use username::UsernameLoginLayer;

// Public/Private encryption for password. Not sure it is
// needed after reading stackexchange.
// https://security.stackexchange.com/questions/233759/passing-plain-text-password-over-https
// #[server]
// pub async fn get_public_key() -> Result<Option<RsaPublicKey>, ServerFnError> {
//     let mut rng = rand::thread_rng();
//     let bits = 2048;
//     let priv_key = match RsaPrivateKey::new(&mut rng, bits) {
//         Ok(ok_prive_key) => Some(ok_prive_key),
//         Err(_e) => None,
//     };
//
//     match priv_key {
//         Some(some_priv_key) => Ok(Some(RsaPublicKey::from(&some_priv_key))),
//         None => Ok(None),
//     }
// }

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
            // <Await
            //     // `future` provides the `Future` to be resolved
            //     future=|| get_public_key()
            //     // the data is bound to whatever variable name you provide
            //     let:public_key
            // >
            //     {match public_key {
            //         Ok(ok_pub_key) => {
            //             match ok_pub_key {
            //                 Some(some_pub_key) => {
            //                     view! { <UsernameLoginLayer pub_key=some_pub_key.clone() /> }
            //                         .into_view()
            //                 }
            //                 None => view! {}.into_view(),
            //             }
            //         }
            //         Err(_) => view! {}.into_view(),
            //     }}
            //
            // </Await>
            <UsernameLoginLayer />
        </div>

        <div class:display=move || show_layer.get()>
            <MobileLoginLayer />
        </div>
    }
}
