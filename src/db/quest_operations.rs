// use chrono::{DateTime, Utc};
use serde_json::Value as JsonValue;
use sqlx::MySqlPool;

use super::quest_models::*;
use crate::utils::error::ApiError;

/// Insert or update a quest
pub async fn upsert_quest(pool: &MySqlPool, quest: &Quest) -> Result<(), ApiError> {
    sqlx::query(
        r#"
        INSERT INTO quests (
            id, config_version, starts_at, expires_at, application_id, application_name,
            application_link, share_policy, preview, primary_color, secondary_color,
            quest_name, game_title, game_publisher, cta_link, cta_button_label,
            task_join_operator, reward_assignment_method, rewards_expire_at
        )
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        ON DUPLICATE KEY UPDATE
            config_version = VALUES(config_version),
            starts_at = VALUES(starts_at),
            expires_at = VALUES(expires_at),
            application_id = VALUES(application_id),
            application_name = VALUES(application_name),
            application_link = VALUES(application_link),
            share_policy = VALUES(share_policy),
            preview = VALUES(preview),
            primary_color = VALUES(primary_color),
            secondary_color = VALUES(secondary_color),
            quest_name = VALUES(quest_name),
            game_title = VALUES(game_title),
            game_publisher = VALUES(game_publisher),
            cta_link = VALUES(cta_link),
            cta_button_label = VALUES(cta_button_label),
            task_join_operator = VALUES(task_join_operator),
            reward_assignment_method = VALUES(reward_assignment_method),
            rewards_expire_at = VALUES(rewards_expire_at),
            updated_at = CURRENT_TIMESTAMP
        "#,
    )
    .bind(&quest.id)
    .bind(quest.config_version)
    .bind(quest.starts_at)
    .bind(quest.expires_at)
    .bind(&quest.application_id)
    .bind(&quest.application_name)
    .bind(&quest.application_link)
    .bind(&quest.share_policy)
    .bind(quest.preview)
    .bind(&quest.primary_color)
    .bind(&quest.secondary_color)
    .bind(&quest.quest_name)
    .bind(&quest.game_title)
    .bind(&quest.game_publisher)
    .bind(&quest.cta_link)
    .bind(&quest.cta_button_label)
    .bind(&quest.task_join_operator)
    .bind(quest.reward_assignment_method)
    .bind(quest.rewards_expire_at)
    .execute(pool)
    .await
    .map_err(|e| ApiError::DatabaseError(e.to_string()))?;

    Ok(())
}

/// Insert or update quest assets
pub async fn upsert_quest_assets(
    pool: &MySqlPool,
    quest_id: &str,
    hero: Option<&str>,
    hero_video: Option<&str>,
    quest_bar_hero: Option<&str>,
    quest_bar_hero_video: Option<&str>,
    game_tile: Option<&str>,
    logotype: Option<&str>,
    game_tile_light: Option<&str>,
    game_tile_dark: Option<&str>,
    logotype_light: Option<&str>,
    logotype_dark: Option<&str>,
) -> Result<(), ApiError> {
    sqlx::query(
        r#"
        INSERT INTO quest_assets (
            quest_id, hero, hero_video, quest_bar_hero, quest_bar_hero_video,
            game_tile, logotype, game_tile_light, game_tile_dark,
            logotype_light, logotype_dark
        )
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        ON DUPLICATE KEY UPDATE
            hero = VALUES(hero),
            hero_video = VALUES(hero_video),
            quest_bar_hero = VALUES(quest_bar_hero),
            quest_bar_hero_video = VALUES(quest_bar_hero_video),
            game_tile = VALUES(game_tile),
            logotype = VALUES(logotype),
            game_tile_light = VALUES(game_tile_light),
            game_tile_dark = VALUES(game_tile_dark),
            logotype_light = VALUES(logotype_light),
            logotype_dark = VALUES(logotype_dark)
        "#,
    )
    .bind(quest_id)
    .bind(hero)
    .bind(hero_video)
    .bind(quest_bar_hero)
    .bind(quest_bar_hero_video)
    .bind(game_tile)
    .bind(logotype)
    .bind(game_tile_light)
    .bind(game_tile_dark)
    .bind(logotype_light)
    .bind(logotype_dark)
    .execute(pool)
    .await
    .map_err(|e| ApiError::DatabaseError(e.to_string()))?;

    Ok(())
}

/// Delete and re-insert quest tasks
pub async fn replace_quest_tasks(
    pool: &MySqlPool,
    quest_id: &str,
    tasks: &[(String, i32, Option<JsonValue>, Option<JsonValue>)],
) -> Result<(), ApiError> {
    // Delete existing tasks
    sqlx::query("DELETE FROM quest_tasks WHERE quest_id = ?")
        .bind(quest_id)
        .execute(pool)
        .await
        .map_err(|e| ApiError::DatabaseError(e.to_string()))?;

    // Insert new tasks
    for (task_type, target, applications, external_ids) in tasks {
        sqlx::query(
            r#"
            INSERT INTO quest_tasks (quest_id, task_type, target, applications, external_ids)
            VALUES (?, ?, ?, ?, ?)
            "#,
        )
        .bind(quest_id)
        .bind(task_type)
        .bind(target)
        .bind(applications)
        .bind(external_ids)
        .execute(pool)
        .await
        .map_err(|e| ApiError::DatabaseError(e.to_string()))?;
    }

    Ok(())
}

/// Delete and re-insert quest rewards
pub async fn replace_quest_rewards(
    pool: &MySqlPool,
    quest_id: &str,
    rewards: &[(
        i32,
        Option<String>,
        String,
        String,
        Option<i32>,
        Option<JsonValue>,
        i32,
    )],
) -> Result<(), ApiError> {
    // Delete existing rewards
    sqlx::query("DELETE FROM quest_rewards WHERE quest_id = ?")
        .bind(quest_id)
        .execute(pool)
        .await
        .map_err(|e| ApiError::DatabaseError(e.to_string()))?;

    // Insert new rewards
    for (
        reward_type,
        sku_id,
        reward_name,
        reward_name_with_article,
        orb_quantity,
        redemption_instructions,
        platform,
    ) in rewards
    {
        sqlx::query(
            r#"
            INSERT INTO quest_rewards (
                quest_id, reward_type, sku_id, reward_name, reward_name_with_article,
                orb_quantity, redemption_instructions, platform
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(quest_id)
        .bind(reward_type)
        .bind(sku_id)
        .bind(reward_name)
        .bind(reward_name_with_article)
        .bind(orb_quantity)
        .bind(redemption_instructions)
        .bind(platform)
        .execute(pool)
        .await
        .map_err(|e| ApiError::DatabaseError(e.to_string()))?;
    }

    Ok(())
}

/// Delete and re-insert quest features
pub async fn replace_quest_features(
    pool: &MySqlPool,
    quest_id: &str,
    features: &[i32],
) -> Result<(), ApiError> {
    // Delete existing features
    sqlx::query("DELETE FROM quest_features WHERE quest_id = ?")
        .bind(quest_id)
        .execute(pool)
        .await
        .map_err(|e| ApiError::DatabaseError(e.to_string()))?;

    // Insert new features
    for feature_id in features {
        sqlx::query("INSERT INTO quest_features (quest_id, feature_id) VALUES (?, ?)")
            .bind(quest_id)
            .bind(feature_id)
            .execute(pool)
            .await
            .map_err(|e| ApiError::DatabaseError(e.to_string()))?;
    }

    Ok(())
}

/// Get all quests with their related data
// pub async fn get_all_complete_quests(pool: &MySqlPool) -> Result<Vec<CompleteQuest>, ApiError> {
//     // Get all quests
//     let quests = sqlx::query_as::<_, Quest>("SELECT * FROM quests ORDER BY starts_at DESC")
//         .fetch_all(pool)
//         .await
//         .map_err(|e| ApiError::DatabaseError(e.to_string()))?;

//     let mut complete_quests = Vec::new();

//     for quest in quests {
//         let complete_quest = get_complete_quest_by_id(pool, &quest.id).await?;
//         complete_quests.push(complete_quest);
//     }

//     Ok(complete_quests)
// }

/// Get recent quests (within age_days) with their related data
pub async fn get_recent_complete_quests(
    pool: &MySqlPool,
    age_days: i64,
) -> Result<Vec<CompleteQuest>, ApiError> {
    // Get quests from the last N days
    let quests = sqlx::query_as::<_, Quest>(
        "SELECT * FROM quests 
         WHERE expires_at >= DATE_SUB(NOW(), INTERVAL ? DAY)
         ORDER BY starts_at DESC",
    )
    .bind(age_days)
    .fetch_all(pool)
    .await
    .map_err(|e| ApiError::DatabaseError(e.to_string()))?;

    let mut complete_quests = Vec::new();

    for quest in quests {
        let complete_quest = get_complete_quest_by_id(pool, &quest.id).await?;
        complete_quests.push(complete_quest);
    }

    Ok(complete_quests)
}

/// Get a complete quest by ID with all related data
async fn get_complete_quest_by_id(
    pool: &MySqlPool,
    quest_id: &str,
) -> Result<CompleteQuest, ApiError> {
    let quest = sqlx::query_as::<_, Quest>("SELECT * FROM quests WHERE id = ?")
        .bind(quest_id)
        .fetch_one(pool)
        .await
        .map_err(|e| ApiError::DatabaseError(e.to_string()))?;

    // Get assets
    let assets = sqlx::query_as::<_, QuestAssets>("SELECT * FROM quest_assets WHERE quest_id = ?")
        .bind(quest_id)
        .fetch_optional(pool)
        .await
        .map_err(|e| ApiError::DatabaseError(e.to_string()))?;

    // Get tasks
    let tasks = sqlx::query_as::<_, QuestTask>("SELECT * FROM quest_tasks WHERE quest_id = ?")
        .bind(quest_id)
        .fetch_all(pool)
        .await
        .map_err(|e| ApiError::DatabaseError(e.to_string()))?;

    // Get rewards
    let rewards =
        sqlx::query_as::<_, QuestReward>("SELECT * FROM quest_rewards WHERE quest_id = ?")
            .bind(quest_id)
            .fetch_all(pool)
            .await
            .map_err(|e| ApiError::DatabaseError(e.to_string()))?;

    // Get features
    let features =
        sqlx::query_as::<_, QuestFeature>("SELECT * FROM quest_features WHERE quest_id = ?")
            .bind(quest_id)
            .fetch_all(pool)
            .await
            .map_err(|e| ApiError::DatabaseError(e.to_string()))?;

    // Get user statuses
    let user_statuses =
        sqlx::query_as::<_, QuestUserStatus>("SELECT * FROM quest_user_status WHERE quest_id = ?")
            .bind(quest_id)
            .fetch_all(pool)
            .await
            .map_err(|e| ApiError::DatabaseError(e.to_string()))?;

    Ok(CompleteQuest {
        quest,
        assets,
        tasks,
        rewards,
        features,
        user_statuses,
    })
}

/// Get all existing quest IDs from database
pub async fn get_existing_quest_ids(pool: &MySqlPool) -> Result<Vec<String>, ApiError> {
    let ids: Vec<(String,)> = sqlx::query_as("SELECT id FROM quests")
        .fetch_all(pool)
        .await
        .map_err(|e| ApiError::DatabaseError(e.to_string()))?;

    Ok(ids.into_iter().map(|(id,)| id).collect())
}
