use leptos::*;
use server_fn::ServerFnError;
use leptos_router::Redirect;

#[server(Logout, "/api")]
pub async fn user_logout() -> Result<Option<()>, ServerFnError> {
    use leptos_axum::extract;
    use axum::http::header::{HeaderMap, HeaderValue};
    use crate::session::{cache::Cache, cookie::Cookie};

    let header: HeaderMap<HeaderValue> = match extract().await {
        Ok(ok_header) => {
            ok_header
        }
        Err(e) => {
            logging::log!("ERROR<session/mod.rs>: {}", e.to_string());
            HeaderMap::new()
        }
    };

    let cookie = match header.get("cookie") {
        Some(some_cookie) => some_cookie.to_str().unwrap().to_string(),
        None => return Err(ServerFnError::Args("cookie not found".to_string())),
    };

    let exit_message = match cookie.split('=').nth(1) {
        Some(some_exit_message) => some_exit_message,
        None => return Err(ServerFnError::Args("malformed cookie".to_string())),
    };

    Cache::delete_cache(exit_message)?;
    Cookie::delete_cookie()?;

    // 改变网址到学生资料
    leptos_axum::redirect("/");

    Ok(Some(()))
}

#[component]
pub fn LogoutPage() -> impl IntoView {
    let logout_state = create_resource(|| (), |_| async move { user_logout().await });

    view! {
        <Transition fallback=move || {
            view! { <p>"正在退出..."</p> }
        }>{move || { logout_state.get().map(|_| view! { <Redirect path="/" /> }) }}</Transition>
    }
}
