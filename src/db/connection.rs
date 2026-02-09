use crate::config::DatabaseConfig;
use crate::error::{IndexerError, Result};
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use std::time::Duration;

pub type DbPool = Pool<Postgres>;

pub async fn create_pool(config: &DatabaseConfig) -> Result<DbPool> {
    let database_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        config.username, config.password, config.host, config.port, config.database
    );

    tracing::info!("Connecting to database at: {}", config.host);

    let pool = PgPoolOptions::new()
        .max_connections(config.max_connections)
        .min_connections(config.min_connections)
        .acquire_timeout(Duration::from_secs(config.acquire_timout_secs))
        .idle_timeout(Duration::from_secs(config.idle_timeout_secs))
        .max_lifetime(Duration::from_secs(config.max_lifetime_secs))
        .connect(&database_url)
        .await
        .map_err(|e| {
            tracing::error!("failed to connect to database: {}", e);
            IndexerError::Database(e)
        })?;

    // test the connection
    sqlx::query("SELECT 1").execute(&pool).await.map_err(|e| {
        tracing::error!("failed to connect to database: {}", e);
        IndexerError::Database(e)
    })?;

    tracing::info!("Database connection established successfully");

    Ok(pool)
}

pub async fn run_migrations(pool: &DbPool) -> Result<()> {
    tracing::info!("Running migrations");

    sqlx::migrate!("./migrations")
        .run(pool)
        .await
        .map_err(|e| {
            tracing::error!("Migration failed: {}", e);
            IndexerError::Database(e.into())
        })?;

    tracing::info!("Database migrations completed successfull");

    Ok(())
}
