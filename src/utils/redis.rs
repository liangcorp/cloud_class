use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use redis::cluster::ClusterClient;
        use redis::Commands;
        use leptos::server_fn::ServerFnError;

        pub struct Redis {
            username: String,
            password: String,
            uri_scheme: String,
            hostname: String
        }

        impl Default for Redis {
            fn default() -> Redis {
                //if Redis server needs secure connection
                // let uri_scheme = match env::var("IS_TLS") {
                //     Ok(_) => "rediss",
                //     Err(_) => "redis",
                // };
                Redis {
                    username: String::from("redis_user"),
                    password: String::from("redis_password"),
                    uri_scheme: String::from("rediss"),
                    hostname: String::from("192.168.110.228")
                }
            }
        }

        impl Redis {
            pub fn fetch_an_integer(&self) -> Result<String, ServerFnError> {

                let nodes = vec![format!("{}://{}:{}@{}/",
                            self.uri_scheme,
                            self.username,
                            self.password,
                            self.hostname)];
                let client = ClusterClient::new(nodes)?;

                let mut connection = client.get_connection()?;

                let _: () = connection.set("test", "test_data")?;
                let rv: String = connection.get("test")?;
                Ok(rv)
            }
        }
    }
}
