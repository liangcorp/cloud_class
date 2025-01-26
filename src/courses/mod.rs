pub mod content;

use leptos::*;
use leptos_router::{ components::Redirect, nested_router::Outlet };

#[component]
pub fn CoursesPage() -> impl IntoView {
    view! { <Outlet /> }
}

#[component]
pub fn NoCoursePage() -> impl IntoView {
    view! { <Redirect path="/profile" /> }
}
