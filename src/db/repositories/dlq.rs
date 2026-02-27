use crate::db::DbPool;
use crate::error::{IndexerError, Result};
use crate::models::DeadLetterQueueEntry;
use sqlx::postgres::PgQueryResult;

#[derive(Clone)]
pub struct DlqRepository {
    pool: DbPool,
}

impl DlqRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, slot: i64, errror: &str) -> Result<PgQueryResult> {
        sqlx::query(
            r#"
            INSERT INTO dead_letter_queue (slot, error)
            VALUES ($1, $2)
            ON CONFLICT (slot) DO UPDATE
            SET error = EXCLUDED.error,
            retry_count = dead_letter_queue.retry_count + 1,
            last_retry = NOW(),
            failed_at = NOW()
            "#,
        )
        .bind(slot)
        .bind(errror)
        .execute(&self.pool)
        .await
        .map_err(IndexerError::Database)
    }

    pub async fn get_by_slot(&self, slot: i64) -> Result<Option<DeadLetterQueueEntry>> {
        sqlx::query_as::<_, DeadLetterQueueEntry>(
            r#"
            SELECT * FROM dead_letter_queue WHERE slot = $1
            "#,
        )
        .bind(slot)
        .fetch_optional(&self.pool)
        .await
        .map_err(IndexerError::Database)
    }

    pub async fn get_retryable(&self, max_retries: i32) -> Result<Vec<DeadLetterQueueEntry>> {
        sqlx::query_as::<_, DeadLetterQueueEntry>(
            r#"
            SELECT * FROM dead_letter_queue WHERE retry_count < $1 ORDER BY create_at ASC
            "#,
        )
        .bind(max_retries)
        .fetch_all(&self.pool)
        .await
        .map_err(IndexerError::Database)
    }

    pub async fn update(&self, entry: &DeadLetterQueueEntry) -> Result<PgQueryResult> {
        sqlx::query(
            r#"
            UPDATE dead_letter_queue 
            SET retry_count = $1, last_retry = $2, error = $3
            WHERE id = $4
            "#,
        )
        .bind(entry.retry_count)
        .bind(entry.last_retry)
        .bind(&entry.error)
        .bind(entry.id)
        .execute(&self.pool)
        .await
        .map_err(IndexerError::Database)
    }

    pub async fn delete_by_slot(&self, slot: i64) -> Result<PgQueryResult> {
        sqlx::query("DELETE FROM dead_letter_queue WHERE slot = $1")
            .bind(slot)
            .execute(&self.pool)
            .await
            .map_err(IndexerError::Database)
    }

    pub async fn count(&self) -> Result<i64> {
        let result = sqlx::query_scalar("SELECT COUNT(*) FROM dead_letter_queue")
            .fetch_one(&self.pool)
            .await
            .map_err(IndexerError::Database)?;

        Ok(result)
    }

    pub async fn cleanup_old_entries(&self, days: i32) -> Result<u64> {
        let result = sqlx::query(
            r#"
            DELETE FROM dead_letter_queue 
            WHERE created_at < NOW() - INTERVAL '1 day' * $1
            AND retry_count >= 5
            "#
        )
        .bind(days)
        .execute(&self.pool)
        .await
        .map_err(IndexerError::Database)?;
        
        Ok(result.rows_affected())
}
