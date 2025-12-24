use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CacheStore {
    pub id: String,
    pub data: serde_json::Value,
    pub updated_at: DateTime<Utc>,
}

