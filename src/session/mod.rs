pub mod cache;
pub mod cookie;

use leptos::*;
use server_fn::ServerFnError;

// Get cookie from HTTP Header
// for some reason it's only returning the first element of the cookie
// maybe it's due to security settings
#[server]
pub async fn extract_session_token() -> Result<Option<String>, ServerFnError> {
    use axum::http::header::{HeaderMap, HeaderValue};
    use leptos_axum::extract;

    let header: HeaderMap<HeaderValue>;

    match extract().await {
        Ok(h) => header = h,
        Err(e) => return Err(ServerFnError::Args(e.to_string())),
    }

    let cookie_header;
    match header.get("cookie") {
        Some(c) => cookie_header = c.to_str().unwrap().to_string(),
        None => return Ok(None),
    };

    match cookie_header.split('=').nth(1) {
        Some(s) => Ok(Some(s.to_string())),
        None => Ok(None),
    }
}

#[server]
pub async fn extract_session_user() -> Result<Option<String>, ServerFnError> {
    // use axum::{extract::Query, http::{Method, header::{HeaderMap, HeaderValue}}};
    use crate::utils::redis::Redis;
    use axum::http::header::{HeaderMap, HeaderValue};
    use leptos_axum::extract;
    use redis::Commands;
    let header: HeaderMap<HeaderValue>;

    match extract().await {
        Ok(h) => header = h,
        Err(e) => return Err(ServerFnError::Args(e.to_string())),
    }

    let cookie_header;
    match header.get("cookie") {
        Some(c) => cookie_header = c.to_str().unwrap().to_string(),
        None => return Ok(None),
    };

    let session_token;
    match cookie_header.split('=').nth(1) {
        Some(s) => session_token = s,
        None => return Ok(None),
    }

    let mut redis_cluster_conn = Redis::get_single_connection()?;

    if let Ok(Some(session_user)) = redis_cluster_conn.get(session_token) {
        Ok(Some(session_user))
    } else {
        Err(ServerFnError::Args(
            "ERROR: user session not found".to_string(),
        ))
    }
}
