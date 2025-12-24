mod config;
mod db;
mod routes;
mod utils;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::get,
    Router,
};
use sqlx::mysql::MySqlPool;
use std::{net::SocketAddr, sync::Arc};
use tower_http::{
    cors::{Any, CorsLayer},
    trace::{DefaultMakeSpan, DefaultOnFailure, DefaultOnResponse, TraceLayer},
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::config::Config;

#[derive(Clone)]
pub struct AppState {
    pub db: MySqlPool,
    pub config: Arc<Config>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing/logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "kythia_quest_api=info,tower_http=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("🚀 Starting Kythia Quest API");

    // Load configuration
    let config = Config::from_env()?;
    tracing::info!("✅ Configuration loaded");

    // Connect to database
    tracing::info!("🔗 Connecting to database...");
    let db = MySqlPool::connect(&config.database_url).await?;
    tracing::info!("✅ Database connected");

    // Run migrations
    tracing::info!("📦 Running database migrations...");
    sqlx::migrate!("./migrations").run(&db).await?;
    tracing::info!("✅ Migrations completed");

    // Create application state
    let app_state = AppState {
        db: db.clone(),
        config: Arc::new(config.clone()),
    };

    // Fetch quests on startup to pre-populate database and cache
    tracing::info!("🚀 Fetching initial quest data...");
    match fetch_and_cache_quests(&app_state).await {
        Ok(_) => tracing::info!("✅ Initial quest data loaded"),
        Err(e) => tracing::warn!("⚠️  Failed to fetch initial quests: {}", e),
    }

    // Build router with API routes
    let api_router = Router::new().nest("/quests", routes::quests::router());

    let app = Router::new()
        .route("/health", get(routes::health::health_check))
        .nest("/v1", api_router)
        .fallback(not_found_handler)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::default().include_headers(true))
                .on_response(DefaultOnResponse::default())
                .on_failure(DefaultOnFailure::default()),
        )
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods([
                    axum::http::Method::GET,
                    axum::http::Method::POST,
                    axum::http::Method::PUT,
                    axum::http::Method::DELETE,
                ])
                .allow_headers(Any),
        )
        .with_state(app_state);

    // Get server address
    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));

    tracing::info!("🎯 Server running on http://0.0.0.0:{}", config.port);
    tracing::info!(
        "📡 API available at http://0.0.0.0:{}/v1/quests",
        config.port
    );
    tracing::info!("💚 Health check at http://0.0.0.0:{}/health", config.port);

    // Start server with graceful shutdown
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();

    Ok(())
}

// Fetch and cache quests on startup
async fn fetch_and_cache_quests(state: &AppState) -> Result<(), Box<dyn std::error::Error>> {
    use crate::db::operations::upsert_cache;
    use crate::utils::discord::fetch_discord_quests;
    use crate::utils::quest_parser::{reconstruct_discord_response, save_discord_quests_to_db};

    // Fetch from Discord
    let quests_data = fetch_discord_quests(&state.config.discord_token).await?;

    // Save new quests to database
    let new_count = save_discord_quests_to_db(&state.db, &quests_data).await?;

    if new_count > 0 {
        tracing::info!("   → Inserted {} new quest(s)", new_count);
    } else {
        tracing::info!("   → Database already up to date");
    }

    // Reconstruct and cache
    let reconstructed =
        reconstruct_discord_response(&state.db, state.config.quest_age_days).await?;
    upsert_cache(&state.db, "discord_quests", &reconstructed).await?;

    Ok(())
}

// 404 handler
async fn not_found_handler() -> impl IntoResponse {
    let body = serde_json::json!({
        "error": "Not Found",
        "message": "The requested endpoint does not exist",
        "status": 404
    });

    (StatusCode::NOT_FOUND, Json(body))
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            tracing::info!("🛑 Received Ctrl+C, shutting down gracefully...");
        },
        _ = terminate => {
            tracing::info!("🛑 Received terminate signal, shutting down gracefully...");
        },
    }
}
