pub mod about;
pub mod header;

use leptos::*;

/// Renders the home page of your application.
#[component]
pub fn HomePage() -> impl IntoView {
    use header::*;

    view! {
        <div class="contents">
            <HeaderMenu />
        </div>
        <div>
            <hr class="page_divider" />
        </div>
        <div class="contents">
            <img src="images/logo.png" />
        </div>
    }
}
