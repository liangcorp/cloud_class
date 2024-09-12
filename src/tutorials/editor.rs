use leptos::*;

#[component]
pub fn TutorialEditorArea() -> impl IntoView {

    view! { <textarea class="editor" spellcheck="false"></textarea> }
}
