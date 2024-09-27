pub mod class;
pub mod info;

use leptos::*;
use leptos_router::Redirect;

/// Renders the profile page of your application.
#[component]
pub fn ProfilePageGate() -> impl IntoView {
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
                        Some(some_u) => view! { <ProfilePage username=some_u.to_string() /> },
                        None => view! { <Redirect path="/" /> },
                    }
                }
                Err(_) => view! { <Redirect path="/" /> },
            }}
        </Await>
    }
}

#[component]
fn ProfilePage(username: String) -> impl IntoView {
    let (show_layer, set_show_layer) = create_signal(true);

    provide_context(show_layer);
    provide_context(set_show_layer);

    view! {
        <ProfilePageHeader username=username.clone() />
        <ProfilePageBody username=username.clone() />
    }
}

#[component]
fn ProfilePageBody(username: String) -> impl IntoView {
    use class::CourseContentPage;
    use info::PersonalContentPage;

    let show_layer = expect_context::<ReadSignal<bool>>();

    view! {
        <div class="contents" class:display=move || !show_layer.get()>
            <CourseContentPage user=username.clone() />
        </div>
        <div class="profile-contents" class:display=move || show_layer.get()>
            <PersonalContentPage user=username.clone() />
        </div>
    }
}

#[component]
fn ProfilePageHeader(username: String) -> impl IntoView {
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
                    <ProfilePageHeaderSelectContent />

                    <td class="header-menu"></td>
                    <td class="header-menu"></td>
                    <td class="header-menu"></td>

                    <td class="header-login">
                        <a class="header" href="/profile">
                            {username}
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
    }
}

#[component]
fn ProfilePageHeaderSelectContent() -> impl IntoView {
    let set_show_layer = expect_context::<WriteSignal<bool>>();

    view! {
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
    }
}
