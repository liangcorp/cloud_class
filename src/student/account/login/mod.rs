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
    let (show, set_show) = create_signal(true);

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
                                                    set_show.update(|n| *n = true);
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
                                                    set_show.update(|n| *n = false);
                                                }
                                            >
                                                短信登录
                                            </a>
                                        </td>
                                    </tr>
                                </table>
                            </div>

                            <div class:display=move || show.get() == false >
                                <UsernameLoginLayer />
                            </div>

                            <div class:display=move || show.get() == true >
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
