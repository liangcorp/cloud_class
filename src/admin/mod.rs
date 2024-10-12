pub mod control;
mod login;

use leptos::*;
use leptos_router::*;

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
                        Some(_) => view! { <Redirect path="/admin/control" /> }.into_view(),
                        None => {
                            view! {
                                <LoginPanel />
                            }
                                .into_view()
                        }
                    }
                }
                Err(_) => {
                    view! {
                        <LoginPanel />
                    }
                        .into_view()
                }
            }}
        </Await>
    }
}

#[component]
pub fn AdminRedirectPage() -> impl IntoView {
    view! { <Redirect path="/admin/control" /> }
}
