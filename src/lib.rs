use serde::{Deserialize, Serialize};
use sqlx::FromRow;

pub mod handlers;

#[derive(Deserialize, Serialize, FromRow)]
pub struct User {
    pub id: Option<i32>,
    pub username: String,
    pub password: Option<String>
}
