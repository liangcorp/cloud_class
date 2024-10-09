
use leptos::*;

/// Renders the home page of your application.
#[component]
pub fn InstructorListPage() -> impl IntoView {
    use crate::header::HeaderSection;

    view! {
        <HeaderSection />

        <div class="contents">
            <img src="images/logo.png" />
        </div>
    }
}
