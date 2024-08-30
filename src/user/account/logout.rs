use leptos::*;
use server_fn::ServerFnError;

#[server(Logout, "/api")]
pub async fn user_logout() -> Result<String, ServerFnError> {
    use leptos_axum::extract;
    use axum::http::header::{HeaderMap, HeaderValue};
    use crate::session::cache::CustomCache;

    let mut header: HeaderMap<HeaderValue> = HeaderMap::new();

    match extract().await {
        Ok(h) => {
            header = h;
            logging::log!("DEBUG<session/mod.rs>: extracted cookie: {:?}", header.get("cookie"));
        }
        Err(e) => {
            logging::log!("ERROR<session/mod.rs>: {}", e.to_string());
        }
    }

    let cookie = match header.get("cookie") {
        Some(c) => c.to_str().unwrap().to_string(),
        None => "".to_string(),
    };

    let session_token = cookie.split('=')
        .nth(1)
        .unwrap_or("");

    CustomCache::delete_cache(session_token)?;

    // 改变网址到学生资料
    leptos_axum::redirect("/");

    Ok("正在退出...".to_string())
}

#[component]
pub fn LogoutPage() -> impl IntoView {

    let (session_token, set_session_token) = create_signal("".to_string());

    view!{
        <Await
            // `future` provides the `Future` to be resolved
            future=user_logout

            // the data is bound to whatever variable name you provide
            let:session_token
        >
            <p>
            {
                match session_token {
                    Ok(s) => set_session_token.set(s.clone()),
                    Err(_) => set_session_token.set("".to_string()),
                }
            }
            </p>
        </Await>

        <p>{move || session_token.get()}</p>
    }
}
