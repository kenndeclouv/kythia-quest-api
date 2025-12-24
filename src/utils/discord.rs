use base64::{engine::general_purpose, Engine};
use reqwest::header::{
    HeaderMap, HeaderValue, ACCEPT, ACCEPT_LANGUAGE, AUTHORIZATION, CONTENT_TYPE, USER_AGENT,
};
use serde_json::{json, Value};

use super::error::ApiError;

const DISCORD_API_URL: &str = "https://discord.com/api/v10/quests/@me";

pub async fn fetch_discord_quests(token: &str) -> Result<Value, ApiError> {
    let headers = generate_headers(token);

    let client = reqwest::Client::new();
    let response = client
        .get(DISCORD_API_URL)
        .headers(headers)
        .send()
        .await
        .map_err(|e| ApiError::DiscordApiError(format!("Request failed: {}", e)))?;

    if !response.status().is_success() {
        let status = response.status();
        let error_text = response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".to_string());
        return Err(ApiError::DiscordApiError(format!(
            "Discord API returned {}: {}",
            status, error_text
        )));
    }

    let data: Value = response
        .json()
        .await
        .map_err(|e| ApiError::DiscordApiError(format!("Failed to parse response: {}", e)))?;

    // Return the full response object (includes quests and excluded_quests)
    Ok(data)
}

fn generate_headers(token: &str) -> HeaderMap {
    let mut headers = HeaderMap::new();

    headers.insert(ACCEPT, HeaderValue::from_static("*/*"));

    headers.insert(
        ACCEPT_LANGUAGE,
        HeaderValue::from_static("en,en-US;q=0.9,ar;q=0.8"),
    );

    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(token.trim()).expect("Invalid token"),
    );

    headers.insert("priority", HeaderValue::from_static("u=1, i"));

    headers.insert(
        "sec-ch-ua",
        HeaderValue::from_static(
            r#""Google Chrome";v="141", "Not?A_Brand";v="8", "Chromium";v="141""#,
        ),
    );

    headers.insert(
        USER_AGENT,
        HeaderValue::from_static("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/141.0.0.0 Safari/537.36"),
    );

    headers.insert("sec-ch-ua-mobile", HeaderValue::from_static("?0"));

    headers.insert("sec-ch-ua-platform", HeaderValue::from_static(r#""Linux""#));

    headers.insert("sec-fetch-dest", HeaderValue::from_static("empty"));

    headers.insert("sec-fetch-mode", HeaderValue::from_static("cors"));

    headers.insert("sec-fetch-site", HeaderValue::from_static("same-origin"));

    headers.insert("x-discord-locale", HeaderValue::from_static("en-US"));

    headers.insert(
        "x-super-properties",
        HeaderValue::from_str(&generate_super_properties()).expect("Invalid super properties"),
    );

    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    headers
}

fn generate_super_properties() -> String {
    let super_properties = json!({
        "os": "Linux",
        "browser": "Chrome",
        "device": "",
        "system_locale": "en-US",
        "browser_user_agent": "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/141.0.0.0 Safari/537.36",
        "browser_version": "124.0.0.0",
        "os_version": "10",
        "referrer": "",
        "referring_domain": "",
        "referrer_current": "",
        "referring_domain_current": "",
        "release_channel": "stable",
        "client_build_number": 9298544,
        "client_event_source": null,
        "design_id": 0
    });

    general_purpose::STANDARD.encode(super_properties.to_string())
}
