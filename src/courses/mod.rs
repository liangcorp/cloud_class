pub mod content;

use leptos::*;
use leptos_router::*;

#[component]
pub fn CoursesPage() -> impl IntoView {
    view! {
        <Outlet/>
        // <Redirect path="/profile" />
    }
}

#[component]
pub fn NoCoursePage() -> impl IntoView {
    view! {
        <Redirect path="/profile" />
    }
}

