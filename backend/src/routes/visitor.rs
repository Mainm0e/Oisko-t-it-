use crate::models::visitor::{VisitResponse, Visitor};
use axum::{
    Json,
    extract::{ConnectInfo, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
};
use sha2::{Digest, Sha256};
use sqlx::PgPool;
use std::net::SocketAddr;

pub async fn record_visit(
    State(pool): State<PgPool>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
) -> impl IntoResponse {
    // 1. Get IP
    // Priority: X-Forwarded-For (if behind proxy), then direct connection
    let ip = headers
        .get("x-forwarded-for")
        .and_then(|val| val.to_str().ok())
        .map(|s| s.split(',').next().unwrap_or(s).trim().to_string())
        .unwrap_or_else(|| addr.ip().to_string());

    // 2. Hash IP (Simple Salt - In prod, load from env)
    let salt = "OISKO_SALT_v1";
    let mut hasher = Sha256::new();
    hasher.update(format!("{}{}", ip, salt));
    let ip_hash = format!("{:x}", hasher.finalize());

    // 3. Upsert into DB
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

    // 4. Get Total Count
    let total_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM visitors")
        .fetch_one(&pool)
        .await
        .unwrap_or(0);

    match result {
        Ok(visitor) => Json(VisitResponse {
            is_first_visit: visitor.visit_count == 1,
            total_unique_visitors: total_count,
        })
        .into_response(),
        Err(e) => {
            tracing::error!("Visitor error: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response()
        }
    }
}
