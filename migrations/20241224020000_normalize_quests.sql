-- Normalize Discord Quest Schema
-- Splits JSON quest data into proper relational tables

-- Main quests table
CREATE TABLE IF NOT EXISTS quests (
    id VARCHAR(255) PRIMARY KEY,
    config_version INT NOT NULL,
    starts_at TIMESTAMP NOT NULL,
    expires_at TIMESTAMP NOT NULL,
    application_id VARCHAR(255) NOT NULL,
    application_name VARCHAR(255) NOT NULL,
    application_link TEXT NOT NULL,
    share_policy VARCHAR(50) NOT NULL,
    preview BOOLEAN NOT NULL DEFAULT FALSE,
    primary_color VARCHAR(20),
    secondary_color VARCHAR(20),
    quest_name VARCHAR(255) NOT NULL,
    game_title VARCHAR(255) NOT NULL,
    game_publisher VARCHAR(255) NOT NULL,
    cta_link TEXT,
    cta_button_label VARCHAR(100),
    task_join_operator VARCHAR(10) NOT NULL DEFAULT 'or',
    reward_assignment_method INT NOT NULL DEFAULT 1,
    rewards_expire_at TIMESTAMP,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    
    INDEX idx_starts_at (starts_at),
    INDEX idx_expires_at (expires_at),
    INDEX idx_application_id (application_id),
    INDEX idx_updated_at (updated_at)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Quest assets (images, videos, logos)
CREATE TABLE IF NOT EXISTS quest_assets (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    quest_id VARCHAR(255) NOT NULL,
    hero TEXT,
    hero_video TEXT,
    quest_bar_hero TEXT,
    quest_bar_hero_video TEXT,
    game_tile TEXT,
    logotype TEXT,
    game_tile_light TEXT,
    game_tile_dark TEXT,
    logotype_light TEXT,
    logotype_dark TEXT,
    
    FOREIGN KEY (quest_id) REFERENCES quests(id) ON DELETE CASCADE,
    UNIQUE KEY unique_quest_assets (quest_id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Quest tasks (requirements like PLAY_ON_DESKTOP)
CREATE TABLE IF NOT EXISTS quest_tasks (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    quest_id VARCHAR(255) NOT NULL,
    task_type VARCHAR(100) NOT NULL,
    target INT NOT NULL,
    applications JSON,
    external_ids JSON,
    
    FOREIGN KEY (quest_id) REFERENCES quests(id) ON DELETE CASCADE,
    INDEX idx_task_type (task_type)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Quest rewards (orbs, items, etc.)
CREATE TABLE IF NOT EXISTS quest_rewards (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    quest_id VARCHAR(255) NOT NULL,
    reward_type INT NOT NULL,
    sku_id VARCHAR(255),
    reward_name VARCHAR(255) NOT NULL,
    reward_name_with_article VARCHAR(255) NOT NULL,
    orb_quantity INT,
    redemption_instructions JSON,
    platform INT NOT NULL DEFAULT 0,
    
    FOREIGN KEY (quest_id) REFERENCES quests(id) ON DELETE CASCADE,
    INDEX idx_reward_type (reward_type)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Quest features (feature flags)
CREATE TABLE IF NOT EXISTS quest_features (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    quest_id VARCHAR(255) NOT NULL,
    feature_id INT NOT NULL,
    
    FOREIGN KEY (quest_id) REFERENCES quests(id) ON DELETE CASCADE,
    INDEX idx_feature_id (feature_id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- User progress and status for each quest
CREATE TABLE IF NOT EXISTS quest_user_status (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    quest_id VARCHAR(255) NOT NULL,
    user_id VARCHAR(255) NOT NULL,
    enrolled_at TIMESTAMP NULL,
    completed_at TIMESTAMP NULL,
    claimed_at TIMESTAMP NULL,
    claimed_tier VARCHAR(50),
    stream_progress_seconds INT NOT NULL DEFAULT 0,
    dismissed_quest_content INT NOT NULL DEFAULT 0,
    progress_data JSON,
    last_updated TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    
    FOREIGN KEY (quest_id) REFERENCES quests(id) ON DELETE CASCADE,
    UNIQUE KEY unique_user_quest (quest_id, user_id),
    INDEX idx_user_id (user_id),
    INDEX idx_completed_at (completed_at)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Excluded quests tracking
CREATE TABLE IF NOT EXISTS excluded_quests (
    id VARCHAR(255) PRIMARY KEY,
    replacement_id VARCHAR(255),
    excluded_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    
    INDEX idx_replacement_id (replacement_id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Drop old quest table (backup first if needed)
DROP TABLE IF EXISTS quest;
