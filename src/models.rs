use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawBlock {
    pub slot: u64,
    pub block_data: serde_json::Value,
    pub block_hash: String,
    pub parent_slot: Option<u64>,
    pub parent_hash: Option<String>,
    pub processed_at: DateTime<Utc>,
    pub processing_duration_ms: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlotCursor {
    pub id: i16,
    pub last_finalized_slot: u64,
    pub last_updated: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeadLetterQueueEntry {
    pub id: uuid::Uuid,
    pub slot: u64,
    pub error: String,
    pub retry_count: u32,
    pub last_retry: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub failed_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug)]
pub struct CreateRawBlock {
    pub slot: u64,
    pub data: serde_json::Value,
    pub block_hash: String,
    pub parent_slot: Option<u64>,
    pub parent_hash: Option<String>,
    pub processing_duration_ms: Option<i32>,
}

#[derive(Debug)]
pub struct CreateDlqEntry {
    pub slot: u64,
    pub error: String,
}

#[derive(Debug, Clone)]
pub struct FetchRequest {
    pub slot: u64,
    pub retry_count: u32,
}

#[derive(Debug, Clone)]
pub struct BlockBatch {
    pub slots: Vec<u64>,
    pub blocks: Vec<RawBlock>,
}
