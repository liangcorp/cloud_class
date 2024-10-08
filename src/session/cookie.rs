use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use leptos::*;
        use std::fmt;
        use server_fn::ServerFnError;
        use chrono::{Datelike, Timelike, Utc};
        use http::{header, HeaderValue};
        use leptos_axum::ResponseOptions;
        // use leptos::expect_context;

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

        impl fmt::Display for Cookie {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "session_token={};domain={};path={};Max-Age={};Expires={};{};{};SameSite={}",
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

        impl Cookie {
            pub fn to_session_only_string(&self) -> String {
                format!("session_token={};domain={};path={};{};{};SameSite={}",
                    self.session_token,
                    self.domain,
                    self.path,
                    self.secure,
                    self.http_only,
                    self.same_site
                )
            }

            pub fn set_cookie(token: &str, is_session_only: bool) -> Result<(), ServerFnError> {
                // create cookie with custom session token
                let cookie = Cookie {
                    session_token: token.to_string(),
                    .. Default::default()
                };

                // pull ResponseOptions from context
                let response = expect_context::<ResponseOptions>();

                let header_cookie = if is_session_only {
                        HeaderValue::from_str(&cookie.to_string())
                    } else {
                        HeaderValue::from_str(&cookie.to_session_only_string())
                    };

                if let Ok(ok_cookie_content) = header_cookie {
                    response.insert_header(header::SET_COOKIE, ok_cookie_content);
                }

                Ok(())
            }

            pub fn delete_cookie() -> Result<(), ServerFnError> {
                let cookie = Cookie {
                    session_token: "".to_string(),
                    max_age: "0".to_string(),
                    expire_date: "Thu, 01 Jan 1970 00:00:00 GMT".to_string(),
                    .. Default::default()
                };

                if let Ok(ok_cookie_content) = HeaderValue::from_str(&cookie.to_string()) {
                    // pull ResponseOptions from context
                    let response = expect_context::<ResponseOptions>();
                    response.insert_header(header::SET_COOKIE, ok_cookie_content);
                }

                Ok(())
            }
        }
    }
}
