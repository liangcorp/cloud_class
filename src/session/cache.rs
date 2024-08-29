use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use leptos::*;
        use server_fn::ServerFnError;
        use chrono::{Datelike, Timelike, Utc};
        use redis::Commands;
        use redis::cluster::ClusterConnection;

        #[derive(Debug)]
        pub struct CustomCache {
            username: String,
            session_token: String,
        }

        impl Default for CustomCache {
            fn default() -> CustomCache {
                CustomCache {
                    username: "".to_string(),
                    session_token: "".to_string(),
                }
            }
        }

        impl CustomCache {
            pub fn to_string(&self) -> String {
                format!("CustomCache: ( {} {} {} )", self.username, self.session_token, self.date_created)
            }

            pub fn set_cache(session_token: String, username: String) -> Result<(), ServerFnError> {
                let mut cache = CustomCache::default();
                cache.username = username;
                cache.session_token = session_token;

                let redis = Redis::default();
                let mut redis_cluster_conn = redis.get_cluster_connection().unwrap();

                let _: () = redis_cluster_conn.set(session_token, username)?;
                let _: () = redis_cluster_conn.expire(session_token, 10)?;

                Ok(())
            }
        }
    }
}
