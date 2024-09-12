use leptos::*;
use leptos::ev::KeyboardEvent;

#[component]
pub fn TutorialEditorArea() -> impl IntoView {
    let (code, set_code) = create_signal("<span style=\"color:red;\">temp default</span>".to_string());

    let on_keydown = move |ev: KeyboardEvent| {

        if ev.code() == "Tab" {
            // stop the key action!
            ev.prevent_default();
            let space = KeyboardEvent::new("keydown");
            space.set("Space");
            // logging::log!("{:?}", space);
        }
        logging::log!("keycode: {}", ev.code());

        // set_code.set(event_target_value(&ev));
    };

    view! {
        // <textarea
        //     class="editor"
        //     spellcheck="false"
        //     prop:value=move || code.get()
        //     on:keydown=on_keydown
        // >
        // </textarea>
        <div class="editor" contenteditable="true" spellcheck="false" on:keydown=on_keydown>
            <div inner_html={move || code.get()} />
       </div>
    }
}
