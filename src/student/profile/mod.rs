pub mod class;
pub mod info;

use leptos::*;
use leptos_router::Redirect;
use crate::student::profile::class::ClassPage;
use crate::student::profile::info::PersonalPage;

// use serde::Deserialize;
/// Renders the profile page of your application.
#[component]
pub fn ProfilePage() -> impl IntoView {
    use crate::session::*;

    let (username, set_username) = create_signal("".to_string());

    let (show, set_show) = create_signal(true);

    let async_data = create_resource(move || username.clone(), |_| async move { extract_session_user().await });

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
                            <a
                                href="/profile"
                                class="header"
                                on:click=move |_| {
                                    set_show.set(true);
                                }
                            >
                                我的课程
                            </a>
                        </td>
                        <td class="header_menu">
                            <a
                                href="/profile"
                                class="header"
                                on:click=move |_| {
                                    set_show.set(false);
                                }
                            >
                                个人资料
                            </a>
                        </td>
                        <td class="header_menu">
                            <input
                                class="course_search_box_profile"
                                style="text"
                                placeholder="搜索"
                            />
                        </td>
                        <Await
                            // `future` provides the `Future` to be resolved
                            future=extract_session_user

                            // the data is bound to whatever variable name you provide
                            let:session_user
                        >
                            {match session_user {
                                Ok(uname) => {
                                    view! {
                                        <td class="header_login">
                                            <a class="header" href="/profile">
                                                {
                                                    set_username.set(uname.clone());
                                                    uname
                                                }
                                            </a>
                                        </td>
                                        <td class="header_login">
                                            <a href="/logout" class="home_login">
                                                退出
                                            </a>
                                        </td>
                                    }
                                        .into_view()
                                }
                                Err(_) => {
                                    view! {
                                        <Redirect path="/" />
                                        <td class="header_login">
                                            <a href="/login" class="home_login">
                                                登陆
                                            </a>
                                        </td>
                                        <td class="header_login">
                                            <a href="/register" class="header">
                                                注册
                                            </a>
                                        </td>
                                    }
                                        .into_view()
                                }
                            }}
                        </Await>
                    </tr>
                </table>
            </div>
        </div>
        <div>
            <hr class="page_divider" />
        </div>
        <Transition fallback=move || view! { <h1>"正在运行..."</h1> }>
            <div class:display=move || show.get() == false>
                <ClassPage user=async_data.get().unwrap_or(Ok("".to_string())).unwrap() />
            </div>
            <div class:display=move || show.get() == true>
                <PersonalPage user=async_data.get().unwrap_or(Ok("".to_string())).unwrap() />
            </div>
        </Transition>
    }
}
