use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use std::fmt;
        use leptos::{server_fn::ServerFnError};
        use redis::{Client, Connection, cluster::{ClusterClient, ClusterConnection}};

        #[allow(dead_code)]
        pub struct Redis {
            username: String,
            password: String,
            uri_scheme: String,
            hostname: String,
            port: String
        }

        impl Default for Redis {
            fn default() -> Redis {
                //if Redis server needs secure connection
                // let uri_scheme = match env::var("IS_TLS") {
                //     Ok(_) => "rediss",
                //     Err(_) => "redis",
                // };
                Redis {
                    username: String::from(""),
                    password: String::from("cikq5XxudvHKUzdPgbQWokCOOhfT8wGQKPsLhBx8Tlw="),
                    uri_scheme: String::from("redis"),
                    hostname: String::from("127.0.0.1"),
                    port: String::from("6379")
                }
            }
        }

        impl fmt::Display for Redis {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                let redis = Redis::default();
                write!(f, "{}://{}:{}@{}:{}/", redis.uri_scheme, redis.username, redis.password, redis.hostname, redis.port)
            }
        }

        impl Redis {
            pub fn get_single_connection() -> Result<Connection, ServerFnError> {
                let client = match Client::open(Redis::default().to_string()) {
                    Ok(c) => c,
                    Err(e) => return Err(ServerFnError::Args(e.to_string())),
                };

                let connection;
                match client.get_connection() {
                    Ok(c) => connection = c,
                    Err(e) => return Err(ServerFnError::Args(e.to_string())),
                }

                Ok(connection)
            }

            pub fn get_cluster_connection() -> Result<ClusterConnection, ServerFnError> {

                // let nodes = vec![format!("{}://{}:{}@{}:{}/",
                //             self.uri_scheme,
                //             self.username,
                //             self.password,
                //             self.hostname,
                //             self.port)];
                //@TODO change this to new user for better security
                let nodes = vec!["redis://:cikq5XxudvHKUzdPgbQWokCOOhfT8wGQKPsLhBx8Tlw=@192.168.110.221:7000/",
                                    "redis://:cikq5XxudvHKUzdPgbQWokCOOhfT8wGQKPsLhBx8Tlw=@192.168.110.221:7001/",
                                    "redis://:cikq5XxudvHKUzdPgbQWokCOOhfT8wGQKPsLhBx8Tlw=@192.168.110.221:7002/",
                                    "redis://:cikq5XxudvHKUzdPgbQWokCOOhfT8wGQKPsLhBx8Tlw=@192.168.110.221:7003/",
                                    "redis://:cikq5XxudvHKUzdPgbQWokCOOhfT8wGQKPsLhBx8Tlw=@192.168.110.221:7004/",
                                    "redis://:cikq5XxudvHKUzdPgbQWokCOOhfT8wGQKPsLhBx8Tlw=@192.168.110.221:7005/"];

                let client;

                match ClusterClient::new(nodes) {
                    Ok(c) => client = c,
                    Err(e) => {
                        return Err(ServerFnError::Args(e.to_string()))
                    },
                }

                let connection;
                match client.get_connection() {
                    Ok(c) => connection = c,
                    Err(e) => {
                        return Err(ServerFnError::Args(e.to_string()))
                    },
                }
                Ok(connection)
                // let _: () = connection.set("test", "test_data")?;
                // let _: () = connection.expire("test", 10)?;
                // let rv: String = connection.get("test")?;
                // Ok(rv)
            }
        }
    }
}
