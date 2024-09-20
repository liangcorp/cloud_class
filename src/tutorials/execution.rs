use leptos::*;

#[component]
pub fn TutorialExecutionArea(user_code: ReadSignal<String>) -> impl IntoView {

    // @TODO actually implement code execution
    view! {
        <div class="output_area">
            <pre>
                <code>{move || user_code.get()}</code>
            </pre>
        </div>
    }
}
