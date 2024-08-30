use leptos::*;
use server_fn::ServerFnError;

#[server(Logout, "/api")]
pub async fn user_logout() -> Result<String, ServerFnError> {
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
        None => "".to_string(),
    };

    let exit_message = cookie.split('=')
        .nth(1)
        .unwrap_or("");

    Cache::delete_cache(exit_message)?;
    Cookie::delete_cookie()?;

    // 改变网址到学生资料
    leptos_axum::redirect("/");

    Ok("正在退出...".to_string())
}

#[component]
pub fn LogoutPage() -> impl IntoView {

    let (exit_message, set_exit_message) = create_signal("正在退出...".to_string());

    view! {
        <Await
            // `future` provides the `Future` to be resolved
            future=user_logout

            // the data is bound to whatever variable name you provide
            let:exit_message
        >
            <p>
                {match exit_message {
                    Ok(s) => set_exit_message.set(s.clone()),
                    Err(_) => set_exit_message.set("".to_string()),
                }}
            </p>
        </Await>

        <p>{move || exit_message.get()}</p>
    }
}
