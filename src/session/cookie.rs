use leptos::*;
use cfg_if::cfg_if;
use server_fn::ServerFnError;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use chrono::{Datelike, Timelike, Utc};
        use http::{header, HeaderValue};
        // use leptos::expect_context;
        use leptos_axum::ResponseOptions;

        #[derive(Debug)]
        pub struct Cookie {
            // pub id: Uuid,
            // pub username: String,
            session_token: String,
            domain: String,
            path: String,
            max_age: String,
            expire_date: String,
            secure: String,
            http_only: String,
            same_site: String,
        }

        impl Default for Cookie {
            fn default() -> Cookie {
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

                Cookie {
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
        }

        impl Cookie {
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

            pub fn insert_cookie_to_header(&mut self, token: &str) -> Result<(), ServerFnError> {
                // set session token in cookie
                self.session_token = token.to_string();

                if let Ok(cookie) = HeaderValue::from_str(&self.to_string()) {
                    // pull ResponseOptions from context
                    let response = expect_context::<ResponseOptions>();
                    response.insert_header(header::SET_COOKIE, cookie);
                }

                Ok(())
            }
        }
    }
}

