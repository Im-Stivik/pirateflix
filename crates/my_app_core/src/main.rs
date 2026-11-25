use my_app_core::{App, Config, media};

#[tokio::main]
async fn main() {
    let app = App::new(Config::from_env()).await.unwrap();

    println!("Hello, world!");

    let media_items: Vec<media::model::MediaItem> = app.media.list_all().await.unwrap();

    for item in media_items {
        println!("{:#?}", item);
    }
}

#[derive(sqlx::prelude::FromRow, Debug)]
struct TestStruct {
    id: i32,
    test: String,
    test2: String,
}