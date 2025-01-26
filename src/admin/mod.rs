pub mod control;
mod login;

use leptos::*;
use leptos::prelude::*;
use leptos_router::{ nested_router::Outlet, components::Redirect };

#[component]
pub fn AdminPage() -> impl IntoView {
    view! { <Outlet /> }
}

#[component]
pub fn AdminLoginPage() -> impl IntoView {
    use crate::session::extract_session_user;
    use login::LoginPanel;

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
                        Some(_) => view! { <Redirect path="/admin/control" /> },
                        None => view! { <LoginPanel /> },
                    }
                }
                Err(_) => view! { <LoginPanel /> },
            }}
        </Await>
    }
}

#[component]
pub fn AdminRedirectPage() -> impl IntoView {
    view! { <Redirect path="/admin/control" /> }
}
