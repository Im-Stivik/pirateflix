use axum::{extract::{Path, State}, routing::get, Json, Router};
use my_app_core::App;
use std::sync::Arc;

pub fn build_router(app: Arc<App>) -> Router {
    Router::new().route("/users/{id}", get(get_user)).with_state(app)
}

async fn get_user(State(app): State<Arc<App>>, Path(id): Path<u32>) -> Json<String> {
    let user = app.users.get_user(id).await.unwrap();
    Json(user.name)
}