use sqlx::{sqlite, SqlitePool, Sqlite, Pool};
use sqlx::sqlite::SqliteConnectOptions;

pub async fn create_pool() -> Result<Pool<Sqlite>, sqlx::Error> {
    let connection_options: SqliteConnectOptions = sqlite::SqliteConnectOptions::new().filename("my_app.db")
        .create_if_missing(true);

    let pool = SqlitePool::connect_with(connection_options).await;

    sqlx::migrate!("src/db/migrations").run(pool.as_ref().unwrap()).await?;

    Ok(pool?)
}
