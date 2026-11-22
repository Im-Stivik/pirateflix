use my_app_core::{App, Config};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let app = App::new(Config::from_env()).await?;
    let router = api::build_router(app);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    tracing::info!("listening on http://127.0.0.1:3000");
    axum::serve(listener, router).await?;
    Ok(())
}