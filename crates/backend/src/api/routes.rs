use axum::{routing::get, Router};
use leptos::prelude::*;

pub fn api_routes<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
    LeptosOptions: axum::extract::FromRef<S>,
{
    Router::new().route("/api/health", get(health_check))
}

async fn health_check() -> &'static str {
    "OK"
}
