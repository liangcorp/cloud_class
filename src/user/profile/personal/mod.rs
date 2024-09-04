use leptos::*;

#[component]
pub fn PersonalPage(user: String) -> impl IntoView {
    view! { <h1>{user}Personal:</h1> }
}
