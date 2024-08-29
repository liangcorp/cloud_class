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

    let redis = Redis::default();
    match redis.fetch_an_integer() {
        Ok(s) => logging::log!("{}", s),
        Err(e) => logging::log!("{}", e.to_string()),
    };

    Ok(cookie)
}

