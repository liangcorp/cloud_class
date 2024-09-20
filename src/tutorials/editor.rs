use leptos::*;
use leptos::ev::KeyboardEvent;

#[component]
pub fn TutorialEditorArea(initial_code: ReadSignal<String>, set_user_code: WriteSignal<String>) -> impl IntoView {
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
        set_user_code.set(value);
    };

    view! {
        <form on:submit=on_submit>
            <div class="toolbar">
                <input class="run_code" type="submit" value="⯈ 运行" />
            </div>
            <div class="editor_area">
                <div class="text_area">
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
