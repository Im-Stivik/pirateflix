use serde::Serialize;

#[derive(Debug, Clone, sqlx::FromRow, Serialize)]
pub struct MediaItem {
    id: i32,
    imdb_id: Option<String>,
    media_type: String,
    title: String,
    overview: Option<String>,
    poster_path: Option<String>,
    backdrop_path: Option<String>,
    release_year: Option<i32>,
    rating: Option<f32>,
}

#[derive(Debug, Serialize)]
pub struct MediaView {
    id: i32,
    media_type: String,
    title: String,
    poster_path: String,
    backdrop_path: String,
}