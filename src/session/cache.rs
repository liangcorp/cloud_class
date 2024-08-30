use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use leptos::*;
        use server_fn::ServerFnError;
        use crate::utils::redis::Redis;
        use redis::Commands;

        #[derive(Debug)]
        pub struct Cache {
            key: String,
            value: String,
        }

        impl Default for Cache {
            fn default() -> Cache {
                Cache {
                    key: "".to_string(),
                    value: "".to_string(),
                }
            }
        }

        impl Cache {
            pub fn to_string(&self) -> String {
                format!("Cache: ( {} {} )", self.value, self.key)
            }

            pub fn set_cache(key: &str, value: &str) -> Result<(), ServerFnError> {
                let mut cache = Cache::default();
                cache.value = value.to_string();
                cache.key = key.to_string();

                let mut redis_cluster_conn = Redis::get_cluster_connection().unwrap();
                logging::log!("DEBUG<session/cache.rs>: set_cache - {}", cache.to_string());

                let _: () = redis_cluster_conn.set(&cache.key, &cache.value)?;
                let _: () = redis_cluster_conn.expire(cache.key, 2592000)?;

                Ok(())
            }

            pub fn delete_cache(key: &str) -> Result<(), ServerFnError> {
                let mut redis_cluster_conn = Redis::get_cluster_connection().unwrap();
                logging::log!("DEBUG<session/cache.rs>: delete_cache - {}", &key);

                let _: () = redis_cluster_conn.expire(key, 0)?;

                Ok(())
            }
        }
    }
}
