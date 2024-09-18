use leptos::*;

#[component]
pub fn TutorialOutputArea(code: ReadSignal<String>) -> impl IntoView {

    view! {
        <div class="output_area">
            <pre>
                <code>{move || code.get()}</code>
            </pre>
        </div>
    }
}
