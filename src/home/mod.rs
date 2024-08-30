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
            future=extract_session_user

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
                                个人中心
                            </a>
                        </td>
                        <td class="header_menu">
                            <a href="#" class="header">
                                商业中心
                            </a>
                        </td>
                        <td class="header_menu">
                            <a href="#" class="header">
                                高校中心
                            </a>
                        </td>
                        <td class="header_menu">
                            <a href="#" class="header">
                                政府中心
                            </a>
                        </td>
                        <td class="header_menu">
                            <a href="#" class="header">
                                关于我们
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
                            <a class="header" href="/profile">
                                {move || username.get()}
                            </a>
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
