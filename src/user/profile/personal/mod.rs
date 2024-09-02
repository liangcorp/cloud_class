use leptos::*;

#[component]
pub fn PersonalPage(username: ReadSignal<String>) -> impl IntoView {

    view!{
        <h1> { move || username.get() } Personal: </h1>
    }
}
