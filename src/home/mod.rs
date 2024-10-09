pub mod about;
pub mod collaboration;
pub mod contact;
pub mod instructor_list;

use leptos::*;

/// Renders the home page of your application.
#[component]
pub fn HomePage() -> impl IntoView {
    use crate::header::HeaderSection;

    view! {
        <HeaderSection />

        <div class="contents">
            <img src="images/logo.png" />
        </div>
    }
}
