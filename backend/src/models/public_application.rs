use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct PublicApplication {
    pub id: Uuid,
    pub company: String,
    pub company_website: Option<String>,
    pub role: String,
    pub status: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct PublicApplicationDetail {
    pub id: Uuid,
    pub company: String,
    pub company_website: Option<String>,
    pub role: String,
    pub status: String,
    pub salary: Option<String>,
    pub cover_letter: Option<String>,
    pub cv_path: Option<String>,
    pub created_at: NaiveDateTime,
}
