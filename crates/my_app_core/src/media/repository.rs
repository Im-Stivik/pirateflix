use sqlx::{Error, SqlitePool};
use domain::error::AppError;
use crate::media::model::MediaItem;

#[derive(Clone)]
pub struct MediaRepository {pool: SqlitePool}

impl MediaRepository {
    pub fn new(pool: SqlitePool) -> Self { Self { pool } }

    pub async fn list_all(&self) -> Result<Vec<MediaItem>, Error> {
        sqlx::query_as("select * from media_items").fetch_all(&self.pool).await
    }
}