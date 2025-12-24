
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// Main quest table
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Quest {
    pub id: String,
    pub config_version: i32,
    pub starts_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub application_id: String,
    pub application_name: String,
    pub application_link: String,
    pub share_policy: String,
    pub preview: bool,
    pub primary_color: Option<String>,
    pub secondary_color: Option<String>,
    pub quest_name: String,
    pub game_title: String,
    pub game_publisher: String,
    pub cta_link: Option<String>,
    pub cta_button_label: Option<String>,
    pub task_join_operator: String,
    pub reward_assignment_method: i32,
    pub rewards_expire_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Quest assets (images, videos, logos)
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct QuestAssets {
    pub id: i64,
    pub quest_id: String,
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

/// Quest task requirements
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct QuestTask {
    pub id: i64,
    pub quest_id: String,
    pub task_type: String,
    pub target: i32,
    pub applications: Option<serde_json::Value>,
    pub external_ids: Option<serde_json::Value>,
}

/// Quest rewards
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct QuestReward {
    pub id: i64,
    pub quest_id: String,
    pub reward_type: i32,
    pub sku_id: Option<String>,
    pub reward_name: String,
    pub reward_name_with_article: String,
    pub orb_quantity: Option<i32>,
    pub redemption_instructions: Option<serde_json::Value>,
    pub platform: i32,
}

/// Quest feature flags
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct QuestFeature {
    pub id: i64,
    pub quest_id: String,
    pub feature_id: i32,
}

/// User quest progress and status
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct QuestUserStatus {
    pub id: i64,
    pub quest_id: String,
    pub user_id: String,
    pub enrolled_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub claimed_at: Option<DateTime<Utc>>,
    pub claimed_tier: Option<String>,
    pub stream_progress_seconds: i32,
    pub dismissed_quest_content: i32,
    pub progress_data: Option<serde_json::Value>,
    pub last_updated: DateTime<Utc>,
}

/// Complete quest data (for reconstruction)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompleteQuest {
    pub quest: Quest,
    pub assets: Option<QuestAssets>,
    pub tasks: Vec<QuestTask>,
    pub rewards: Vec<QuestReward>,
    pub features: Vec<QuestFeature>,
    pub user_statuses: Vec<QuestUserStatus>,
}
