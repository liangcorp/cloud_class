use leptos::*;

#[component]
pub fn TutorialExecutionArea() -> impl IntoView {
    view! {
        <div class="output_area">
            <iframe class="code_execution" src="http://localhost:8501/" />
        </div>
    }
}
