use leptos::*;
use leptos::ev::KeyboardEvent;
// use web_sys::KeyboardEvent;

#[component]
pub fn TutorialEditorArea() -> impl IntoView {
    let (code, set_code) = create_signal("".to_string());

    let on_keydown = move |ev: KeyboardEvent| {

        if ev.code() == "Tab" {
            // stop the key action!
            ev.prevent_default();
        }
    };

    view! {
        <textarea
            class="editor"
            spellcheck="false"
            prop:value=move || code.get()
            on:keydown=on_keydown
        >
        </textarea>
       //  <div class="text_area">
       //      1<br/>2
       //      <div class="editor" contenteditable="true" spellcheck="false" on:keydown=on_keydown>
       //          <div inner_html={move || code.get()} />
       //     </div>
       // </div>
    }
}
