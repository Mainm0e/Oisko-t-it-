use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Deserialize)]
pub struct ContactPayload {
    name: String,
    email: String,
    message: String,
    link: Option<String>,
    // Honeypot field - should be empty for humans
    bot_field: Option<String>,
}

#[derive(Serialize)]
pub struct ContactResponse {
    message: String,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    error: String,
}

pub async fn send_contact_email(Json(payload): Json<ContactPayload>) -> impl IntoResponse {
    // 1. Bot Detection (Honeypot)
    if let Some(val) = &payload.bot_field {
        if !val.is_empty() {
            // It's a bot. Return success to fool them, but do nothing.
            return (
                StatusCode::OK,
                Json(ContactResponse {
                    message: "Message sent successfully".to_string(),
                }),
            )
                .into_response();
        }
    }
    let api_key = match env::var("RESEND_API_KEY") {
        Ok(key) => key,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "Email service not configured".to_string(),
                }),
            )
                .into_response();
        }
    };

    let sender = env::var("SENDER_EMAIL").unwrap_or_else(|_| "onboarding@resend.dev".to_string());
    // In a real app, you might want to send this TO yourself, FROM the system.
    // The "reply-to" header would be the user's email.
    // For Resend 'onboarding' you can only send to yourself potentially or verified domains.
    // Let's assume SENDER_EMAIL is a verified sender.
    // We want to receive the email. So 'to' should be the owner's email.
    // Let's use SENDER_EMAIL as the recipient too for simplicity or an ADMIN_EMAIL if it existed.
    // For now, I'll send it back to the SENDER_EMAIL (acting as owner) or maybe we assume the user puts their own email in ContactPayload?
    // Wait, the form is "Contact Me". So the user (payload.email) sends a message to ME (owner).
    // Resend requires 'from' to be a verified domain.

    let owner_email = env::var("OWNER_EMAIL").unwrap_or_else(|_| sender.clone());

    let link_section = if let Some(link) = &payload.link {
        if !link.is_empty() {
            format!(
                "<p><strong>Job Link:</strong> <a href=\"{}\">{}</a></p>",
                link, link
            )
        } else {
            String::new()
        }
    } else {
        String::new()
    };

    let email_body = format!(
        r#"
        <div style="font-family: sans-serif; max-width: 600px; margin: 0 auto;">
            <h2>New Contact Request</h2>
            <p><strong>Name:</strong> {}</p>
            <p><strong>Email:</strong> {}</p>
            {}
            <p><strong>Message:</strong></p>
            <blockquote style="background: #f9f9f9; padding: 10px; border-left: 5px solid #ccc;">
                {}
            </blockquote>
        </div>
        "#,
        payload.name, payload.email, link_section, payload.message
    );

    let resend_payload = serde_json::json!({
        "from": sender,
        "to": owner_email,
        "reply_to": payload.email,
        "subject": format!("Contact Form: {} reached out", payload.name),
        "html": email_body
    });

    let client = reqwest::Client::new();

    match client
        .post("https://api.resend.com/emails")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&resend_payload)
        .send()
        .await
    {
        Ok(res) => {
            if res.status().is_success() {
                (
                    StatusCode::OK,
                    Json(ContactResponse {
                        message: "Message sent successfully".to_string(),
                    }),
                )
                    .into_response()
            } else {
                let status = res.status();
                let text = res.text().await.unwrap_or_default();
                tracing::error!("Resend API Error: Status: {}, Body: {}", status, text);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorResponse {
                        error: "Failed to send email".to_string(),
                    }),
                )
                    .into_response()
            }
        }
        Err(e) => {
            tracing::error!("Failed to execute Resend request: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "Failed to connect to email service".to_string(),
                }),
            )
                .into_response()
        }
    }
}
