pub mod media;
mod db;

use domain::{error::AppError, UserDto};
use std::sync::Arc;

use media::service;
use crate::media::service::MediaService;
use db::db_pool;
use crate::db::db_pool::create_pool;
use crate::media::repository::MediaRepository;

pub struct Config;
impl Config {
    pub fn from_env() -> Self { Config }
}

pub struct App {
    pub media: MediaService
}

impl App {
    pub async fn new(_config: Config) -> Result<Arc<Self>, AppError> {
        let pool = create_pool().await.unwrap();
        
        Ok(Arc::new(App {
            media: MediaService::new(pool.clone(), MediaRepository::new(pool.clone()))
        }))
    }
}