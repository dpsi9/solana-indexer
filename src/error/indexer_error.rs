use sqlx::Error as SqlxError;

#[derive(Debug, thiserror::Error)]
pub enum IndexerError {
    #[error("Database error: {0}")]
    Database(#[from] SqlxError),

    #[error("Rpc error: {0}")]
    Rpc(String),

    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serde(#[from] serde_json::Error),

    #[error("Network error: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("Slot gap detected: {0} - {1}")]
    SlotGap(u64, u64),

    #[error("Max retries exceeded for slot: {0}")]
    MaxRetriesExceeded(u64),

    #[error("Channel error: {0}")]
    Channel(String),

    #[error("Migration error: {0}")]
    Migration(String),
}

pub type Result<T> = std::result::Result<T, IndexerError>;
