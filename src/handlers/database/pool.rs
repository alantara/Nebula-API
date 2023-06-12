use sqlx::postgres::{PgPoolOptions, PgPool};
use sqlx::Error;

pub async fn create() -> Result<PgPool, Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://superuser:12345678@localhost/nebula").await;

    match pool{
        Ok(v) => { Ok(v) }
        Err(v) => { Err(v) }
    }
}