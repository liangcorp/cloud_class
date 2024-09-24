pub mod class;
pub mod info;

use leptos::*;
use leptos_router::Redirect;

/// Renders the profile page of your application.
#[component]
pub fn ProfilePage() -> impl IntoView {
    use crate::session::*;

    view! {
        <Await
            // `future` provides the `Future` to be resolved
            future=extract_session_user

            // the data is bound to whatever variable name you provide
            let:session_user
        >
            {match session_user {
                Ok(ok_u) => {
                    match ok_u {
                        Some(some_u) => view! { <ProfilePageContent username=some_u.to_string() /> },
                        None => view! { <Redirect path="/" /> },
                    }
                }
                Err(_) => view! { <Redirect path="/" /> },
            }}
        </Await>
    }
}

#[component]
fn ProfilePageContent(username: String) -> impl IntoView {
    use class::CourseContentPage;
    use info::PersonalContentPage;

    let (show_layer, set_show_layer) = create_signal(true);

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
                    <td class="header-menu">
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
                    <td class="header-menu"></td>
                    <td class="header-menu"></td>
                    <td class="header-menu"></td>

                    <td class="header-login">
                        <a class="header" href="/profile">
                            {username.clone()}
                        </a>
                    </td>
                    <td class="header-login">
                        <a href="/logout" class="home-login">
                            "退出"
                        </a>
                    </td>
                </tr>
            </table>
        </div>
        <div>
            <hr class="page-divider" />
        </div>
        <Transition fallback=move || view! { <h1>"正在运行..."</h1> }>
            <div class="contents" class:display=move || !show_layer.get()>
                <CourseContentPage user=username.clone() />
            </div>
            <div class="profile-contents" class:display=move || show_layer.get()>
                <PersonalContentPage user=username.clone() />
            </div>
        </Transition>
    }
}
