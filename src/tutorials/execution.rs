use leptos::*;

#[server]
pub async fn execute_user_code(code: String) -> Result<(), ServerFnError> {
    use std::fs::File;
    use std::io::prelude::*;
    use std::process::Command;

    let mut file = match File::create("./student_codes/streamlist_student1.py") {
        Ok(f) => f,
        Err(e) => {
            // logging::log!("ERROR <tutorials/execution.rs:9>: {}", e.to_string());
            return Err(ServerFnError::Args(e.to_string()))
        },
    };

    match file.write_all(&code.into_bytes()) {
        Ok(_) => {
            match Command::new("podman")
                .arg("cp")
                .arg("student_codes/streamlist_student1.py")
                .arg("student1:streamlit_app.py")
                .output() {
                    Ok(output) => {
                        Ok(())
                    },
                    Err(e) => {
                        // logging::log!("ERROR <tutorials/execution.rs:26>: {}", e.to_string());
                        return Err(ServerFnError::Args(e.to_string()))
                    }
                }
        },
        Err(e) => return Err(ServerFnError::Args(e.to_string())),
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
            <Transition fallback=move || view! { <p>"Loading..."</p> }>
                <pre>
                    <code>
                        {move || match user_code_execution_result.get() {
                            Some(_) => "".to_string(),
                            None => "".to_string(),
                        }}
                    </code>
                </pre>
                <iframe
                    style="width:100%;height:890px;border:none;"
                    src="http://localhost:8501/"
                ></iframe>
            </Transition>
        </div>
    }
}
