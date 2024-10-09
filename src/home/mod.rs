pub mod about;
pub mod collaboration;
pub mod contact;
pub mod instructor_list;

use leptos::*;
use leptos_meta::Title;

/// Renders the home page of your application.
#[component]
pub fn HomePage() -> impl IntoView {
    use crate::header::HeaderSection;

    view! {
        <Title text="首页" />

        <HeaderSection />

        <div class="contents">
            <img src="images/logo.png" />
        </div>
    }
}
