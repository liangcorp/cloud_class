use leptos::prelude::*;
use leptos_router::components::Redirect;
use server_fn::ServerFnError;

#[server(Logout, "/api")]
pub async fn user_logout() -> Result<Option<()>, ServerFnError> {
    use crate::session::{cache::Cache, cookie::Cookie};
    use axum::http::header::{HeaderMap, HeaderValue};
    use leptos_axum::extract;

    let header: HeaderMap<HeaderValue> = match extract().await {
        Ok(ok_header) => ok_header,
        Err(_e) => {
            // logging::log!("ERROR<session/mod.rs>: {}", e.to_string());
            HeaderMap::new()
        }
    };

    let cookie_result = match header.get("cookie") {
        Some(some_cookie) => some_cookie.to_str(),
        None => {
            //  logging::log!("cookie not found");
            return Ok(None);
        }
    };

    let cookie = match cookie_result {
        Ok(c) => c,
        Err(_e) => {
            // logging::log!("invalid cookie");
            return Ok(None);
        }
    };

    let exit_message = match cookie.split('=').nth(1) {
        Some(some_exit_message) => some_exit_message,
        None => {
            // logging::log!("malformed cookie");
            return Ok(None);
        }
    };

    Cache::delete_cache(exit_message)?;
    Cookie::delete_cookie()?;

    // 改变网址到学生资料
    leptos_axum::redirect("/");

    Ok(Some(()))
}

#[component]
pub fn LogoutPage() -> impl IntoView {
    let logout_state = Resource::new(|| (), |_| async move { user_logout().await });

    view! {
        <Transition fallback=move || {
            view! { <p>"正在退出..."</p> }
        }>{move || { logout_state.get().map(|_| view! { <Redirect path="/" /> }) }}</Transition>
    }
}
