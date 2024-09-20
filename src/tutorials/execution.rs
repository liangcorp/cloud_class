use leptos::*;

pub async fn execute_user_code(code: String) -> Result<String, ServerFnError> {
    Ok(code)
}

#[component]
pub fn TutorialExecutionArea(user_code: ReadSignal<String>) -> impl IntoView {

    // our resource
    let user_code_execution_result = create_resource(
        move || user_code.get(),
        // every time `count` changes, this will run
        move |value| async move {
            // logging::log!("executing user code");
            execute_user_code(value).await
        },
    );

    view! {
        <div class="output_area">
            <Suspense fallback=move || view! { <p>"Loading..."</p> }>
                <pre>
                    <code>
                    {move || match user_code_execution_result.get(){
                        Some(some_code_data) => {
                            match some_code_data {
                                Ok(ok_code_data) => {
                                    ok_code_data
                                }
                                Err(_) => "".to_string(),
                            }
                        }
                        None => "".to_string(),
                    }}
                    </code>
                </pre>
            </Suspense>
        </div>
    }
}
