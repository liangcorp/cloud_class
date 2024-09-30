mod header;

pub mod about;
pub mod contact;
pub mod collaboration;

use leptos::*;

/// Renders the home page of your application.
#[component]
pub fn HomePage() -> impl IntoView {
    use header::HeaderSection;

    view! {
        <HeaderSection />

        <div class="contents">
            <img src="images/logo.png" />
        </div>
    }
}
