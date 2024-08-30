use leptos::*;
// use server_fn::ServerFnError;
// use serde::Deserialize;
/// Renders the home page of your application.
#[component]
pub fn HomePage() -> impl IntoView {
    use crate::session::*;

    let (username, set_username) = create_signal("".to_string());
    let (login_button, set_login_button) = create_signal("inline".to_string());
    let (logout_button, set_logout_button) = create_signal("none".to_string());

    view! {
        <Await
            // `future` provides the `Future` to be resolved
            future=extract_session

            // the data is bound to whatever variable name you provide
            let:session_user
        >
            <p>
                {match session_user {
                    Ok(s) => {
                        if s == "" {
                            set_login_button.set("inline".to_string());
                            set_logout_button.set("none".to_string());
                        } else {
                            set_login_button.set("none".to_string());
                            set_logout_button.set("inline".to_string());
                        }
                        set_username.set((*s).clone());
                    }
                    Err(_) => {
                        set_username.set("".to_string());
                    }
                }}
            </p>
        </Await>

        <div class="contents">
            <div class="header">
                <table class="header-menu">
                    <tr>
                        <td class="header">
                            <img src="images/logo.png" />
                        </td>
                        <td class="header_menu">
                            <a href="/" class="header">
                                首页
                            </a>
                        </td>
                        <td class="header_menu">
                            <a href="#" class="header">
                                走进学校
                            </a>
                        </td>
                        <td class="header_menu">
                            <a href="#" class="header">
                                课程中心
                            </a>
                        </td>
                        <td class="header_menu">
                            <a href="#" class="header">
                                继续教育
                            </a>
                        </td>
                        <td class="header_menu">
                            <a href="#" class="header">
                                师资力量
                            </a>
                        </td>
                        <td class="header_menu">
                            <a href="#" class="header">
                                新闻中心
                            </a>
                        </td>
                        <td class="header_menu">
                            <a href="#" class="header">
                                在线学习
                            </a>
                        </td>
                        <td class="header_menu">
                            <a href="#" class="header">
                                就业招聘
                            </a>
                        </td>

                        <td class="header_login">
                            <a
                                href="/login"
                                class="home_login"
                                style:display=move || login_button.get()
                            >
                                登陆
                            </a>
                            <a class="header" href="/profile">{move || username.get()}</a>
                        </td>
                        <td class="header_login">
                            <a
                                href="/register"
                                class="header"
                                style:display=move || login_button.get()
                            >
                                注册
                            </a>
                            <a
                                href="/logout"
                                class="home_login"
                                style:display=move || logout_button.get()
                            >
                                退出
                            </a>
                        </td>
                    </tr>
                </table>
            </div>
        </div>
        <div>
            <img src="images/banners/3.财务会计banner.jpg" class="banner" />
        </div>
    }
}
