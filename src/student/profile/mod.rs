pub mod class;
pub mod info;

use leptos::*;
use leptos_router::Redirect;
use crate::student::profile::class::CourseContentPage;
use crate::student::profile::info::PersonalContentPage;

// use serde::Deserialize;
/// Renders the profile page of your application.
#[component]
pub fn ProfilePage() -> impl IntoView {
    use crate::session::*;

    let (username, set_username) = create_signal("".to_string());

    let (show_layer, set_show_layer) = create_signal(true);

    let async_data = create_resource(move || username, |_| async move { extract_session_user().await });

    view! {
        <div class="contents">
            <table>
                <tr>
                    <td class="header_image">
                        <a href="/" class="header">
                            <img class="header" src="images/logo.png" />
                        </a>
                    </td>
                    // <td class="header_menu">
                    // 首页
                    // </td>
                    <td class="header_menu">
                        <a
                            href="/profile"
                            class="header"
                            on:click=move |_| {
                                set_show_layer.set(true);
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
                                set_show_layer.set(false);
                            }
                        >
                            个人资料
                        </a>
                    </td>
                    <td class="header_menu">
                    </td>
                    <td class="header_menu">
                    </td>
                    <td class="header_menu">
                    </td>
                    <td class="header_menu">
                    </td>
                    <Await
                        // `future` provides the `Future` to be resolved
                        future=extract_session_user

                        // the data is bound to whatever variable name you provide
                        let:session_user
                    >
                        {match session_user {
                            Ok(uname) => {
                                match uname {
                                    Some(u) => {
                                        view! {
                                            <td class="header_login">
                                                <a class="header" href="/profile">
                                                    {
                                                        set_username.set(u.clone());
                                                        u
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
                                    None => {
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
                                }
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
        <div>
            <hr class="page_divider" />
        </div>
        <Transition fallback=move || view! { <h1>"正在运行..."</h1> }>
            <div class:display=move || show_layer.get() == false>
                <CourseContentPage user=async_data.get().unwrap_or(Ok(None)).expect("REASON") />
            </div>
            <div class:display=move || show_layer.get() == true>
                <PersonalContentPage user=async_data.get().unwrap_or(Ok(None)).expect("REASON") />
            </div>
        </Transition>
    }
}
