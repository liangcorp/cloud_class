mod header;
pub mod instructors;

use leptos::prelude::*;
use leptos_router::{components::Redirect, nested_router::Outlet};

#[component]
pub fn ControlPanel() -> impl IntoView {
    view! { <Outlet /> }
}

#[component]
pub fn ControlPanelPortal() -> impl IntoView {
    use crate::session::extract_session_user;
    use header::HeaderSection;

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
                                <HeaderSection username=some_username.to_string() />
                                <ControlPanelLandingPage />
                            }

                        }
                        None => view! { <Redirect path="/admin/login" /> },
                    }
                }
                Err(_) => view! { <Redirect path="/admin/login" /> },
            }}
        </Await>
    }
}

#[component]
fn ControlPanelLandingPage() -> impl IntoView {
    view! {
        <div class="contents">
            <img src="images/banners/admin_home.png" />
        </div>
    }
}
