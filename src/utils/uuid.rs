use leptos::*;
use cfg_if::cfg_if;
use server_fn::ServerFnError;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use uuid::Uuid;

        pub fn get_session_id() -> String {
            Uuid::new_v4().to_string()
        }
    }
}
