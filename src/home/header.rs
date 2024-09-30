use leptos::*;

/// Renders the header menu of your home page.
#[component]
pub fn HeaderSection() -> impl IntoView {
    view! {
        <div class="contents">
            <table>
                <tr>
                    <td class="header-image">
                        <a href="/" class="header">
                            <img class="header" src="images/logo.png" />
                        </a>
                    </td>
                    <td class="header-menu">
                        <a href="/" class="header">
                            "首页"
                        </a>
                    </td>
                    <td class="header-menu">
                        <a href="/collaboration" class="header">
                            "合作中心"
                        </a>
                    </td>
                    <td class="header-menu">
                        <a href="/instructor" class="header">
                            "教师中心"
                        </a>
                    </td>
                    <td class="header-menu">
                        <a href="/about" class="header">
                            "关于我们"
                        </a>
                    </td>
                    <td class="header-menu"></td>
                    <td class="header-menu"></td>
                    <LoginLogoutSection />
                </tr>
            </table>
        </div>
        <div style="padding-bottom:30px">
            <hr class="page-divider" />
        </div>
    }
}

#[component]
fn LoginLogoutSection() -> impl IntoView {
    use crate::session::*;
    view! {
        <Await
            // `future` provides the `Future` to be resolved
            future=extract_session_user

            // the data is bound to whatever variable name you provide
            let:session_user
        >
            {match session_user {
                Ok(ok_username) => {
                    match ok_username {
                        Some(some_username) => {
                            view! {
                                <td class="header-login">
                                    <a class="header" href="/profile">
                                        {some_username}
                                    </a>
                                </td>
                                <td class="header-login">
                                    <a href="/logout" class="home-login">
                                        "退出"
                                    </a>
                                </td>
                            }
                                .into_view()
                        }
                        None => {
                            view! {
                                <td class="header-login">
                                    <a href="/login" class="home-login">
                                        "登陆"
                                    </a>
                                </td>
                                <td class="header-login">
                                    <a href="/register" class="header">
                                        "注册"
                                    </a>
                                </td>
                            }
                                .into_view()
                        }
                    }
                }
                Err(_) => {
                    view! {
                        <td class="header-login">
                            <a href="/login" class="home-login">
                                "登陆"
                            </a>
                        </td>
                        <td class="header-login">
                            <a href="/register" class="header">
                                "注册"
                            </a>
                        </td>
                    }
                        .into_view()
                }
            }}
        </Await>
    }
}
