// use leptos::*;
use cfg_if::cfg_if;
// use server_fn::ServerFnError;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        // use crate::state::AppState;
        use chrono::{Datelike, Timelike, Utc};

        #[derive(Debug)]
        pub struct CustomCache {
            username: String,
            session_token: String,
            date_created: String,
        }

        impl CustomCache {
            pub fn new(username: String, session_token: String) -> CustomCache {
                let now = Utc::now();

                let date_created = format!(
                    "{}, {:?} {} {:?} {:02}:{:02}:{:02} UTC",
                    now.weekday(),
                    now.day(),
                    now.month(),  // list start from 0
                    now.year(),
                    now.hour(),
                    now.minute(),
                    now.second(),
                );

                CustomCache {
                    username,
                    session_token,
                    date_created,
                }
            }
        }
    }
}
