use crate::models::user::User;
use argon2::{
    Argon2, PasswordHasher,
    password_hash::{PasswordHash, PasswordVerifier},
};
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use jsonwebtoken::{EncodingKey, Header, encode};
use rand::{Rng, distributions::Alphanumeric};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::env;

#[derive(Deserialize)]
pub struct LoginPayload {
    email: String,
    password: String,
}

#[derive(Deserialize)]
pub struct VerifyPayload {
    token: String,
}

#[derive(Deserialize)]
pub struct RegisterPayload {
    email: String,
    password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    token: String,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // user id
    pub exp: usize,
}

#[axum::async_trait]
impl<S> axum::extract::FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, Json<ErrorResponse>);

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get(axum::http::header::AUTHORIZATION)
            .and_then(|h| h.to_str().ok())
            .ok_or((
                StatusCode::UNAUTHORIZED,
                Json(ErrorResponse {
                    error: "Missing authorization header".to_string(),
                }),
            ))?;

        if !auth_header.starts_with("Bearer ") {
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(ErrorResponse {
                    error: "Invalid authorization header".to_string(),
                }),
            ));
        }

        let token = &auth_header[7..];
        let secret = env::var("JWT_SECRET").unwrap_or_else(|_| "secret".to_string());

        let token_data = jsonwebtoken::decode::<Claims>(
            token,
            &jsonwebtoken::DecodingKey::from_secret(secret.as_bytes()),
            &jsonwebtoken::Validation::default(),
        )
        .map_err(|_| {
            (
                StatusCode::UNAUTHORIZED,
                Json(ErrorResponse {
                    error: "Invalid token".to_string(),
                }),
            )
        })?;

        Ok(token_data.claims)
    }
}

pub async fn login(
    State(pool): State<PgPool>,
    Json(payload): Json<LoginPayload>,
) -> impl IntoResponse {
    // 1. Find user by email
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
        .bind(&payload.email)
        .fetch_optional(&pool)
        .await;

    let user = match user {
        Ok(Some(u)) => {
            if !u.is_verified {
                return (
                    StatusCode::FORBIDDEN,
                    Json(ErrorResponse {
                        error: "Email not verified".to_string(),
                    }),
                )
                    .into_response();
            }
            u
        }
        Ok(None) => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(ErrorResponse {
                    error: "Invalid credentials".to_string(),
                }),
            )
                .into_response();
        }
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "Database error".to_string(),
                }),
            )
                .into_response();
        }
    };

    // 2. Verify password
    let parsed_hash = match PasswordHash::new(&user.password_hash) {
        Ok(h) => h,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "Auth error".to_string(),
                }),
            )
                .into_response();
        }
    };

    if Argon2::default()
        .verify_password(payload.password.as_bytes(), &parsed_hash)
        .is_err()
    {
        return (
            StatusCode::UNAUTHORIZED,
            Json(ErrorResponse {
                error: "Invalid credentials".to_string(),
            }),
        )
            .into_response();
    }

    // 3. Generate JWT
    let secret = env::var("JWT_SECRET").unwrap_or_else(|_| "secret".to_string());
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(24))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: user.id.to_string(),
        exp: expiration,
    };

    let token = match encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    ) {
        Ok(t) => t,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "Token creation failed".to_string(),
                }),
            )
                .into_response();
        }
    };

    (StatusCode::OK, Json(LoginResponse { token })).into_response()
}

pub async fn register(
    State(pool): State<PgPool>,
    Json(payload): Json<RegisterPayload>,
) -> impl IntoResponse {
    // 1. Check if user exists
    let user_exists = sqlx::query("SELECT id FROM users WHERE email = $1")
        .bind(&payload.email)
        .fetch_optional(&pool)
        .await;

    if let Ok(Some(_)) = user_exists {
        // Return 200 checks to avoid enumeration attacks, or 400 if we don't care about personal app
        return (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "User already exists".to_string(),
            }),
        )
            .into_response();
    }

    // 2. Hash password
    let salt =
        argon2::password_hash::SaltString::generate(&mut argon2::password_hash::rand_core::OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(payload.password.as_bytes(), &salt)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
        .unwrap()
        .to_string();

    // 3. Generate Token
    let verification_token: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect();

    // 4. Create User
    let insert_result = sqlx::query(
        "INSERT INTO users (email, password_hash, verification_token, is_verified) VALUES ($1, $2, $3, false)"
    )
    .bind(&payload.email)
    .bind(password_hash)
    .bind(&verification_token)
    .execute(&pool)
    .await;

    match insert_result {
        Ok(_) => {
            // 5. "Send" Email (Log it for now)
            tracing::info!(">>> MOCK EMAIL SENT <<<");
            tracing::info!("To: {}", payload.email);
            tracing::info!("Action: Verify Account");
            tracing::info!("Token: {}", verification_token);
            tracing::info!(
                "Link: http://localhost:8080/admin/verify?token={}",
                verification_token
            );
            tracing::info!(">>> END EMAIL <<<");

            (StatusCode::CREATED, Json(serde_json::json!({ "message": "Changes saved. Check backend logs for token." }))).into_response()
        }
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "Database error".to_string(),
            }),
        )
            .into_response(),
    }
}

pub async fn verify_email(
    State(pool): State<PgPool>,
    Json(payload): Json<VerifyPayload>,
) -> impl IntoResponse {
    let result = sqlx::query("UPDATE users SET is_verified = true, verification_token = NULL WHERE verification_token = $1")
        .bind(&payload.token)
        .execute(&pool)
        .await;

    match result {
        Ok(res) => {
            if res.rows_affected() > 0 {
                (
                    StatusCode::OK,
                    Json(serde_json::json!({ "message": "Email verified" })),
                )
                    .into_response()
            } else {
                (
                    StatusCode::BAD_REQUEST,
                    Json(ErrorResponse {
                        error: "Invalid token".to_string(),
                    }),
                )
                    .into_response()
            }
        }
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "Database error".to_string(),
            }),
        )
            .into_response(),
    }
}
