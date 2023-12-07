use axum::{
    extract::Json, http::StatusCode, Extension,
};
use shuttle_secrets::SecretStore;
use sqlx::PgPool;
use serde::{Deserialize, Serialize};

use crate::User;

type Response = Result<String, (StatusCode, String)>;

pub async fn get_todos(Extension(pool): Extension<PgPool>, Json(user): Json<User>) -> Response {
    if !auth_user(&pool, &user).await? {
        return Err((StatusCode::UNAUTHORIZED, "Unauthorized".into()));
    }

    Ok("Good to go!".into())
}


#[derive(Deserialize, Serialize)]
pub struct RegisterForm {
    password: String,
    username: String,
    api_token: String,
}

pub async fn register(Extension(pool): Extension<PgPool>, Extension(secret_store): Extension<SecretStore>, Json(form): Json<RegisterForm>) -> Response {
    if form.api_token != secret_store.get("API_KEY").unwrap() {
        return Err((StatusCode::UNAUTHORIZED, "Unauthorized".into()));
    }

    match sqlx::query("INSERT INTO users (username, password) VALUES ($1, $2)")
        .bind(form.username)
        .bind(form.password)
        .execute(&pool)
        .await {
            Ok(_) => Ok("Ok".into()),
            Err(_) => Err((StatusCode::INTERNAL_SERVER_ERROR, "Error while creating user".into()))
        }
}

async fn auth_user(pool: &PgPool, user: &User) -> Result<bool, (StatusCode, String)> {
    let db_user: User = match sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = $1")
        .bind(&user.username)
        .fetch_one(pool)
        .await {
            Ok(row) => row,
            Err(_) => return Err((StatusCode::UNAUTHORIZED, "Unauthorized".into()))
        };

    Ok(db_user.password == user.password)
}
