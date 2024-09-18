use leptos::*;

#[component]
pub fn TutorialConsoleArea() -> impl IntoView {
    view! {
        <div class="output_area">
            <pre>
                <code>{move || code.get()}</code>
            </pre>
        </div>
    }
}
