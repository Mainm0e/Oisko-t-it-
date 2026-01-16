use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Visitor {
    pub ip_hash: String,
    pub visit_count: i32,
    pub first_seen_at: NaiveDateTime,
    pub last_seen_at: NaiveDateTime,
}

#[derive(Serialize)]
pub struct VisitResponse {
    pub is_first_visit: bool,
    pub total_unique_visitors: i64,
}
