pub mod cookie;
pub mod cache;

use leptos::*;
use server_fn::ServerFnError;
// Get cookie from HTTP Header
// for some reason it's only returning the first element of the cookie
// maybe it's due to security settings
#[server]
pub async fn extract_session() -> Result<String, ServerFnError> {
    // use axum::{extract::Query, http::{Method, header::{HeaderMap, HeaderValue}}};
    use axum::http::header::{HeaderMap, HeaderValue};
    use leptos_axum::extract;
    use crate::utils::redis::Redis;
    use redis::Commands;
    // let (method, query): (Method, Query<MyQuery>);
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

    let session_token = cookie.split('=').next();

    let mut redis_cluster_conn = Redis::get_cluster_connection().unwrap();

    // let _: () = redis_cluster_conn.set(session_token, "user")?;
    // let _: () = redis_cluster_conn.expire(session_token, 10)?;
    if let Ok(Some(username)) =  redis_cluster_conn.get(session_token) {
        Ok(username)
    } else {
        Ok("".to_string())
    }

    // Ok(cookie)
}

