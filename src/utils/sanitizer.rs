use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        pub fn sanitize_username(username: String) -> Result<(), &'static str> {
            Ok(())
        }
    }
}
