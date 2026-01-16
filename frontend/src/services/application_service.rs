use crate::models::application::{Application, CreateApplicationPayload, UpdateApplicationPayload};
use dioxus::prelude::*;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};

pub const BASE_URL: &str = "http://localhost:3000";
pub const API_BASE_URL: &str = "http://localhost:3000/api";

pub async fn get_token() -> Option<String> {
    dioxus_logger::tracing::info!("DEBUG: get_token called");
    let mut eval = document::eval(
        "
        console.log('JS: getting token');
        let token = localStorage.getItem('admin_token');
        console.log('JS: sending token', token);
        dioxus.send(token);
    ",
    );
    dioxus_logger::tracing::info!("DEBUG: eval created, waiting for recv");

    match eval.recv::<serde_json::Value>().await {
        Ok(val) => {
            dioxus_logger::tracing::info!("DEBUG: eval received: {:?}", val);
            if let Some(token) = val.as_str() {
                return Some(token.to_string());
            }
        }
        Err(e) => {
            dioxus_logger::tracing::error!("DEBUG: eval recv error: {}", e);
        }
    }
    dioxus_logger::tracing::warn!("DEBUG: get_token returning None");
    None
}

pub async fn list_applications() -> Result<Vec<Application>, String> {
    let token = get_token().await.ok_or("No token found")?;
    let client = reqwest::Client::new();

    let res = client
        .get(format!("{}/applications", API_BASE_URL))
        .header(AUTHORIZATION, format!("Bearer {}", token))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if res.status().is_success() {
        res.json::<Vec<Application>>()
            .await
            .map_err(|e| e.to_string())
    } else {
        Err(format!("Request failed: {}", res.status()))
    }
}

pub async fn create_application(payload: CreateApplicationPayload) -> Result<Application, String> {
    let token = get_token().await.ok_or("No token found")?;
    let client = reqwest::Client::new();

    let res = client
        .post(format!("{}/applications", API_BASE_URL))
        .header(AUTHORIZATION, format!("Bearer {}", token))
        .header(CONTENT_TYPE, "application/json")
        .json(&payload)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if res.status().is_success() {
        res.json::<Application>().await.map_err(|e| e.to_string())
    } else {
        let status = res.status();
        let text = res.text().await.unwrap_or_default();
        Err(format!("Request failed: {} - {}", status, text))
    }
}

pub async fn update_application(
    id: &str,
    payload: UpdateApplicationPayload,
) -> Result<Application, String> {
    let token = get_token().await.ok_or("No token found")?;
    let client = reqwest::Client::new();

    let res = client
        .put(format!("{}/applications/{}", API_BASE_URL, id))
        .header(AUTHORIZATION, format!("Bearer {}", token))
        .header(CONTENT_TYPE, "application/json")
        .json(&payload)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if res.status().is_success() {
        res.json::<Application>().await.map_err(|e| e.to_string())
    } else {
        Err(format!("Request failed: {}", res.status()))
    }
}

pub async fn delete_application(id: &str) -> Result<(), String> {
    let token = get_token().await.ok_or("No token found")?;
    let client = reqwest::Client::new();

    let res = client
        .delete(format!("{}/applications/{}", API_BASE_URL, id))
        .header(AUTHORIZATION, format!("Bearer {}", token))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if res.status().is_success() {
        Ok(())
    } else {
        Err(format!("Request failed: {}", res.status()))
    }
}

pub async fn upload_file(file_data: Vec<u8>, file_name: String) -> Result<String, String> {
    let token = get_token().await.ok_or("No token found")?;
    let client = reqwest::Client::new();

    // Create multipart form
    let part = reqwest::multipart::Part::bytes(file_data).file_name(file_name);
    let form = reqwest::multipart::Form::new().part("file", part);

    let res = client
        .post(format!("{}/upload", API_BASE_URL))
        .header(AUTHORIZATION, format!("Bearer {}", token))
        .multipart(form)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if res.status().is_success() {
        let json: serde_json::Value = res.json().await.map_err(|e| e.to_string())?;
        Ok(json["url"].as_str().unwrap_or("").to_string())
    } else {
        Err("Upload failed".to_string())
    }
}

pub async fn get_application(id: &str) -> Result<Application, String> {
    let token = get_token().await.ok_or("No token found")?;
    let client = reqwest::Client::new();

    let res = client
        .get(format!("{}/applications/{}", API_BASE_URL, id))
        .header(AUTHORIZATION, format!("Bearer {}", token))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if res.status().is_success() {
        res.json::<Application>().await.map_err(|e| e.to_string())
    } else {
        Err(format!("Request failed: {}", res.status()))
    }
}

pub async fn get_public_applications(
) -> Result<Vec<crate::models::application::PublicApplication>, String> {
    let client = reqwest::Client::new();

    let res = client
        .get(format!("{}/public/applications", API_BASE_URL))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if res.status().is_success() {
        res.json::<Vec<crate::models::application::PublicApplication>>()
            .await
            .map_err(|e| e.to_string())
    } else {
        Err(format!("Request failed: {}", res.status()))
    }
}

pub async fn get_public_application_detail(
    id: &str,
) -> Result<crate::models::application::PublicApplicationDetail, String> {
    let client = reqwest::Client::new();

    let res = client
        .get(format!("{}/public/applications/{}", API_BASE_URL, id))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if res.status().is_success() {
        res.json::<crate::models::application::PublicApplicationDetail>()
            .await
            .map_err(|e| e.to_string())
    } else {
        Err(format!("Request failed: {}", res.status()))
    }
}

pub async fn get_comments(
    application_id: &str,
) -> Result<Vec<crate::models::application::Comment>, String> {
    let client = reqwest::Client::new();

    let res = client
        .get(format!(
            "{}/public/applications/{}/comments",
            API_BASE_URL, application_id
        ))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if res.status().is_success() {
        res.json::<Vec<crate::models::application::Comment>>()
            .await
            .map_err(|e| e.to_string())
    } else {
        Err(format!("Request failed: {}", res.status()))
    }
}

pub async fn create_comment(
    application_id: &str,
    payload: crate::models::application::CreateComment,
) -> Result<crate::models::application::Comment, String> {
    let client = reqwest::Client::new();

    let res = client
        .post(format!(
            "{}/public/applications/{}/comments",
            API_BASE_URL, application_id
        ))
        .header(CONTENT_TYPE, "application/json")
        .json(&payload)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if res.status().is_success() {
        res.json::<crate::models::application::Comment>()
            .await
            .map_err(|e| e.to_string())
    } else {
        Err(format!("Request failed: {}", res.status()))
    }
}

pub async fn get_recent_comments(
) -> Result<Vec<crate::models::application::CommentWithContext>, String> {
    let token = get_token().await.ok_or("No token found")?;
    let client = reqwest::Client::new();

    let res = client
        .get(format!("{}/comments/recent", API_BASE_URL))
        .header(AUTHORIZATION, format!("Bearer {}", token))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if res.status().is_success() {
        res.json::<Vec<crate::models::application::CommentWithContext>>()
            .await
            .map_err(|e| e.to_string())
    } else {
        Err(format!("Request failed: {}", res.status()))
    }
}

pub async fn get_dashboard_stats() -> Result<crate::models::application::DashboardStats, String> {
    let token = get_token().await.ok_or("No token found")?;
    let client = reqwest::Client::new();

    let res = client
        .get(format!("{}/applications/stats", API_BASE_URL))
        .header(reqwest::header::AUTHORIZATION, format!("Bearer {}", token))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if res.status().is_success() {
        res.json::<crate::models::application::DashboardStats>()
            .await
            .map_err(|e| e.to_string())
    } else {
        Err(format!("Request failed: {}", res.status()))
    }
}
