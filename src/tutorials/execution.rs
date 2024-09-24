use leptos::*;

#[component]
pub fn TutorialExecutionArea() -> impl IntoView {
    view! {
        <div class="output-area">
            <iframe class="code-execution" src="http://localhost:8501/" />
        </div>
    }
}
