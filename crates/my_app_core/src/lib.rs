use domain::{error::AppError, UserDto};
use std::sync::Arc;

pub struct Config;
impl Config {
    pub fn from_env() -> Self { Config }
}

pub struct UserService;
impl UserService {
    pub async fn get_user(&self, id: u32) -> Result<UserDto, AppError> {
        Ok(UserDto { id, name: "demo".into() })
    }
}

pub struct App {
    pub users: UserService,
}
impl App {
    pub async fn new(_config: Config) -> Result<Arc<Self>, AppError> {
        Ok(Arc::new(App { users: UserService }))
    }
}