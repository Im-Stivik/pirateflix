mod db;

use db::db_pool;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let pool = db_pool::create_pool().await.unwrap();

    let test_query: Vec<TestStruct>= sqlx::query_as("SELECT * from test").fetch_all(&pool).await.unwrap();

    for row in test_query {
        println!("{:?}", row);
    }
}

#[derive(sqlx::prelude::FromRow, Debug)]
struct TestStruct {
    id: i32,
    test: String,
    test2: String,
}