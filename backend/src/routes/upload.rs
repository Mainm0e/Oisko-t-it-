use crate::routes::auth::{Claims, ErrorResponse};
use axum::{extract::Multipart, http::StatusCode, response::IntoResponse, Json};
use std::path::Path;
use tokio::fs;
use uuid::Uuid;

const MAX_FILE_SIZE: usize = 10 * 1024 * 1024; // 10MB (Matching the main.rs limit)
const ALLOWED_EXTENSIONS: &[&str] = &["pdf", "docx", "png", "jpg", "jpeg", "webp"];

pub async fn upload_file(
    _claims: Claims, // Enforce authentication
    mut multipart: Multipart,
) -> impl IntoResponse {
    tracing::info!("UPLOAD: Handler started");

    loop {
        let field_result = multipart.next_field().await;

        match field_result {
            Ok(Some(field)) => {
                let name = field.name().unwrap_or("unknown").to_string();
                tracing::info!("UPLOAD: Found field name='{}'", name);

                if name == "file" {
                    let file_name = if let Some(fn_val) = field.file_name() {
                        fn_val.to_string()
                    } else {
                        tracing::warn!("UPLOAD: Field 'file' has no filename, skipping");
                        continue;
                    };
                    tracing::info!("UPLOAD: Processing file '{}'", file_name);

                    // 1. Validate extension
                    let extension = Path::new(&file_name)
                        .extension()
                        .and_then(|ext| ext.to_str())
                        .unwrap_or("")
                        .to_lowercase();

                    tracing::info!("UPLOAD: Extension detected: '{}'", extension);

                    if !ALLOWED_EXTENSIONS.contains(&extension.as_str()) {
                        tracing::error!(
                            "UPLOAD: Extension '{}' not allowed. Allowed: {:?}",
                            extension,
                            ALLOWED_EXTENSIONS
                        );
                        return (
                            StatusCode::BAD_REQUEST,
                            Json(ErrorResponse {
                                error: format!("File type .{} not allowed", extension),
                            }),
                        )
                            .into_response();
                    }

                    let data = match field.bytes().await {
                        Ok(d) => d,
                        Err(e) => {
                            tracing::error!("UPLOAD: Failed to read bytes: {}", e);
                            continue;
                        }
                    };

                    tracing::info!("UPLOAD: File size: {} bytes", data.len());

                    // 2. Validate size
                    if data.len() > MAX_FILE_SIZE {
                        tracing::error!("UPLOAD: File too large ({} bytes)", data.len());
                        return (
                            StatusCode::PAYLOAD_TOO_LARGE,
                            Json(ErrorResponse {
                                error: "File too large (Max 10MB)".to_string(),
                            }),
                        )
                            .into_response();
                    }

                    // Create uploads directory if it doesn't exist
                    if !Path::new("uploads").exists() {
                        if let Err(e) = fs::create_dir("uploads").await {
                            tracing::error!("Failed to create uploads directory: {}", e);
                            return (
                                StatusCode::INTERNAL_SERVER_ERROR,
                                Json(ErrorResponse {
                                    error: "Server configuration error (Uploads dir)".to_string(),
                                }),
                            )
                                .into_response();
                        }
                    }

                    // 3. Sanitize filename (Uuid prefix + extension)
                    let safe_filename = format!("{}.{}", Uuid::new_v4(), extension);
                    let filepath = format!("uploads/{}", safe_filename);

                    match fs::write(&filepath, &data).await {
                        Ok(_) => {
                            tracing::info!("File uploaded successfully: {}", filepath);
                            return (
                                StatusCode::OK,
                                Json(serde_json::json!({
                                    "url": format!("/uploads/{}", safe_filename)
                                })),
                            )
                                .into_response();
                        }
                        Err(e) => {
                            tracing::error!("Failed to write file to disk: {}", e);
                            return (
                                StatusCode::INTERNAL_SERVER_ERROR,
                                Json(ErrorResponse {
                                    error: format!("Failed to save file: {}", e),
                                }),
                            )
                                .into_response();
                        }
                    }
                } else {
                    tracing::warn!("UPLOAD: Skipping unknown field '{}'", name);
                }
            }
            Ok(None) => {
                tracing::info!("UPLOAD: End of multipart stream, 'file' field not processed");
                break;
            }
            Err(e) => {
                tracing::error!("UPLOAD: Multipart error: {}", e);
                return (
                    StatusCode::BAD_REQUEST,
                    Json(ErrorResponse {
                        error: format!("Multipart structure error: {}", e),
                    }),
                )
                    .into_response();
            }
        }
    }

    tracing::error!("UPLOAD: Failed - No valid 'file' field processed");
    (
        StatusCode::BAD_REQUEST,
        Json(ErrorResponse {
            error: "File upload failed: No 'file' field found in request".to_string(),
        }),
    )
        .into_response()
}
