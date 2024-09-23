use leptos::*;

#[component]
pub fn TutorialExecutionArea() -> impl IntoView {
    view! {
        <div class="output_area">
            <Transition fallback=move || view! { <p>"正在联系..."</p> }>
                <iframe class="code_execution" src="http://localhost:8501/" />
            </Transition>
        </div>
    }
}
