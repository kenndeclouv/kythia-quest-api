use chrono::{DateTime, Utc};
use sqlx::MySqlPool;

use super::models::CacheStore;
use crate::utils::error::ApiError;

pub async fn get_cache(pool: &MySqlPool, key: &str) -> Result<Option<CacheStore>, ApiError> {
    let cache = sqlx::query_as::<_, CacheStore>(
        "SELECT id, data, updated_at FROM cache_store WHERE id = ?",
    )
    .bind(key)
    .fetch_optional(pool)
    .await
    .map_err(|e| ApiError::DatabaseError(e.to_string()))?;

    Ok(cache)
}

pub async fn upsert_cache(
    pool: &MySqlPool,
    key: &str,
    data: &serde_json::Value,
) -> Result<(), ApiError> {
    sqlx::query(
        r#"
        INSERT INTO cache_store (id, data, updated_at)
        VALUES (?, ?, UTC_TIMESTAMP())
        ON DUPLICATE KEY UPDATE
            data = VALUES(data),
            updated_at = UTC_TIMESTAMP()
        "#,
    )
    .bind(key)
    .bind(data)
    .execute(pool)
    .await
    .map_err(|e| ApiError::DatabaseError(e.to_string()))?;

    Ok(())
}

pub fn is_cache_stale(updated_at: DateTime<Utc>, duration_ms: i64) -> bool {
    let now = Utc::now();
    let time_diff = now.signed_duration_since(updated_at);
    time_diff.num_milliseconds() >= duration_ms
}
