mod header;

pub mod about;
pub mod collaboration;
pub mod contact;

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
