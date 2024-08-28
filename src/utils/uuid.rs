// use leptos::*;
use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use uuid::Uuid;

        pub fn get_session_id() -> String {
            Uuid::new_v4().to_string()
        }
    }
}
