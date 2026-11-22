pub mod error;

#[derive(Debug, Clone)]
pub struct UserDto {
    pub id: u32,
    pub name: String,
}