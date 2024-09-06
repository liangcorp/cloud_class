pub mod content;

use leptos::*;
use leptos_router::*;

#[component]
pub fn CoursesPage() -> impl IntoView {
    view! { <Redirect path="/profile" /> }
}

