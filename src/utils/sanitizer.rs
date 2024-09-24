use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        pub fn sanitize(_dirty: String) -> String {
            "".to_string()
        }
    }
}
