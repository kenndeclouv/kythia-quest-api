use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde_json::{json, Value as JsonValue};
use sqlx::MySqlPool;

use crate::db::quest_models::*;
use crate::db::quest_operations::*;
use crate::utils::error::ApiError;

/// Discord API quest response structures (for parsing)
#[derive(Debug, Deserialize)]
pub struct DiscordQuestResponse {
    pub quests: Vec<DiscordQuest>,
}

#[derive(Debug, Deserialize)]
pub struct DiscordQuest {
    pub id: String,
    pub config: QuestConfig,
    pub preview: bool,
}

#[derive(Debug, Deserialize)]
pub struct QuestConfig {
    pub config_version: i32,
    pub starts_at: String,
    pub expires_at: String,
    pub features: Option<Vec<i32>>,
    pub application: Application,
    pub assets: Assets,
    pub colors: Colors,
    pub messages: Messages,
    pub task_config_v2: Option<TaskConfigV2>,
    pub rewards_config: RewardsConfig,
    pub share_policy: String,
    pub cta_config: Option<CtaConfig>,
}

#[derive(Debug, Deserialize)]
pub struct Application {
    pub id: String,
    pub name: String,
    pub link: String,
}

#[derive(Debug, Deserialize)]
pub struct Assets {
    pub hero: Option<String>,
    pub hero_video: Option<String>,
    pub quest_bar_hero: Option<String>,
    pub quest_bar_hero_video: Option<String>,
    pub game_tile: Option<String>,
    pub logotype: Option<String>,
    pub game_tile_light: Option<String>,
    pub game_tile_dark: Option<String>,
    pub logotype_light: Option<String>,
    pub logotype_dark: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Colors {
    pub primary: String,
    pub secondary: String,
}

#[derive(Debug, Deserialize)]
pub struct Messages {
    pub quest_name: String,
    pub game_title: String,
    pub game_publisher: String,
}

#[derive(Debug, Deserialize)]
pub struct TaskConfigV2 {
    pub tasks: serde_json::Map<String, JsonValue>,
    pub join_operator: String,
}

#[derive(Debug, Deserialize)]
pub struct RewardsConfig {
    pub assignment_method: i32,
    pub rewards: Vec<Reward>,
    pub rewards_expire_at: Option<String>,
    pub platforms: Vec<i32>,
}

#[derive(Debug, Deserialize)]
pub struct Reward {
    #[serde(rename = "type")]
    pub reward_type: i32,
    pub sku_id: Option<String>,
    pub messages: RewardMessages,
    pub orb_quantity: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct RewardMessages {
    pub name: String,
    pub name_with_article: String,
    pub redemption_instructions_by_platform: Option<serde_json::Map<String, JsonValue>>,
}

#[derive(Debug, Deserialize)]
pub struct CtaConfig {
    pub link: String,
    pub button_label: String,
}

// Parse and save ONLY NEW Discord quests to database (optimization)
pub async fn save_discord_quests_to_db(
    pool: &MySqlPool,
    response: &JsonValue,
) -> Result<usize, ApiError> {
    let quest_response: DiscordQuestResponse = serde_json::from_value(response.clone())
        .map_err(|e| ApiError::InternalError(format!("Failed to parse Discord response: {}", e)))?;

    // Get existing quest IDs from database
    let existing_ids: std::collections::HashSet<String> =
        crate::db::quest_operations::get_existing_quest_ids(pool)
            .await?
            .into_iter()
            .collect();

    // Filter to only new quests (not in database)
    let new_quests: Vec<_> = quest_response
        .quests
        .into_iter()
        .filter(|q| !existing_ids.contains(&q.id))
        .collect();

    let new_count = new_quests.len();

    // Only insert new quests
    for quest_data in new_quests {
        save_single_quest(pool, &quest_data).await?;
    }

    Ok(new_count)
}

/// Save a single quest with all related data
async fn save_single_quest(pool: &MySqlPool, quest_data: &DiscordQuest) -> Result<(), ApiError> {
    let config = &quest_data.config;

    // Parse timestamps
    let starts_at = parse_timestamp(&config.starts_at)?;
    let expires_at = parse_timestamp(&config.expires_at)?;
    let rewards_expire_at = config
        .rewards_config
        .rewards_expire_at
        .as_ref()
        .map(|s| parse_timestamp(s))
        .transpose()?;

    // Create Quest struct
    let quest = Quest {
        id: quest_data.id.clone(),
        config_version: config.config_version,
        starts_at,
        expires_at,
        application_id: config.application.id.clone(),
        application_name: config.application.name.clone(),
        application_link: config.application.link.clone(),
        share_policy: config.share_policy.clone(),
        preview: quest_data.preview,
        primary_color: Some(config.colors.primary.clone()),
        secondary_color: Some(config.colors.secondary.clone()),
        quest_name: config.messages.quest_name.clone(),
        game_title: config.messages.game_title.clone(),
        game_publisher: config.messages.game_publisher.clone(),
        cta_link: config.cta_config.as_ref().map(|c| c.link.clone()),
        cta_button_label: config.cta_config.as_ref().map(|c| c.button_label.clone()),
        task_join_operator: config
            .task_config_v2
            .as_ref()
            .map(|t| t.join_operator.clone())
            .unwrap_or_else(|| "or".to_string()),
        reward_assignment_method: config.rewards_config.assignment_method,
        rewards_expire_at,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    // Save main quest
    upsert_quest(pool, &quest).await?;

    // Save assets
    let assets = &config.assets;
    upsert_quest_assets(
        pool,
        &quest_data.id,
        assets.hero.as_deref(),
        assets.hero_video.as_deref(),
        assets.quest_bar_hero.as_deref(),
        assets.quest_bar_hero_video.as_deref(),
        assets.game_tile.as_deref(),
        assets.logotype.as_deref(),
        assets.game_tile_light.as_deref(),
        assets.game_tile_dark.as_deref(),
        assets.logotype_light.as_deref(),
        assets.logotype_dark.as_deref(),
    )
    .await?;

    // Save tasks
    if let Some(task_config) = &config.task_config_v2 {
        let mut tasks = Vec::new();
        for (task_type, task_data) in &task_config.tasks {
            let target = task_data["target"].as_i64().unwrap_or(0) as i32;
            let applications = task_data.get("applications").cloned();
            let external_ids = task_data.get("external_ids").cloned();
            tasks.push((task_type.clone(), target, applications, external_ids));
        }
        replace_quest_tasks(pool, &quest_data.id, &tasks).await?;
    }

    // Save rewards
    let mut rewards = Vec::new();
    for reward in &config.rewards_config.rewards {
        let platform = config
            .rewards_config
            .platforms
            .first()
            .copied()
            .unwrap_or(0);
        let redemption_instructions = reward
            .messages
            .redemption_instructions_by_platform
            .as_ref()
            .map(|m| json!(m));

        rewards.push((
            reward.reward_type,
            reward.sku_id.clone(),
            reward.messages.name.clone(),
            reward.messages.name_with_article.clone(),
            reward.orb_quantity,
            redemption_instructions,
            platform,
        ));
    }
    replace_quest_rewards(pool, &quest_data.id, &rewards).await?;

    // Save features
    if let Some(features) = &config.features {
        replace_quest_features(pool, &quest_data.id, features).await?;
    }

    // Skip user status - we only track quest configuration, not user progress

    Ok(())
}

/// Parse timestamp string to DateTime<Utc>
fn parse_timestamp(s: &str) -> Result<DateTime<Utc>, ApiError> {
    DateTime::parse_from_rfc3339(s)
        .map(|dt| dt.with_timezone(&Utc))
        .map_err(|e| ApiError::InternalError(format!("Failed to parse timestamp: {}", e)))
}

/// Reconstruct Discord API format from database (with age filter)
pub async fn reconstruct_discord_response(
    pool: &MySqlPool,
    age_days: i64,
) -> Result<JsonValue, ApiError> {
    let complete_quests =
        crate::db::quest_operations::get_recent_complete_quests(pool, age_days).await?;

    let mut quests_json = Vec::new();

    for cq in complete_quests {
        let quest_json = reconstruct_single_quest(&cq);
        quests_json.push(quest_json);
    }

    Ok(json!({
        "quests": quests_json,
    }))
}

/// Reconstruct a single quest in Discord format
fn reconstruct_single_quest(cq: &CompleteQuest) -> JsonValue {
    let q = &cq.quest;
    let assets = cq.assets.as_ref();

    // Reconstruct tasks
    let mut tasks_map = serde_json::Map::new();
    for task in &cq.tasks {
        let task_obj = json!({
            "type": task.task_type,
            "target": task.target,
            "applications": task.applications,
            "external_ids": task.external_ids
        });
        tasks_map.insert(task.task_type.clone(), task_obj);
    }

    // Reconstruct rewards
    let rewards_json: Vec<JsonValue> = cq
        .rewards
        .iter()
        .map(|r| {
            json!({
                "type": r.reward_type,
                "sku_id": r.sku_id,
                "messages": {
                    "name": r.reward_name,
                    "name_with_article": r.reward_name_with_article,
                    "redemption_instructions_by_platform": r.redemption_instructions
                },
                "orb_quantity": r.orb_quantity
            })
        })
        .collect();

    // Reconstruct features
    let features: Vec<i32> = cq.features.iter().map(|f| f.feature_id).collect();

    // No user status - we only track quest configuration

    json!({
        "id": q.id,
        "config": {
            "id": q.id,
            "config_version": q.config_version,
            "starts_at": q.starts_at,
            "expires_at": q.expires_at,
            "features": features,
            "application": {
                "id": q.application_id,
                "name": q.application_name,
                "link": q.application_link
            },
            "assets": {
                "hero": assets.and_then(|a| a.hero.clone()),
                "hero_video": assets.and_then(|a| a.hero_video.clone()),
                "quest_bar_hero": assets.and_then(|a| a.quest_bar_hero.clone()),
                "quest_bar_hero_video": assets.and_then(|a| a.quest_bar_hero_video.clone()),
                "game_tile": assets.and_then(|a| a.game_tile.clone()),
                "logotype": assets.and_then(|a| a.logotype.clone()),
                "game_tile_light": assets.and_then(|a| a.game_tile_light.clone()),
                "game_tile_dark": assets.and_then(|a| a.game_tile_dark.clone()),
                "logotype_light": assets.and_then(|a| a.logotype_light.clone()),
                "logotype_dark": assets.and_then(|a| a.logotype_dark.clone())
            },
            "colors": {
                "primary": q.primary_color,
                "secondary": q.secondary_color
            },
            "messages": {
                "quest_name": q.quest_name,
                "game_title": q.game_title,
                "game_publisher": q.game_publisher
            },
            "task_config_v2": {
                "tasks": tasks_map,
                "join_operator": q.task_join_operator
            },
            "rewards_config": {
                "assignment_method": q.reward_assignment_method,
                "rewards": rewards_json,
                "rewards_expire_at": q.rewards_expire_at,
                "platforms": [0]
            },
            "share_policy": q.share_policy,
            "cta_config": if q.cta_link.is_some() {
                Some(json!({
                    "link": q.cta_link,
                    "button_label": q.cta_button_label
                }))
            } else {
                None
            }
        },
        "user_status": null,
        "targeted_content": [],
        "preview": q.preview
    })
}
