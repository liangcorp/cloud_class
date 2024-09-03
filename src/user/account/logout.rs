use leptos::*;
use server_fn::ServerFnError;
use leptos_router::Redirect;

#[server(Logout, "/api")]
pub async fn user_logout() -> Result<Option<()>, ServerFnError> {
    use leptos_axum::extract;
    use axum::http::header::{HeaderMap, HeaderValue};
    use crate::session::{cache::Cache, cookie::Cookie};

    let mut header: HeaderMap<HeaderValue> = HeaderMap::new();

    match extract().await {
        Ok(h) => {
            header = h;
        }
        Err(e) => {
            logging::log!("ERROR<session/mod.rs>: {}", e.to_string());
        }
    }

    let cookie = match header.get("cookie") {
        Some(c) => c.to_str().unwrap().to_string(),
        None => return Err(ServerFnError::Args("cookie not found".to_string())),
    };

    let exit_message;
    match cookie.split('=').nth(1) {
        Some(e) => exit_message = e,
        None => return Err(ServerFnError::Args("malformed cookie".to_string())),
    }

    Cache::delete_cache(exit_message)?;
    Cookie::delete_cookie()?;

    // 改变网址到学生资料
    leptos_axum::redirect("/");

    Ok(Some(()))
}

#[component]
pub fn LogoutPage() -> impl IntoView {
    let a = create_resource(|| (), |_| async move { user_logout().await });

    view! {
        <Transition
            fallback=move || view! { <p>"正在退出..."</p> }
        >
            {move || {
                a.get()
                    .map(|_| view! { <Redirect path="/" /> })
            }}
        </Transition>
    }
}
