use sqlx::PgPool;

// We will use the pool directly in handlers, but we can add helper functions here.
pub type DbPool = PgPool;
