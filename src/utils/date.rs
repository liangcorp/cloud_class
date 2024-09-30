use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use chrono::{Datelike, Utc};

        pub fn get_current_date() -> String {
            let now = Utc::now();

            format!(
                "{:?}-{:?}-{:?}",
                now.year(),
                now.month(),
                now.day(),
            )
        }
    }
}
