use axum::{routing::get, Router, Extension};
use leptos::prelude::*;
use sqlx::SqlitePool;

pub fn api_routes<S>(pool: SqlitePool) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
    LeptosOptions: axum::extract::FromRef<S>,
{
    Router::new()
        .route("/api/health", get(health_check))
        .layer(Extension(pool))
}

async fn health_check() -> &'static str {
    "OK"
}
