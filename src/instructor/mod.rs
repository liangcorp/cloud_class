use leptos::*;
use leptos_meta::*;

#[component]
pub fn InstructorPage() -> impl IntoView {
    use crate::header::HeaderSection;

    view! {
        <Title text="教师中心" />

        <HeaderSection />

        <div class="contents">
        </div>
    }
}
