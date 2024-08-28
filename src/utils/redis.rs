use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use redis::cluster::ClusterClient;
        use redis::Commands;
        use leptos::server_fn::ServerFnError;

        pub fn fetch_an_integer() -> Result<String, ServerFnError> {
            let nodes = vec!["redis://redis_user:redis_password@192.168.110.228/"];
            let client = ClusterClient::new(nodes)?;

            let mut connection = client.get_connection()?;

            let _: () = connection.set("test", "test_data")?;
            let rv: String = connection.get("test")?;
            Ok(rv)
        }
    }
}
