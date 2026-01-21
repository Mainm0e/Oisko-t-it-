use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Comment {
    pub id: Uuid,
    pub application_id: Uuid,
    pub visitor_name: String,
    pub content: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct CreateComment {
    pub visitor_name: String,
    pub content: String,
    // Honeypot field
    pub bot_field: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct CommentWithContext {
    pub id: Uuid,
    pub application_id: Uuid,
    pub visitor_name: String,
    pub content: String,
    pub created_at: NaiveDateTime,
    pub company: String,
    pub role: String,
}
