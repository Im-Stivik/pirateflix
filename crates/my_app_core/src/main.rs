use sqlx::{sqlite, Executor, prelude::FromRow};
use sqlx::sqlite::SqliteConnectOptions;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let connection_options: SqliteConnectOptions = sqlite::SqliteConnectOptions::new().filename("my_app.db")
        .create_if_missing(true);

    let connection = sqlite::SqlitePool::connect_with(connection_options).await.unwrap();

    connection.execute("CREATE TABLE IF NOT EXISTS users (\
    id SERIAL PRIMARY KEY, \
    username TEXT NOT NULL)").await.unwrap();

    println!("Created table");

    sqlx::query("INSERT INTO users (username) VALUES (?1)").bind(&"test_user_1").execute(&connection)
        .await.unwrap();
    sqlx::query("INSERT INTO users (username) VALUES (?1)").bind(&"test_user_2").execute(&connection)
        .await.unwrap();
    sqlx::query("INSERT INTO users (username) VALUES (?1)").bind(&"test_user_3").execute(&connection)
        .await.unwrap();

    println!("Created rows");

    let users: Vec<TestUser>= sqlx::query_as("SELECT * FROM users").fetch_all(&connection)
        .await.unwrap();

    for user in users {
        println!("{:?}", user);
    }
}

#[derive(FromRow, Debug)]
struct TestUser {
    id: i32,
    username: String,
}