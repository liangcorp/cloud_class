#![recursion_limit = "256"]

pub mod admin; // 管理员页
pub mod app;
pub mod courses;
pub mod header;
pub mod home; // 主页
pub mod session;
pub mod state;
pub mod student; // 用户登陆页
pub mod tutorials;
pub mod utils;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}
