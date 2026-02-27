use config::{Config, Environment, File};
use serde::Deserialize;
use std::env;

use crate::error::indexer_error;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub rpc: RpcConfig,
    pub database: DatabaseConfig,
    pub ingestion: IngestionConfig,
    pub logging: LoggingConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RpcConfig {
    pub endpoint: String,
    pub ws_endpoint: Option<String>,
    pub commitment: String,
    pub max_retries: u32,
    pub timeout_secs: u64,
    pub retry_delay_ms: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub database: String,
    pub username: String,
    pub password: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub acquire_timout_secs: u64, // how long the app will wait to get db connection, before giving up
    pub idle_timeout_secs: u64,   // how long a connection is allowed to sit unused
    pub max_lifetime_secs: u64,   // max lifetime of any connection
}

#[derive(Debug, Deserialize, Clone)]
pub struct IngestionConfig {
    pub batch_size: usize, // how many slots/blocks/tx/signatures to fetch per ingestion batch
    pub max_concurrent_fetches: usize, // max no of rpc requests at same time (parallel)
    pub gap_check_interval_secs: u64, // how often indexer needs to check for gaps or missing slots
    pub dlq_retry_interval_secs: u64, // time to retry items in dlq
    pub start_slot: Option<u64>, // if Some(n) -> start from n slot, if none -> start from last_indexed or latest finalized slot
    pub stop_on_gap: bool, // what to do when gap found(stop & retry or continue, let dlq handle)
    pub enable_dlq: bool,  // enable dlq or failures are dropped or just logged
    pub dlq_max_retries: u32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct LoggingConfig {
    pub level: String,
    pub format: String,
    pub enable_file_logging: bool,
    pub log_file_booth: Option<String>,
}

impl AppConfig {
    pub fn load() -> indexer_error::Result<Self> {
        dotenvy::dotenv().ok();

        let config_builder = Config::builder()
            // default config for rpc
            .set_default("rpc.endpoint", "https://api.mainnet-beta.solana.com")?
            .set_default("rpc.commitment", "confirmed")?
            .set_default("rpc.timeout_secs", 30)?
            .set_default("rpc.max_retries", 5)?
            .set_default("rpc.retry_delay_ms", 1000)?
            // default config for database
            .set_default("database.host", "localhost")?
            .set_default("database.port", 5432)?
            .set_default("database.database", "solana_indexer")?
            .set_default("database.username", "postgres")?
            .set_default("database.password", "postgres")?
            .set_default("database.max_connections", 10)?
            .set_default("database.min_connections", 2)?
            .set_default("database.acquire_timeout_secs", 30)?
            .set_default("database.idle_timeout_secs", 600)?
            .set_default("database.max_lifetime_secs", 1800)?
            // default config for ingestion
            .set_default("ingestion.batch_size", 10)?
            .set_default("ingestion.max_concurrent_fetches", 5)?
            .set_default("ingestion.gap_check_interval_secs", 60)?
            .set_default("ingestion.dlq_retry_interval_secs", 300)?
            .set_default("ingestion.stop_on_gap", true)?
            .set_default("ingestion.enable_dlq", true)?
            .set_default("ingestion.dlq_max_retries", 5)?
            // default config for logging
            .set_default("logging.level", "info")?
            .set_default("logging.format", "pretty")?
            .set_default("logging.enable_file_logging", false)?
            // load from config file if it exists
            .add_source(File::with_name("config").required(false))
            // or override with environment variables
            .add_source(Environment::with_prefix("SOLANA_INDEXER").separator("__"))
            .build()
            .unwrap();

        config_builder
            .try_deserialize()
            .map_err(|e| indexer_error::IndexerError::Config(e))
    }

    pub fn database_url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.database.username,
            self.database.password,
            self.database.host,
            self.database.port,
            self.database.database
        )
    }
}
