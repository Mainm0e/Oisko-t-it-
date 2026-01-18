use crate::routes::auth::{Claims, ErrorResponse};
use axum::{Json, extract::Multipart, http::StatusCode, response::IntoResponse};
use std::path::Path;
use tokio::fs;
use uuid::Uuid;

const MAX_FILE_SIZE: usize = 5 * 1024 * 1024; // 5MB
const ALLOWED_EXTENSIONS: &[&str] = &["pdf", "docx", "png", "jpg", "jpeg", "webp"];

pub async fn upload_file(
    _claims: Claims, // Enforce authentication
    mut multipart: Multipart,
) -> impl IntoResponse {
    while let Some(field) = multipart.next_field().await.unwrap_or(None) {
        let name = field.name().unwrap_or("file").to_string();

        if name == "file" {
            let file_name = if let Some(file_name) = field.file_name() {
                file_name.to_string()
            } else {
                continue;
            };

            // 1. Validate extension
            let extension = Path::new(&file_name)
                .extension()
                .and_then(|ext| ext.to_str())
                .unwrap_or("")
                .to_lowercase();

            if !ALLOWED_EXTENSIONS.contains(&extension.as_str()) {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(ErrorResponse {
                        error: format!("File type .{} not allowed", extension),
                    }),
                )
                    .into_response();
            }

            let data = if let Ok(data) = field.bytes().await {
                data
            } else {
                continue;
            };

            // 2. Validate size
            if data.len() > MAX_FILE_SIZE {
                return (
                    StatusCode::PAYLOAD_TOO_LARGE,
                    Json(ErrorResponse {
                        error: "File too large (Max 5MB)".to_string(),
                    }),
                )
                    .into_response();
            }

            // Create uploads directory if it doesn't exist
            if !Path::new("uploads").exists() {
                let _ = fs::create_dir("uploads").await;
            }

            // 3. Sanitize filename (Uuid prefix + extension)
            let safe_filename = format!("{}.{}", Uuid::new_v4(), extension);
            let filepath = format!("uploads/{}", safe_filename);

            if let Ok(_) = fs::write(&filepath, &data).await {
                return (
                    StatusCode::OK,
                    Json(serde_json::json!({
                        "url": format!("/uploads/{}", safe_filename)
                    })),
                )
                    .into_response();
            }
        }
    }

    (
        StatusCode::BAD_REQUEST,
        Json(ErrorResponse {
            error: "File upload failed".to_string(),
        }),
    )
        .into_response()
}
