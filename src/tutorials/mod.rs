// pub mod editor;
pub mod output;
// pub mod console;

use leptos::*;


// use editor::TutorialEditorArea;
// use output::TutorialOutputArea;
// use console::TutorialConsoleArea;

#[component]
pub fn TutorialPage() -> impl IntoView {
    use leptos_router::Redirect;
    use leptos::ev::KeyboardEvent;
    use crate::session::extract_session_user;

    let (code, set_code) = create_signal("".to_string());

    let input_element: NodeRef<html::Textarea> = create_node_ref();

    let on_keydown = move |ev: KeyboardEvent| {

        if ev.code() == "Tab" {
            // stop the key action!
            ev.prevent_default();
        }
    };

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        // stop the page from reloading!
        ev.prevent_default();

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
        set_code.set(value);
    };

    view! {
        <Await
            // `future` provides the `Future` to be resolved
            future=extract_session_user

            // the data is bound to whatever variable name you provide
            let:session_user
        >
            {match session_user {
                Ok(uname) => {
                    match uname {
                        Some(_u) => {
                            view! {
                                <div class="tutorial">
                                    <form on:submit=on_submit>
                                        <div class="toolbar">
                                            <input class="run_code" type="submit" value="⯈ 运行" />
                                        </div>
                                        <div class="editor_area">
                                            <div class="text_area">
                                                <textarea
                                                    class="editor"
                                                    spellcheck="false"
                                                    prop:value=move || code.get()
                                                    on:keydown=on_keydown
                                                    node_ref=input_element
                                                ></textarea>
                                            </div>
                                        </div>
                                    </form>
                                    <div class="output_area">
                                        <pre>
                                            <code>{move || code.get()}</code>
                                        </pre>
                                    </div>
                                </div>
                            }
                                .into_view()
                        }
                        None => view! { <Redirect path="/courses" /> }.into_view(),
                    }
                }
                Err(_) => view! { <Redirect path="/courses" /> }.into_view(),
            }}
        </Await>
    }
}
