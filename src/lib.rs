pub mod app;
pub mod error_template;
pub mod home;   // 主页
pub mod admin;  // 管理员页
pub mod student;  // 用户登陆页
pub mod session;

#[cfg(feature = "ssr")]
pub mod fileserv;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    console_error_panic_hook::set_once();
    leptos::mount_to_body(App);
}
