use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use leptos::prelude::LeptosOptions;
        use sqlx::SqlitePool;
        use crate::utils::regex::InputValidationRegex;

        /// This takes advantage of Axum's SubStates feature by deriving FromRef. This is the only way to have more than one
        /// item in Axum's State. Leptos requires you to have leptosOptions in your State struct for the leptos route handlers
        #[derive(Debug, Clone)]
        pub struct AppState {
            pub leptos_options: LeptosOptions,
            pub pool: SqlitePool,
            pub validation_regex: InputValidationRegex,
        }
    }
}
