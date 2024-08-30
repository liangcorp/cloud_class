use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use leptos::*;
        use server_fn::ServerFnError;
        use crate::utils::redis::Redis;
        use redis::Commands;

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
                format!("CustomCache: ( {} {} )", self.username, self.session_token)
            }

            pub fn set_cache(session_token: String, username: String) -> Result<(), ServerFnError> {
                let mut cache = CustomCache::default();
                cache.username = username;
                cache.session_token = session_token;

                let mut redis_cluster_conn = Redis::get_cluster_connection().unwrap();

                let _: () = redis_cluster_conn.set(&cache.session_token, &cache.username)?;
                let _: () = redis_cluster_conn.expire(cache.session_token, 10)?;

                Ok(())
            }
        }
    }
}
