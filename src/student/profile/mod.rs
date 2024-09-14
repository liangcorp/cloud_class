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
                            set_username.set(some_username.clone());
                            view! {
                                <div class="contents">
                                    <table>
                                        <tr>
                                            <td class="header_image">
                                                <a href="/" class="header">
                                                    <img class="header" src="images/logo.png" />
                                                </a>
                                            </td>
                                            <td class="header_menu">
                                                <a class="header" href="/">
                                                    "首页"
                                                </a>
                                            </td>
                                            <td class="header_menu">
                                                <a
                                                    href="/profile"
                                                    class="header"
                                                    on:click=move |_| {
                                                        set_show_layer.set(true);
                                                    }
                                                >
                                                    "我的课程"
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
                                                    "个人资料"
                                                </a>
                                            </td>
                                            <td class="header_menu"></td>
                                            <td class="header_menu"></td>
                                            <td class="header_menu"></td>
                                            <td class="header_login">
                                                <a class="header" href="/profile">
                                                    {some_username}
                                                </a>
                                            </td>
                                            <td class="header_login">
                                                <a href="/logout" class="home_login">
                                                    "退出"
                                                </a>
                                            </td>
                                        </tr>
                                    </table>
                                </div>
                                <div>
                                    <hr class="page_divider" />
                                </div>
                                <Transition fallback=move || view! { <h1>"正在运行..."</h1> }>
                                    <div class:display=move || show_layer.get() == false>
                                        <CourseContentPage user=username.get() />
                                    </div>
                                    <div class:display=move || show_layer.get() == true>
                                        <PersonalContentPage user=username.get() />
                                    </div>
                                </Transition>
                            }
                                .into_view()
                        }
                        None => view! { <Redirect path="/" /> }.into_view(),
                    }
                }
                Err(_) => view! { <Redirect path="/" /> }.into_view(),
            }}
        </Await>
    }
}
