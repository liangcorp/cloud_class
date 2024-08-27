use leptos::*;
use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use chrono::{Datelike, Timelike, Utc};

        #[derive(Debug)]
        pub struct CustomCookie {
            // pub id: Uuid,
            // pub username: String,
            pub session_token: String,
            pub domain: String,
            pub path: String,
            pub max_age: String,
            pub expire_date: String,
            pub secure: String,
            pub http_only: String,
            pub same_site: String,
        }

        impl CustomCookie {
            pub fn new() -> CustomCookie {
                let now = Utc::now();

                let month_str = [
                    "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
                ];

                let month = match now.month() {
                    12 => 0,            // first month in the vector
                    _ => now.month(),
                };

                let expire = format!(
                    "{}, {:?} {} {:?} {:02}:{:02}:{:02} UTC",
                    now.weekday(),
                    now.day(),
                    month_str[month as usize],  // list start from 0
                    now.year(),
                    now.hour(),
                    now.minute(),
                    now.second(),
                );

                CustomCookie {
                    session_token: "".to_string(),
                    domain: "".to_string(),
                    path: "/".to_string(),
                    expire_date: expire,
                    max_age: "2592000".to_string(),     // 30 days
                    // max_age: "10".to_string(),       // 10 seconds for testing
                    // secure: "Secure".to_string(),    // only enable for HTTPS
                    secure: "".to_string(),             // for HTTP
                    http_only: "HttpOnly".to_string(),  // stop JavaScript from modifying cookies
                    same_site: "Strict".to_string(),    // Strict, Lax, and None
                }
            }

            pub fn to_string(&self) -> String {
                format!("session_token={};domain={};path={};Max-Age={};Expires={};{};{};SameSite={}",
                    self.session_token,
                    self.domain,
                    self.path,
                    self.max_age,
                    self.expire_date,
                    self.secure,
                    self.http_only,
                    self.same_site
                )
            }
        }
    }
}

// Get cookie from HTTP Header
// for some reason it's only returning the first element of the cookie
// maybe it's due to security settings
#[server]
pub async fn extract_header_cookie() -> Result<String, ServerFnError> {
    // use axum::{extract::Query, http::{Method, header::{HeaderMap, HeaderValue}}};
    use axum::http::header::{HeaderMap, HeaderValue};
    use leptos_axum::extract;

    // let (method, query): (Method, Query<MyQuery>);
    let mut header: HeaderMap<HeaderValue> = HeaderMap::new();

    match extract().await {
        Ok(h) => {
            header = h;
            logging::log!("DEBUG<session/cookie.rs>: extracted cookie: {:?}", header.get("cookie"));
        }
        Err(e) => {
            logging::log!("ERROR<session/cookie.rs>: {}", e.to_string());
        }
    }

    let cookie = match header.get("cookie") {
        Some(c) => c.to_str().unwrap().to_string(),
        None => "".to_string(),
    };

    Ok(cookie)
}
