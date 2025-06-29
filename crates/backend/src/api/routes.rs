use axum::{routing::get, Router, Extension, middleware};
use leptos::prelude::*;
use sqlx::SqlitePool;
use crate::auth::{auth_routes, auth_middleware};

pub fn api_routes<S>(pool: SqlitePool) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
    LeptosOptions: axum::extract::FromRef<S>,
{
    let public_routes = Router::new()
        .route("/api/health", get(health_check))
        .merge(auth_routes());

    let protected_routes = Router::new()
        .route("/api/protected", get(protected_endpoint))
        .route_layer(middleware::from_fn(auth_middleware));

    Router::new()
        .merge(public_routes)
        .merge(protected_routes)
        .layer(Extension(pool))
}

async fn health_check() -> &'static str {
    "OK"
}

async fn protected_endpoint() -> &'static str {
    "This is a protected endpoint"
}
