use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct RawBlock {
    pub slot: i64,
    pub block_data: serde_json::Value,
    pub block_hash: String,
    pub parent_slot: Option<i64>,
    pub parent_hash: Option<String>,
    pub processed_at: DateTime<Utc>,
    pub processing_duration_ms: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct SlotCursor {
    pub id: i16,
    pub last_finalized_slot: i64,
    pub last_updated: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct DeadLetterQueueEntry {
    pub id: uuid::Uuid,
    pub slot: i64,
    pub error: String,
    pub retry_count: i32,
    pub last_retry: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub failed_at: DateTime<Utc>,
}

#[derive(Debug)]
pub struct CreateRawBlock {
    pub slot: i64,
    pub data: serde_json::Value,
    pub block_hash: String,
    pub parent_slot: Option<i64>,
    pub parent_hash: Option<String>,
    pub processing_duration_ms: Option<i32>,
}

#[derive(Debug)]
pub struct CreateDlqEntry {
    pub slot: i64,
    pub error: String,
}

#[derive(Debug, Clone)]
pub struct FetchRequest {
    pub slot: i64,
    pub retry_count: u32,
}

#[derive(Debug, Clone)]
pub struct BlockBatch {
    pub slots: Vec<i64>,
    pub blocks: Vec<RawBlock>,
}
