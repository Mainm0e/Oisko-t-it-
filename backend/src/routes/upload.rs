use axum::{
    extract::Multipart,
    http::StatusCode,
    response::{IntoResponse, Json},
};
use serde_json::json;
use std::path::Path;
use tokio::fs;
use uuid::Uuid;

pub async fn upload_file(mut multipart: Multipart) -> impl IntoResponse {
    while let Some(field) = multipart.next_field().await.unwrap_or(None) {
        let name = field.name().unwrap_or("file").to_string();

        if name == "file" {
            let file_name = if let Some(file_name) = field.file_name() {
                file_name.to_string()
            } else {
                continue;
            };

            let data = if let Ok(data) = field.bytes().await {
                data
            } else {
                continue;
            };

            // Create uploads directory if it doesn't exist
            if !Path::new("uploads").exists() {
                let _ = fs::create_dir("uploads").await;
            }

            let new_filename = format!("{}_{}", Uuid::new_v4(), file_name);
            let filepath = format!("uploads/{}", new_filename);

            if let Ok(_) = fs::write(&filepath, &data).await {
                return (
                    StatusCode::OK,
                    Json(json!({
                        "url": format!("/uploads/{}", new_filename)
                    })),
                )
                    .into_response();
            }
        }
    }

    (StatusCode::BAD_REQUEST, "File upload failed").into_response()
}
