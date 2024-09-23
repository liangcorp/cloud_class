use leptos::*;

#[component]
pub fn TutorialExecutionArea() -> impl IntoView {

    view! {
        <div class="output_area">
            <Transition fallback=move || view! { <p>"正在联系..."</p> }>
                <iframe
                    style="width:100%;height:890px;border:none;"
                    src="http://localhost:8501/"
                ></iframe>
            </Transition>
        </div>
    }
}
