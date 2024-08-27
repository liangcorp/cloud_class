use leptos::*;
use cfg_if::cfg_if;
use server_fn::ServerFnError;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use crate::state::AppState;

        pub fn create_cache(session_token: &str) -> Result<(), ServerFnError> {
            //  取得软件状态
            let state;
            match use_context::<AppState>() {
                Some(s) => state = s,
                None => return Err(ServerFnError::Args("ERROR<session/cache.rs>: during application state retrieval".to_string())),
            }

            //  取得数据库信息
            let pool = state.pool;

            Ok(())
        }
    }
}
