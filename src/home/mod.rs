use leptos::*;

/// Renders the home page of your application.
#[component]
pub fn HomePage() -> impl IntoView {
    use crate::session::*;

    view! {
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
                            <a href="/profile" class="header">
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
                        <td class="header_menu">
                            <input class="course_search_box_home" style="text" placeholder="搜索" />
                        </td>
                        <Await
                            // `future` provides the `Future` to be resolved
                            future=extract_session_user

                            // the data is bound to whatever variable name you provide
                            let:session_user
                        >
                                {match session_user {
                                    Ok(username) => {
                                        view! {
                                            <td class="header_login">
                                                <a class="header" href="/profile">
                                                     {username}
                                                </a>
                                            </td>
                                            <td class="header_login">
                                                <a
                                                    href="/logout"
                                                    class="home_login"
                                                >
                                                    退出
                                                </a>
                                            </td>
                                        }.into_view()
                                    },
                                    Err(_) => {
                                        view!{
                                            <td class="header_login">
                                                <a
                                                    href="/login"
                                                    class="home_login"
                                                >
                                                    登陆
                                                </a>
                                            </td>
                                            <td class="header_login">
                                                <a
                                                    href="/register"
                                                    class="header"
                                                >
                                                    注册
                                                </a>
                                            </td>
                                        }.into_view()
                                    },
                                }}
                        </Await>
                    </tr>
                </table>
            </div>
        </div>
        <div>
            <hr class="page_divider"></hr>
        </div>
    }
}
