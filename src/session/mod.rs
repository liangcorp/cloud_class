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
        }
        Err(e) => {
            logging::log!("ERROR<session/mod.rs>: {}", e.to_string());
        }
    }

    let cookie = match header.get("cookie") {
        Some(c) => c.to_str().unwrap().to_string(),
        None => return Err(ServerFnError::Args("INFO: cookie not found".to_string())),
    };

    match cookie.split('=').nth(1) {
        Some(s) => Ok(s.to_string()),
        None => return Err(ServerFnError::Args("malformed cookie".to_string())),
    }
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
        }
        Err(e) => {
            logging::log!("ERROR<session/mod.rs>: {}", e.to_string());
            return Err(ServerFnError::Args(e.to_string()));
        }
    }

    let cookie = match header.get("cookie") {
        Some(c) => c.to_str().unwrap().to_string(),
        None => return Err(ServerFnError::Args("INFO: empty cookie".to_string())),
    };

    let session_token;
    match cookie.split('=').nth(1) {
        Some(s) => session_token = s,
        None => return Err(ServerFnError::Args("ERROR: malformed cookie".to_string())),
    }

    let mut redis_cluster_conn = Redis::get_cluster_connection()?;

    logging::log!("DEBUG<session/mod.rs:extract_session_user()>: {:?}", session_token);

    if let Ok(Some(session_user)) =  redis_cluster_conn.get(session_token) {
        Ok(session_user)
    } else {
        Err(ServerFnError::Args("INFO: cache not found".to_string()))
    }
}
