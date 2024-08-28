use leptos::*;
use cfg_if::cfg_if;
use server_fn::ServerFnError;

cfg_if! {
    if #[cfg(feature = "ssr")] {

    }
}
