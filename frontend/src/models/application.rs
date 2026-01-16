use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Application {
    pub id: Uuid,
    #[serde(default)]
    // backend sends user_id, but frontend might not always need it or it might be missing in some contexts? Actually backend Application struct has it.
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
    #[serde(default)]
    pub comment_count: Option<i64>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateApplicationPayload {
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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateApplicationPayload {
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

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct PublicApplication {
    pub id: Uuid,
    pub company: String,
    pub company_website: Option<String>,
    pub role: String,
    pub status: String,
    pub logo_url: Option<String>,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct PublicApplicationDetail {
    pub id: Uuid,
    pub company: String,
    pub company_website: Option<String>,
    pub role: String,
    pub status: String,
    pub salary: Option<String>,
    pub cover_letter: Option<String>,
    pub cv_path: Option<String>,
    pub logo_url: Option<String>,
    pub description: Option<String>,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Comment {
    pub id: Uuid,
    pub application_id: Uuid,
    pub visitor_name: String,
    pub content: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CreateComment {
    pub visitor_name: String,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CommentWithContext {
    pub id: Uuid,
    pub application_id: Uuid,
    pub visitor_name: String,
    pub content: String,
    pub created_at: chrono::NaiveDateTime,
    pub company: String,
    pub role: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct DashboardStats {
    pub daily_activity: Vec<DailyCount>,
    pub status_distribution: Vec<StatusCount>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct DailyCount {
    pub date: String,
    pub count: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct StatusCount {
    pub status: String,
    pub count: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CompanyIntel {
    pub company_name: Option<String>,
    pub description: Option<String>,
    pub logo_url: Option<String>,
}
