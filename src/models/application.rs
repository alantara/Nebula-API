use sqlx::postgres::PgPool;

pub struct AppState {
    pub pg_pool: PgPool,
}
