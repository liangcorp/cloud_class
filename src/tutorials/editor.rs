use leptos::*;

#[component]
pub fn TutorialEditorArea() -> impl IntoView {
    let (code, set_code) = create_signal("default".to_string());

    let on_keyboard = move |ev: leptos::ev::KeyboardEvent| {
        // stop the page from reloading!
        // ev.prevent_default();

        if ev.key() == "click" || ev.code() == "0x000F" {
            set_code.set(format!("{}    ", event_target_value(&ev)));
        }
        logging::log!("keycode: {}", ev.key_code());
        logging::log!("charcode: {}", ev.char_code());

        // set_code.set(event_target_value(&ev));
    };

    view! {
        <textarea
            class="editor"
            spellcheck="false"
            prop:value=move || code.get()
            on:keyboard=on_keyboard
        >
            { move || code.get() }
        </textarea>
    }
}
