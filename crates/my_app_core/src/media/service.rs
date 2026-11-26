use sqlx::{Error};
use crate::media::model::{MediaItem};
use crate::media::repository::MediaRepository;

#[derive(Clone)]
pub struct MediaService {
    repo: MediaRepository,
}

impl MediaService {
    pub fn new(repo: MediaRepository) -> MediaService { Self {repo } }

    pub async fn list_all(&self) -> Result<Vec<MediaItem>, Error> {
        self.repo.list_all().await
    }
}