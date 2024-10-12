use leptos::*;
use leptos_router::*;

/// Renders the control panel for manaing instructors
#[component]
pub fn AdminInstructorPage() -> impl IntoView {
    use crate::session::extract_session_user;
    use super::header::HeaderSection;

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
                                <HeaderSection username=some_username.to_string()/>
                                "instructor admin page"
                            }
                                .into_view()
                        }
                        None => view! { <Redirect path="/admin/login" /> }.into_view(),
                    }
                }
                Err(_) => view! { <Redirect path="/admin/login" /> }.into_view(),
            }}
        </Await>
    }
}
