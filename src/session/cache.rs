use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use leptos::*;
        use server_fn::ServerFnError;
        use chrono::{Datelike, Timelike, Utc};

        #[derive(Debug)]
        pub struct CustomCache {
            username: String,
            session_token: String,
            date_created: String
        }

        impl Default for CustomCache {
            fn default() -> CustomCache {
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
                    username: "".to_string(),
                    session_token: "".to_string(),
                    date_created
                }
            }
        }

        impl CustomCache {
            pub fn to_string(&self) -> String {
                format!("CustomCache: ( {} {} {} )", self.username, self.session_token, self.date_created)
            }

            pub fn set_cache(session_token: String) -> Result<(), ServerFnError> {
                let mut cache = CustomCache::default();
                cache.session_token = session_token;

                Ok(())
            }
        }
    }
}
