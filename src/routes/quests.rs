use axum::{extract::State, routing::get, Json, Router};
use serde_json::Value;

use crate::{
    db::operations::{get_cache, is_cache_stale, upsert_cache},
    utils::{
        discord::fetch_discord_quests,
        error::ApiError,
        quest_parser::{reconstruct_discord_response, save_discord_quests_to_db},
    },
    AppState,
};

const CACHE_KEY: &str = "discord_quests";

pub fn router() -> Router<AppState> {
    Router::new().route("/", get(get_quests))
}

async fn get_quests(State(state): State<AppState>) -> Result<Json<Value>, ApiError> {
    // Check cache for complete response
    let cached_data = get_cache(&state.db, CACHE_KEY).await?;

    if let Some(cache) = cached_data {
        let is_stale = is_cache_stale(cache.updated_at, state.config.cache_duration_ms());

        if !is_stale {
            tracing::debug!("🎯 Cache hit for {}", CACHE_KEY);
            return Ok(Json(cache.data));
        } else {
            tracing::debug!("⏰ Cache stale for {}", CACHE_KEY);
        }
    } else {
        tracing::debug!("❌ Cache miss for {}", CACHE_KEY);
    }

    // Fetch fresh data from Discord API
    tracing::info!("📡 Fetching quests from Discord API");
    let quests_data = fetch_discord_quests(&state.config.discord_token).await?;

    // Save ONLY NEW quests to database (optimization)
    tracing::info!("💾 Checking for new quests...");
    let new_count = save_discord_quests_to_db(&state.db, &quests_data).await?;

    if new_count > 0 {
        tracing::info!("✅ Inserted {} new quest(s) to database", new_count);
    } else {
        tracing::info!("✅ No new quests - database is up to date");
    }

    // Reconstruct response from database
    tracing::info!("🔄 Reconstructing response from database");
    let reconstructed =
        reconstruct_discord_response(&state.db, state.config.quest_age_days).await?;

    // Update cache with reconstructed response
    upsert_cache(&state.db, CACHE_KEY, &reconstructed).await?;
    tracing::info!("✅ Cache updated for {}", CACHE_KEY);

    Ok(Json(reconstructed))
}
