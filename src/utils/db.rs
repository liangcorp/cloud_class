use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use leptos::server_fn::ServerFnError;
        use sqlx::{Connection, SqliteConnection, Pool, Postgres, Sqlite};
        use sqlx::postgres::PgPoolOptions;
        use sqlx::sqlite::SqlitePoolOptions;

        pub async fn db() -> Result<SqliteConnection, ServerFnError> {
            Ok(SqliteConnection::connect("sqlite:Cloud_class.db").await?)
        }

        // @TODO use this in the future
        pub async fn create_pg_pool() -> Result<Pool<Postgres>, ServerFnError> {
            Ok(PgPoolOptions::new()
                .max_connections(5)
                .connect("postgres://postgres:password@localhost/test").await?)
        }

        pub async fn create_pool() -> Result<Pool<Sqlite>, ServerFnError> {
            Ok(SqlitePoolOptions::new()
                .max_connections(5)
                .connect("sqlite:Cloud_class.db").await?)
        }
    }
}
