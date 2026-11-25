use sqlx::{Error, SqlitePool};
use domain::error::AppError;
use crate::media::model::{MediaItem, MediaView};
use crate::media::repository::MediaRepository;

#[derive(Clone)]
pub struct MediaService {
    pool: SqlitePool,
    repo: MediaRepository,
}

impl MediaService {
    pub fn new(pool: SqlitePool, repo: MediaRepository) -> MediaService { Self { pool, repo } }

    pub async fn list_all(&self) -> Result<Vec<MediaItem>, Error> {
        self.repo.list_all().await
    }
}