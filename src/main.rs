use axum::{routing::{get, post}, Router, Extension};
use shuttle_secrets::SecretStore;
use sqlx::PgPool;
use tracing::info;

use to_synchronize::handlers;

#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres] pool: PgPool,
    #[shuttle_secrets::Secrets] secret_store: SecretStore
) -> shuttle_axum::ShuttleAxum {
    info!("Running database migrations...");
    sqlx::migrate!().run(&pool).await.expect("Migrations failed :(");
    info!("Migrations OK!");

    let router = Router::new()
        .route("/todos", get(handlers::get_todos))
        .route("/register", post(handlers::register))
        .layer(Extension(pool))
        .layer(Extension(secret_store));

    Ok(router.into())
}
