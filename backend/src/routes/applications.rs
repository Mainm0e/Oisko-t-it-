use crate::models::application::{Application, CreateApplication, UpdateApplication};
use crate::models::event::AppEvent;
use crate::routes::auth::Claims;
use async_stream::stream;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::{
        IntoResponse,
        sse::{Event, Sse},
    },
};
use futures_util::stream::Stream;
use sqlx::PgPool;
use std::convert::Infallible;
use tokio::sync::broadcast;
use uuid::Uuid;

pub async fn list_applications(State(pool): State<PgPool>, claims: Claims) -> impl IntoResponse {
    let user_id = Uuid::parse_str(&claims.sub).map_err(|_| StatusCode::UNAUTHORIZED);
    let user_id = match user_id {
        Ok(id) => id,
        Err(e) => return e.into_response(),
    };

    let applications = sqlx::query_as::<_, Application>(
        "SELECT a.*, (SELECT COUNT(*) FROM comments c WHERE c.application_id = a.id) as comment_count 
         FROM applications a WHERE user_id = $1 ORDER BY created_at DESC",
    )
    .bind(user_id)
    .fetch_all(&pool)
    .await;

    match applications {
        Ok(apps) => Json(apps).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response(),
    }
}

pub async fn create_application(
    State(pool): State<PgPool>,
    claims: Claims,
    Json(payload): Json<CreateApplication>,
) -> impl IntoResponse {
    let user_id = Uuid::parse_str(&claims.sub).map_err(|_| StatusCode::UNAUTHORIZED);
    let user_id = match user_id {
        Ok(id) => id,
        Err(e) => return e.into_response(),
    };

    let result = sqlx::query_as::<_, Application>(
        r#"
        INSERT INTO applications (
            user_id, company, company_website, role, status, salary, contact_person, 
            cv_version, cv_path, cover_letter, cover_letter_path, logo_url, description
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
        RETURNING *
        "#,
    )
    .bind(user_id)
    .bind(&payload.company)
    .bind(&payload.company_website)
    .bind(&payload.role)
    .bind(payload.status.unwrap_or_else(|| "Applied".to_string()))
    .bind(&payload.salary)
    .bind(&payload.contact_person)
    .bind(&payload.cv_version)
    .bind(&payload.cv_path)
    .bind(&payload.cover_letter)
    .bind(&payload.cover_letter_path)
    .bind(&payload.logo_url)
    .bind(&payload.description)
    .fetch_one(&pool)
    .await;

    match result {
        Ok(app) => (StatusCode::CREATED, Json(app)).into_response(),
        Err(e) => {
            tracing::error!("Failed to create application: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response()
        }
    }
}

pub async fn get_application(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    claims: Claims,
) -> impl IntoResponse {
    let user_id = Uuid::parse_str(&claims.sub).map_err(|_| StatusCode::UNAUTHORIZED);
    let user_id = match user_id {
        Ok(uid) => uid,
        Err(e) => return e.into_response(),
    };

    let application = sqlx::query_as::<_, Application>(
        "SELECT * FROM applications WHERE id = $1 AND user_id = $2",
    )
    .bind(id)
    .bind(user_id)
    .fetch_optional(&pool)
    .await;

    match application {
        Ok(Some(app)) => Json(app).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "Application not found").into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response(),
    }
}

pub async fn update_application(
    State(pool): State<PgPool>,
    State(tx): State<broadcast::Sender<AppEvent>>,
    Path(id): Path<Uuid>,
    claims: Claims,
    Json(payload): Json<UpdateApplication>,
) -> impl IntoResponse {
    let user_id = Uuid::parse_str(&claims.sub).map_err(|_| StatusCode::UNAUTHORIZED);
    let user_id = match user_id {
        Ok(uid) => uid,
        Err(e) => return e.into_response(),
    };

    // First check if it exists and belongs to user
    // (Optional optimization: do it in one query with UPDATE ... WHERE user_id = ...)
    // Let's do one query for atomic update

    // Construct dynamic query or just update all fields coalescing
    // For simplicity, let's fetch first to verify ownership (handling 404 vs 403 implicitly)
    // Actually, `UPDATE ... RETURNING *` returning nothing means not found/not owned.

    let result = sqlx::query_as::<_, Application>(
        r#"
        UPDATE applications
        SET 
            company = COALESCE($1, company),
            company_website = COALESCE($2, company_website),
            role = COALESCE($3, role),
            status = COALESCE($4, status),
            salary = COALESCE($5, salary),
            contact_person = COALESCE($6, contact_person),
            cv_version = COALESCE($7, cv_version),
            cv_path = COALESCE($8, cv_path),
            cover_letter = COALESCE($9, cover_letter),
            cover_letter_path = COALESCE($10, cover_letter_path),
            logo_url = COALESCE($11, logo_url),
            description = COALESCE($12, description),
            updated_at = NOW()
        WHERE id = $13 AND user_id = $14
        RETURNING *
        "#,
    )
    .bind(&payload.company)
    .bind(&payload.company_website)
    .bind(&payload.role)
    .bind(&payload.status)
    .bind(&payload.salary)
    .bind(&payload.contact_person)
    .bind(&payload.cv_version)
    .bind(&payload.cv_path)
    .bind(&payload.cover_letter)
    .bind(&payload.cover_letter_path)
    .bind(&payload.logo_url)
    .bind(&payload.description)
    .bind(id)
    .bind(user_id)
    .fetch_optional(&pool)
    .await;

    match result {
        Ok(Some(app)) => {
            // Broadcast update
            let _ = tx.send(AppEvent::ApplicationStatusUpdated {
                id: app.id,
                company: app.company.clone(),
                status: app.status.clone(),
            });
            Json(app).into_response()
        }
        Ok(None) => (StatusCode::NOT_FOUND, "Application not found").into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response(),
    }
}

pub async fn delete_application(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    claims: Claims,
) -> impl IntoResponse {
    let user_id = Uuid::parse_str(&claims.sub).map_err(|_| StatusCode::UNAUTHORIZED);
    let user_id = match user_id {
        Ok(uid) => uid,
        Err(e) => return e.into_response(),
    };

    let result = sqlx::query("DELETE FROM applications WHERE id = $1 AND user_id = $2")
        .bind(id)
        .bind(user_id)
        .execute(&pool)
        .await;

    match result {
        Ok(res) => {
            if res.rows_affected() > 0 {
                StatusCode::NO_CONTENT.into_response()
            } else {
                (StatusCode::NOT_FOUND, "Application not found").into_response()
            }
        }
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response(),
    }
}

pub async fn get_public_applications(State(pool): State<PgPool>) -> impl IntoResponse {
    use crate::models::public_application::PublicApplication;

    let applications = sqlx::query_as::<_, PublicApplication>(
        "SELECT id, company, company_website, role, status, logo_url, created_at FROM applications ORDER BY created_at DESC",
    )
    .fetch_all(&pool)
    .await;

    match applications {
        Ok(apps) => Json(apps).into_response(),
        Err(e) => {
            tracing::error!("Failed to fetch public applications: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response()
        }
    }
}

pub async fn get_public_application_detail(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    use crate::models::public_application::PublicApplicationDetail;

    let application = sqlx::query_as::<_, PublicApplicationDetail>(
        "SELECT id, company, company_website, role, status, salary, cover_letter, cv_path, logo_url, description, created_at FROM applications WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(&pool)
    .await;

    match application {
        Ok(Some(app)) => Json(app).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "Application not found").into_response(),
        Err(e) => {
            tracing::error!("Failed to fetch public application detail: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response()
        }
    }
}

pub async fn get_comments(
    State(pool): State<PgPool>,
    Path(application_id): Path<Uuid>,
) -> impl IntoResponse {
    use crate::models::comment::Comment;

    let comments = sqlx::query_as::<_, Comment>(
        "SELECT * FROM comments WHERE application_id = $1 ORDER BY created_at DESC",
    )
    .bind(application_id)
    .fetch_all(&pool)
    .await;

    match comments {
        Ok(comments) => Json(comments).into_response(),
        Err(e) => {
            tracing::error!("Failed to fetch comments: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response()
        }
    }
}

pub async fn create_comment(
    State(pool): State<PgPool>,
    State(tx): State<broadcast::Sender<AppEvent>>,
    Path(application_id): Path<Uuid>,
    Json(payload): Json<crate::models::comment::CreateComment>,
) -> impl IntoResponse {
    use crate::models::comment::Comment;

    // Get application info for the event
    let app_info = sqlx::query!(
        "SELECT company, role FROM applications WHERE id = $1",
        application_id
    )
    .fetch_optional(&pool)
    .await;

    let result = sqlx::query_as::<_, Comment>(
        "INSERT INTO comments (application_id, visitor_name, content) VALUES ($1, $2, $3) RETURNING *",
    )
    .bind(application_id)
    .bind(&payload.visitor_name)
    .bind(&payload.content)
    .fetch_one(&pool)
    .await;

    match result {
        Ok(comment) => {
            // Broadcast event
            if let Ok(Some(app)) = app_info {
                let _ = tx.send(AppEvent::CommentCreated {
                    id: comment.id,
                    application_id: comment.application_id,
                    visitor_name: comment.visitor_name.clone(),
                    company: app.company,
                    role: app.role,
                });
            }
            (StatusCode::CREATED, Json(comment)).into_response()
        }
        Err(e) => {
            tracing::error!("Failed to create comment: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response()
        }
    }
}

pub async fn sse_handler(
    State(tx): State<broadcast::Sender<AppEvent>>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    tracing::info!("DEBUG: SSE connection attempt received");
    let mut rx = tx.subscribe();

    let stream = stream! {
        loop {
            match rx.recv().await {
                Ok(event) => {
                    yield Ok(Event::default().json_data(event).unwrap());
                }
                Err(tokio::sync::broadcast::error::RecvError::Lagged(_)) => {
                    continue;
                }
                Err(tokio::sync::broadcast::error::RecvError::Closed) => {
                    break;
                }
            }
        }
    };

    Sse::new(stream).keep_alive(axum::response::sse::KeepAlive::default())
}

pub async fn get_recent_comments(State(pool): State<PgPool>, claims: Claims) -> impl IntoResponse {
    use crate::models::comment::CommentWithContext;

    let user_id = Uuid::parse_str(&claims.sub).map_err(|_| StatusCode::UNAUTHORIZED);
    let user_id = match user_id {
        Ok(id) => id,
        Err(e) => return e.into_response(),
    };

    let comments = sqlx::query_as::<_, CommentWithContext>(
        r#"
        SELECT c.*, a.company, a.role 
        FROM comments c 
        JOIN applications a ON c.application_id = a.id 
        WHERE a.user_id = $1 
        ORDER BY c.created_at DESC 
        LIMIT 10
        "#,
    )
    .bind(user_id)
    .fetch_all(&pool)
    .await;

    match comments {
        Ok(comments) => Json(comments).into_response(),
        Err(e) => {
            tracing::error!("Failed to fetch recent comments: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response()
        }
    }
}

pub async fn get_dashboard_stats(State(pool): State<PgPool>, claims: Claims) -> impl IntoResponse {
    use crate::models::application::{DailyCount, DashboardStats, StatusCount};

    let user_id = Uuid::parse_str(&claims.sub).map_err(|_| StatusCode::UNAUTHORIZED);
    let user_id = match user_id {
        Ok(id) => id,
        Err(e) => return e.into_response(),
    };

    // 1. Get daily activity for the last 30 days
    let daily_activity = sqlx::query_as::<_, DailyCount>(
        r#"
        SELECT 
            TO_CHAR(d, 'YYYY-MM-DD') as date,
            COUNT(a.id) as count
        FROM (
            SELECT (CURRENT_DATE - s.i)::DATE as d
            FROM generate_series(0, 29) as s(i)
        ) d
        LEFT JOIN applications a ON DATE(a.created_at) = d.d AND a.user_id = $1
        GROUP BY d.d
        ORDER BY d.d ASC
        "#,
    )
    .bind(user_id)
    .fetch_all(&pool)
    .await;

    // 2. Get status distribution
    let status_distribution = sqlx::query_as::<_, StatusCount>(
        r#"
        SELECT status, COUNT(*) as count 
        FROM applications 
        WHERE user_id = $1 
        GROUP BY status
        "#,
    )
    .bind(user_id)
    .fetch_all(&pool)
    .await;

    match (daily_activity, status_distribution) {
        (Ok(daily), Ok(status)) => Json(DashboardStats {
            daily_activity: daily,
            status_distribution: status,
        })
        .into_response(),
        (Err(e1), _) => {
            tracing::error!("Daily activity query failed: {:?}", e1);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch stats").into_response()
        }
        (_, Err(e2)) => {
            tracing::error!("Status distribution query failed: {:?}", e2);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch stats").into_response()
        }
    }
}
