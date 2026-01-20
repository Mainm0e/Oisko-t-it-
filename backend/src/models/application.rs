use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Application {
    pub id: Uuid,
    pub user_id: Uuid,
    pub company: String,
    pub company_website: Option<String>,
    pub role: String,
    pub status: String,
    pub salary: Option<String>,
    pub contact_person: Option<String>,
    pub cv_version: Option<String>,
    pub cv_path: Option<String>,
    pub cover_letter: Option<String>,
    pub cover_letter_path: Option<String>,
    pub logo_url: Option<String>,
    pub description: Option<String>,
    #[sqlx(default)]
    pub comment_count: Option<i64>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DashboardStats {
    pub daily_activity: Vec<DailyCount>,
    pub status_distribution: Vec<StatusCount>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct DailyCount {
    pub date: String,
    pub count: i64,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct StatusCount {
    pub status: String,
    pub count: i64,
}

#[derive(Debug, Deserialize)]
pub struct CreateApplication {
    pub company: String,
    pub company_website: Option<String>,
    pub role: String,
    pub status: Option<String>,
    pub salary: Option<String>,
    pub contact_person: Option<String>,
    pub cv_version: Option<String>,
    pub cv_path: Option<String>,
    pub cover_letter: Option<String>,
    pub cover_letter_path: Option<String>,
    pub logo_url: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateApplication {
    pub company: Option<String>,
    pub company_website: Option<String>,
    pub role: Option<String>,
    pub status: Option<String>,
    pub salary: Option<String>,
    pub contact_person: Option<String>,
    pub cv_version: Option<String>,
    pub cv_path: Option<String>,
    pub cover_letter: Option<String>,
    pub cover_letter_path: Option<String>,
    pub logo_url: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, FromRow)]
pub struct AppInfo {
    pub company: String,
    pub role: String,
}
