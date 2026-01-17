use crate::models::visitor::{VisitResponse, Visitor};
use axum::{
    Json,
    extract::State,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
};
use sha2::{Digest, Sha256};
use sqlx::PgPool;

pub async fn record_visit(State(pool): State<PgPool>, headers: HeaderMap) -> impl IntoResponse {
    tracing::info!("RECORD_VISIT REQUEST RECEIVED");

    // 1. Get IP
    let ip = headers
        .get("x-forwarded-for")
        .and_then(|val| val.to_str().ok())
        .map(|s| s.split(',').next().unwrap_or(s).trim().to_string())
        .unwrap_or_else(|| "0.0.0.0".to_string());

    tracing::info!("Visitor Source Detected: {}", ip);

    // 2. Check if this is the first global visit of the day (Finland Time)
    let was_someone_here_today: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM visitors WHERE last_seen_at::date = (CURRENT_TIMESTAMP AT TIME ZONE 'Europe/Helsinki')::date)",
    )
    .fetch_one(&pool)
    .await
    .unwrap_or(false);

    // 3. Hash IP
    let salt = "OISKO_SALT_v1";
    let mut hasher = Sha256::new();
    hasher.update(format!("{}{}", ip, salt));
    let ip_hash = format!("{:x}", hasher.finalize());

    // 4. Upsert into DB
    let result = sqlx::query_as::<_, Visitor>(
        r#"
        INSERT INTO visitors (ip_hash, visit_count, first_seen_at, last_seen_at)
        VALUES ($1, 1, NOW(), NOW())
        ON CONFLICT (ip_hash) 
        DO UPDATE SET 
            visit_count = visitors.visit_count + 1,
            last_seen_at = NOW()
        RETURNING *
        "#,
    )
    .bind(&ip_hash)
    .fetch_one(&pool)
    .await;

    // 5. Get Counts
    let total_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM visitors")
        .fetch_one(&pool)
        .await
        .unwrap_or(0);

    let today_count: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM visitors WHERE last_seen_at::date = (CURRENT_TIMESTAMP AT TIME ZONE 'Europe/Helsinki')::date")
            .fetch_one(&pool)
            .await
            .unwrap_or(0);

    let is_first_of_day = !was_someone_here_today;
    if is_first_of_day {
        tracing::info!("TACTICAL ALERT: This is the first visitor of the day!");
    }

    match result {
        Ok(visitor) => {
            tracing::info!("SUCCESS: Intercepted signal from {}", visitor.ip_hash);
            Json(VisitResponse {
                is_first_visit: visitor.visit_count == 1,
                is_first_of_day,
                total_unique_visitors: total_count,
                today_visitors: today_count,
            })
            .into_response()
        }
        Err(e) => {
            tracing::error!("DATABASE ERROR during visitor log: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Intercept Failure: {}", e),
            )
                .into_response()
        }
    }
}
