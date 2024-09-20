use leptos::*;

#[server]
pub async fn execute_user_code(code: String) -> Result<(), ServerFnError> {
    use std::fs::File;
    use std::io::prelude::*;

    let mut file = match File::create("streamlist_student.py") {
        Ok(f) => f,
        Err(e) => {
            logging::log!("ERROR <tutorials/execution.rs:9>: {}", e.to_string());
            return Err(ServerFnError::Args(e.to_string()))
        },
    };

    match file.write_all(&code.into_bytes()) {
        Ok(_) => Ok(()),
        Err(e) => Err(ServerFnError::Args(e.to_string())),
    }
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
                        Some(_) => "".to_string(),
                        None => "".to_string(),
                    }}
                    </code>
                </pre>
                <iframe style="width:100%;height:890px;border:none;" src="http://localhost:8501/"></iframe>
            </Suspense>
        </div>
    }
}
