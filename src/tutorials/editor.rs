use leptos::ev::KeyboardEvent;
use leptos::*;
use leptos::prelude::*;
use leptos::task::spawn_local;

#[server]
pub async fn execute_user_code(code: String, username: String) -> Result<(), ServerFnError> {
    use std::fs::File;
    use std::io::prelude::*;
    use std::process::Command;

    let mut file = match File::create(format!("/tmp/python_streamlit_{}.py", username)) {
        Ok(f) => f,
        Err(e) => {
            // logging::log!("ERROR <tutorials/editor.rs:{}>: {}", line!(), e.to_string());
            return Err(ServerFnError::Args(e.to_string()));
        }
    };

    match file.write_all(&code.into_bytes()) {
        Ok(_) => {
            match Command::new("docker")
                .arg("cp")
                .arg(format!("/tmp/python_streamlit_{}.py", username))
                .arg(format!("{}:streamlit_app.py", username))
                .spawn()
            {
                Ok(_) => Ok(()),
                Err(e) => {
                    // logging::log!("ERROR <tutorials/editor.rs:{}>: {}", line!(), e.to_string());
                    Err(ServerFnError::Args(e.to_string()))
                }
            }
        }
        Err(e) => Err(ServerFnError::Args(e.to_string())),
    }
}

#[component]
pub fn TutorialEditorArea(initial_code: ReadSignal<String>, username: String) -> impl IntoView {
    let input_element: NodeRef<html::Textarea> = NodeRef::new();

    let on_keydown = move |ev: KeyboardEvent| {
        if ev.code() == "Tab" {
            // stop the key action
            ev.prevent_default();
        }
    };

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        // stop the page from reloading!
        ev.prevent_default();

        let username_clone = username.clone();
        // here, we'll extract the value from the input
        let value = input_element
            .get()
            // event handlers can only fire after the view
            // is mounted to the DOM, so the `NodeRef` will be `Some`
            .expect("<input> should be mounted")
            // `leptos::HtmlElement<html::Input>` implements `Deref`
            // to a `web_sys::HtmlInputElement`.
            // this means we can call`HtmlInputElement::value()`
            // to get the current value of the input
            .value();

        spawn_local(async move {
            let _ = execute_user_code(value, username_clone).await;
        });
    };

    view! {
        <form on:submit=on_submit>
            <div class="toolbar">
                <input class="run-code" type="submit" value="⯈ 运行" />
            </div>
            <div class="editor-area">
                <div class="text-area">
                    <textarea
                        class="editor"
                        spellcheck="false"
                        prop:value=move || initial_code.get()
                        on:keydown=on_keydown
                        node_ref=input_element
                    ></textarea>
                </div>
            </div>
        </form>
    }
}
