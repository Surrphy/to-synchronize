use axum::{
    extract::{Json, Multipart, Query}, http::StatusCode, Extension,
};
use shuttle_secrets::SecretStore;
use sqlx::{PgPool, Row};
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::User;

type Response = Result<String, (StatusCode, String)>;

pub async fn get_todos(Extension(pool): Extension<PgPool>, Query(user): Query<User>) -> Result<Vec<u8>, (StatusCode, String)> {
    let Some(user_id) = auth_user(&pool, &user).await? else {
        return Err((StatusCode::UNAUTHORIZED, "Unauthorized".into()))
    };

    match sqlx::query("SELECT todos FROM todos WHERE user_id = $1")
        .bind(user_id)
        .fetch_one(&pool)
        .await {
            Ok(row) => Ok(row.get("todos")),
            Err(_) => Err((StatusCode::INTERNAL_SERVER_ERROR, "Error while querying todos".into()))
    }
}

pub async fn post_todos(Extension(pool): Extension<PgPool>, Query(user): Query<User>, mut todos: Multipart) -> Response {
    let Some(user_id) = auth_user(&pool, &user).await? else {
        return Err((StatusCode::UNAUTHORIZED, "Unauthorized".into()))
    };

    let Some(field) = todos.next_field().await.unwrap() else {
        return Err((StatusCode::BAD_REQUEST, "No multipart data".into()))
    };

    let Ok(data) = field.bytes().await else {
        return Err((StatusCode::BAD_REQUEST, "Wrong multipart data".into()))
    };
    
    match sqlx::query("
INSERT INTO todos (user_id, todos)
VALUES ($1, $2)
ON CONFLICT (user_id)
DO UPDATE SET todos = EXCLUDED.todos
WHERE todos.user_id = EXCLUDED.user_id
")
        .bind(user_id)
        .bind(data.to_vec())
        .execute(&pool)
        .await {
            Ok(_) => Ok("Ok".into()),
            Err(err) => {info!("{err:?}"); return Err((StatusCode::INTERNAL_SERVER_ERROR, "Error while creating post".into()))}
        }
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

async fn auth_user(pool: &PgPool, user: &User) -> Result<Option<i32>, (StatusCode, String)> {
    let db_user: User = match sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = $1")
        .bind(&user.username)
        .fetch_one(pool)
        .await {
            Ok(row) => row,
            Err(_) => return Err((StatusCode::UNAUTHORIZED, "Unauthorized".into()))
        };

    if db_user.password == user.password { Ok(db_user.id) } else { Ok(None) }
}
