use leptos::*;

#[component]
pub fn TutorialOutputArea(code_exe_result: ReadSignal<String>) -> impl IntoView {

    // @TODO actually implement code execution
    view! {
        <div class="output_area">
            <pre>
                <code>{move || code_exe_result.get()}</code>
            </pre>
        </div>
    }
}
