use crate::db::DbPool;
use crate::error::{IndexerError, Result};
use crate::models::{CreateRawBlock, RawBlock};
use sqlx::postgres::PgQueryResult;

#[derive(Clone)]
pub struct RawBlockRepository {
    pub pool: DbPool,
}

impl RawBlockRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    pub async fn insert(&self, block: &CreateRawBlock) -> Result<PgQueryResult> {
        sqlx::query(
            r#"
            INSERT INTO raw_blocks (slot, block_data, block_hash, parent_slot, parent_hash, processing_duration_ms)
            VALUES ($1, $2, $3, $4, $5, $6)
            ON CONFLICT (slot) DO NOTHING
            "#
        ).bind(block.slot )
        .bind(&block.data)
        .bind(&block.block_hash)
        .bind(block.parent_slot )
        .bind(block.parent_hash.as_ref())
        .bind(block.processing_duration_ms)
        .execute(&self.pool).await.map_err(IndexerError::Database)
    }

    pub async fn get_by_slot(&self, slot: i64) -> Result<Option<RawBlock>> {
        sqlx::query_as::<_, RawBlock>("SELECT * FROM raw_blocks WHERE slot = $1")
            .bind(slot as i64)
            .fetch_optional(&self.pool)
            .await
            .map_err(IndexerError::Database)
    }

    pub async fn get_latest_block(&self) -> Result<Option<RawBlock>> {
        sqlx::query_as::<_, RawBlock>("SELECT * FROM raw_blocks ORDER BY slot DESC LIMIT 1")
            .fetch_optional(&self.pool)
            .await
            .map_err(IndexerError::Database)
    }

    pub async fn get_blocks_in_range(&self, start_slot: u64, end_slot: u64) -> Result<Vec<RawBlock>> {
        sqlx::query_as::<_, RawBlock>(
            "SELECT * FROM raw_blocks WHERE slot BETWEEN $1 AND $2 ORDER BY slot ASC"
        )
        .bind(start_slot as i64)
        .bind(end_slot as i64)
        .fetch_all(&self.pool)
        .await
        .map_err(IndexerError::Database)
    }
    
    pub async fn exists(&self, slot: u64) -> Result<bool> {
        let result: Option<(bool,)> = sqlx::query_as(
            "SELECT EXISTS(SELECT 1 FROM raw_blocks WHERE slot = $1)"
        )
        .bind(slot as i64)
        .fetch_optional(&self.pool)
        .await
        .map_err(IndexerError::Database)?;
        
        Ok(result.map(|(exists,)| exists).unwrap_or(false))
    }
    
    pub async fn count(&self) -> Result<i64> {
        let result: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM raw_blocks"
        )
        .fetch_one(&self.pool)
        .await
        .map_err(IndexerError::Database)?;
        
        Ok(result.0)
    }
}
