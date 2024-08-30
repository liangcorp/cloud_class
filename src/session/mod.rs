pub mod cookie;
pub mod cache;

use leptos::*;
use server_fn::ServerFnError;

// Get cookie from HTTP Header
// for some reason it's only returning the first element of the cookie
// maybe it's due to security settings
#[server]
pub async fn extract_session_token() -> Result<String, ServerFnError> {
    use axum::http::header::{HeaderMap, HeaderValue};
    use leptos_axum::extract;

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

    Ok(cookie.split('=')
        .nth(1)
        .unwrap_or("")
        .to_string())
}

#[server]
pub async fn extract_session_user() -> Result<String, ServerFnError> {
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

    let session_token = cookie
        .split('=')
        .nth(1)
        .unwrap_or("");

    let mut redis_cluster_conn = Redis::get_cluster_connection()?;

    logging::log!("DEBUG<session/mod.rs>: session token: {:?}", session_token);

    if let Ok(Some(session_user)) =  redis_cluster_conn.get(session_token) {
        Ok(session_user)
    } else {
        Ok("".to_string())
    }
}

