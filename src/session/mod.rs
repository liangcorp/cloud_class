use leptos::*;

// #[cfg(feature = "ssr", derive(serde::Deserialize))]
// #[derive(Deserialize, Debug)]
// struct MyQuery {
//     response: ResponseOptions,
// }

#[server]
pub async fn extract_cookie() -> Result<String, ServerFnError> {
    // use axum::{extract::Query, http::{Method, header::{HeaderMap, HeaderValue}}};
    use axum::http::header::{HeaderMap, HeaderValue};
    use leptos_axum::extract;

    // let (method, query): (Method, Query<MyQuery>);
    let mut header: HeaderMap<HeaderValue> = HeaderMap::new();

    match extract().await {
        Ok(h) => {
            header = h;
            logging::log!("{:?}", header.get("cookie"));
        }
        Err(e) => {
            logging::log!("Error: {}", e.to_string());
        }
    }

    let cookie = match header.get("cookie") {
        Some(c) => c.to_str().unwrap().to_string(),
        None => "".to_string(),
    };

    logging::log!("layer loading is working");
    Ok(cookie)
}
