use leptos::*;
use cfg_if::cfg_if;
use server_fn::ServerFnError;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use sqlx::{Connection, SqliteConnection, Pool, Postgres, Sqlite};
        use sqlx::postgres::PgPoolOptions;
        use sqlx::sqlite::SqlitePoolOptions;

        // use crate::utils::app_state::AppState;

        pub async fn db() -> Result<SqliteConnection, ServerFnError> {
            Ok(SqliteConnection::connect("sqlite:Users.db").await?)
        }

        pub async fn create_pg_pool() -> Result<Pool<Postgres>, ServerFnError> {
            Ok(PgPoolOptions::new()
                .max_connections(5)
                .connect("postgres://postgres:password@localhost/test").await?)
        }

        pub async fn create_pool() -> Result<Pool<Sqlite>, ServerFnError> {
            Ok(SqlitePoolOptions::new()
                .max_connections(5)
                .connect("sqlite:Users.db").await?)
        }

        // pub async fn get_sqlite_conn() -> Result<SqliteConnection, ServerFnError> {
        //     let state = use_context::<AppState>();
        //     Ok(state.unwrap().pool.aquire().await?)
        // }
    }
}
