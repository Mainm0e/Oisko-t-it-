use crate::models::event::AppEvent;
use axum::{
    extract::FromRef,
    routing::{get, post},
    Router,
};
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::net::SocketAddr;
use tokio::sync::broadcast;
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod db;
mod models;
mod routes; // Added for Company Intel

#[derive(Clone)]
pub struct AppState {
    pub pool: sqlx::PgPool,
    pub tx: broadcast::Sender<AppEvent>,
}

impl FromRef<AppState> for sqlx::PgPool {
    fn from_ref(state: &AppState) -> Self {
        state.pool.clone()
    }
}

impl FromRef<AppState> for broadcast::Sender<AppEvent> {
    fn from_ref(state: &AppState) -> Self {
        state.tx.clone()
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to Postgres");

    tracing::info!("Connected to database");

    // Run migrations
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    let (tx, _rx) = broadcast::channel(100);
    let state = AppState { pool, tx };

    let _ = env::var("JWT_SECRET").expect("JWT_SECRET must be set for signal encryption");

    use axum::extract::DefaultBodyLimit;

    let app = Router::new()
        .route("/api/auth/login", post(routes::auth::login))
        .route("/api/auth/register", post(routes::auth::register))
        .route("/api/auth/verify", post(routes::auth::verify_email))
        .route("/api/visit", post(routes::visitor::record_visit))
        .route(
            "/api/applications",
            post(routes::applications::create_application)
                .get(routes::applications::list_applications),
        )
        .route(
            "/api/applications/stats",
            get(routes::applications::get_dashboard_stats),
        )
        .route(
            "/api/applications/:id",
            get(routes::applications::get_application)
                .put(routes::applications::update_application)
                .delete(routes::applications::delete_application),
        )
        .route(
            "/api/public/applications",
            get(routes::applications::get_public_applications),
        )
        .route(
            "/api/public/applications/:id",
            get(routes::applications::get_public_application_detail),
        )
        .route(
            "/api/public/applications/:id/comments",
            get(routes::applications::get_comments).post(routes::applications::create_comment),
        )
        .route(
            "/api/comments/recent",
            get(routes::applications::get_recent_comments),
        )
        .route("/api/intel", get(routes::intel::get_company_intel))
        .route("/api/events", get(routes::applications::sse_handler))
        .route("/api/upload", post(routes::upload::upload_file))
        .nest_service("/uploads", ServeDir::new("uploads"))
        .layer(TraceLayer::new_for_http())
        .layer(DefaultBodyLimit::max(10 * 1024 * 1024)) // Increase limit to 10MB
        .layer(
            CorsLayer::new()
                .allow_origin(tower_http::cors::Any) // Replace with specific domains in production
                .allow_methods([
                    axum::http::Method::GET,
                    axum::http::Method::POST,
                    axum::http::Method::PUT,
                    axum::http::Method::DELETE,
                ])
                .allow_headers([
                    axum::http::header::AUTHORIZATION,
                    axum::http::header::CONTENT_TYPE,
                ]),
        )
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}
