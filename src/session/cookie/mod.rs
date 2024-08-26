
use leptos::*;
use uuid::Uuid;
use chrono::{Datelike, Timelike, Utc};

// #[cfg(feature = "ssr", derive(serde::Deserialize))]
#[derive(Debug)]
pub struct CustomCookie {
    pub id: Uuid,
    pub username: String,
    pub domain: String,
    pub path: String,
    pub session_token: String,
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
            id: Uuid::new_v4(),
            username: "".to_string(),
            session_token: "".to_string(),
            domain: "".to_string(),
            path: "/".to_string(),
            expire_date: expire,
            max_age: "2592000".to_string(),     // 30 days
            // secure: "Secure".to_string(),    // only enable for HTTPS
            secure: "".to_string(),             // for HTTP
            http_only: "HttpOnly".to_string(),  // stop JavaScript from modifying cookies
            same_site: "Strict".to_string(),    // Strict, Lax, and None
        }
    }

    pub fn to_string(&self) -> String {
        format!("id={};session_token={};username={};domain={};path={};Max-Age={};Expires={};{};{};SameSite={}",
            self.id,
            self.session_token,
            self.username,
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

// #[server]
// pub async fn create_header_cookie(id: Uuid, username: String, session_hash: String, expire: String) -> Result<(), ServerFnError> {
//     todo!();
// }

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
