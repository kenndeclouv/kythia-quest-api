use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub discord_token: String,
    pub database_url: String,
    pub port: u16,
    pub cache_duration_minutes: u64,
    pub quest_age_days: i64,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        dotenvy::dotenv().ok();

        let discord_token = env::var("DISCORD_TOKEN")
            .map_err(|_| anyhow::anyhow!("DISCORD_TOKEN must be set in environment"))?;

        let database_url = env::var("DATABASE_URL")
            .map_err(|_| anyhow::anyhow!("DATABASE_URL must be set in environment"))?;

        let port = env::var("PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse()
            .unwrap_or(3000);

        let cache_duration_minutes = env::var("CACHE_DURATION_MINUTES")
            .unwrap_or_else(|_| "30".to_string())
            .parse()
            .unwrap_or(30);

        let quest_age_days = env::var("QUEST_AGE_DAYS")
            .unwrap_or_else(|_| "30".to_string())
            .parse()
            .unwrap_or(30);

        if discord_token.trim().is_empty() {
            anyhow::bail!("DISCORD_TOKEN cannot be empty");
        }

        if database_url.trim().is_empty() {
            anyhow::bail!("DATABASE_URL cannot be empty");
        }

        Ok(Self {
            discord_token,
            database_url,
            port,
            cache_duration_minutes,
            quest_age_days,
        })
    }

    pub fn cache_duration_ms(&self) -> i64 {
        (self.cache_duration_minutes * 60 * 1000) as i64
    }
}
