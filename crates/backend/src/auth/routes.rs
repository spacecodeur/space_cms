use axum::{
    extract::{Extension, Json},
    http::StatusCode,
    response::IntoResponse,
    Router,
    routing::post,
};
use serde_json::json;
use space_cms_shared::{AuthResponse, LoginRequest, RegisterRequest, User};
use sqlx::{SqlitePool, Row};
use uuid::Uuid;

use super::{create_jwt, hash_password, verify_password};

pub fn auth_routes<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/api/auth/register", post(register))
        .route("/api/auth/login", post(login))
}

async fn register(
    Extension(pool): Extension<SqlitePool>,
    Json(req): Json<RegisterRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    // Hash the password
    let password_hash = hash_password(&req.password)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Generate a new user ID
    let user_id = Uuid::new_v4().to_string();

    // Insert the new user
    let result = sqlx::query(
        "INSERT INTO users (id, username, email, password_hash) VALUES (?1, ?2, ?3, ?4)"
    )
    .bind(&user_id)
    .bind(&req.username)
    .bind(&req.email)
    .bind(&password_hash)
    .execute(&pool)
    .await;

    match result {
        Ok(_) => {
            // For now, create a user object manually
            // In production, you would fetch from DB with proper query
            let user = User {
                id: user_id,
                username: req.username,
                email: req.email,
                password_hash: None,
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            };

            // Create JWT token
            let token = create_jwt(&user.id)
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

            Ok(Json(AuthResponse {
                token,
                user: User {
                    password_hash: None, // Don't send password hash to client
                    ..user
                },
            }))
        }
        Err(e) => {
            eprintln!("Registration error: {}", e);
            if e.to_string().contains("UNIQUE") {
                Err(StatusCode::CONFLICT)
            } else {
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    }
}

async fn login(
    Extension(pool): Extension<SqlitePool>,
    Json(req): Json<LoginRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    // Find user by username
    let user_result = sqlx::query("SELECT id, username, email, password_hash, created_at, updated_at FROM users WHERE username = ?1")
        .bind(&req.username)
        .fetch_optional(&pool)
        .await;

    match user_result {
        Ok(Some(row)) => {
            let user = User {
                id: row.get("id"),
                username: row.get("username"),
                email: row.get("email"),
                password_hash: row.get("password_hash"),
                created_at: chrono::DateTime::from_timestamp(row.get::<i64, _>("created_at"), 0)
                    .unwrap_or_default()
                    .with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::from_timestamp(row.get::<i64, _>("updated_at"), 0)
                    .unwrap_or_default()
                    .with_timezone(&chrono::Utc),
            };

            // Verify password
            if let Some(hash) = &user.password_hash {
                if verify_password(&req.password, hash)
                    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
                {
                    // Create JWT token
                    let token = create_jwt(&user.id)
                        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

                    // Don't send password hash to client
                    let user_response = User {
                        password_hash: None,
                        ..user
                    };

                    Ok(Json(AuthResponse { token, user: user_response }))
                } else {
                    Err(StatusCode::UNAUTHORIZED)
                }
            } else {
                Err(StatusCode::UNAUTHORIZED)
            }
        }
        Ok(None) => Err(StatusCode::UNAUTHORIZED),
        Err(e) => {
            eprintln!("Login database error: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}