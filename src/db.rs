use std::env;
use anyhow;
use tracing::*;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres };

/// Initializes the database connection pool and runs migrations.
pub async fn init_db() -> anyhow::Result<Pool<Postgres>>{
    let db_url = env::var("DATABASE_URL").expect("DB_URL must be set");
    let pool = PgPoolOptions::new().max_connections(5).connect(&db_url).await?;
    sqlx::migrate!("./migrations").run(&pool).await?;
     info!("✅ Database connected successfully");
    Ok(pool)
}