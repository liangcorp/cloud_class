use leptos::*;
use leptos_meta::Title;

/// Renders the home page of your application.
#[component]
pub fn InstructorListPage() -> impl IntoView {
    use crate::header::HeaderSection;

    view! {
        <Title text="教师中心" />

        <HeaderSection />

        <div class="contents">
            <img src="images/logo.png" />
        </div>
    }
}
