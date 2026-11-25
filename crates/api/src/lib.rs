use axum::{extract::{Path, State}, routing::get, Json, Router};
use my_app_core::App;
use std::sync::Arc;
use domain::error::AppError;
use my_app_core::media::model::MediaItem;

pub fn build_router(app: Arc<App>) -> Router {
    Router::new().route("/media", get(list_media)).with_state(app)
}

async fn list_media(State(app): State<Arc<App>>) -> Json<Vec<MediaItem>> {
    let media = app.media.list_all().await.unwrap();
    Json(media)
}