# Kythia Quest API 🎮

A high-performance Discord Quest API written in Rust with MySQL database support. This API fetches, stores, and serves Discord quest data through a fast and reliable REST endpoint with intelligent caching and database normalization.

[![Rust](https://img.shields.io/badge/rust-1.75+-orange.svg)](https://www.rust-lang.org)
[![MySQL](https://img.shields.io/badge/mysql-8.0+-blue.svg)](https://www.mysql.com)
[![Docker](https://img.shields.io/badge/docker-ready-brightgreen.svg)](https://www.docker.com)

---

## ✨ Features

- 🚀 **Blazing Fast**: Written in Rust with Axum framework
- 💾 **Normalized Database**: Proper relational schema with historical tracking
- ⚡ **Smart Caching**: Configurable cache duration to minimize API calls
- 🎯 **Intelligent Updates**: Only inserts new quests, skips existing ones
- 📅 **Age Filtering**: Configurable quest age filter to reduce response size
- 🚀 **Startup Fetch**: Automatically pre-loads quests on server start
- 🐳 **Docker Ready**: Full Docker and Docker Compose support
- 🔧 **Flexible Config**: All settings via environment variables
- 📊 **Health Checks**: Built-in monitoring endpoints
- 🛡️ **Robust Errors**: Comprehensive error handling with proper HTTP codes
- 📝 **Structured Logging**: Detailed logging with tracing

---

## 📋 Table of Contents

- [Quick Start (Beginners)](#-quick-start-beginners)
- [Advanced Setup (Experts)](#-advanced-setup-experts)
- [API Documentation](#-api-documentation)
- [Database Schema](#️-database-schema)
- [Configuration Reference](#-configuration-reference)
- [Performance Optimizations](#-performance-optimizations)
- [Troubleshooting](#-troubleshooting)

---

## 🚀 Quick Start (Beginners)

Perfect for getting up and running in 5 minutes!

### Prerequisites

- **Docker Desktop** (easiest option) - [Download here](https://www.docker.com/products/docker-desktop)
- **Discord Account** with an auth token (we'll show you how to get it)

### Step 1: Get Your Discord Token

1. Open [Discord](https://discord.com) in your browser
2. Press `F12` to open Developer Tools
3. Go to the **Console** tab
4. Paste this code and press Enter:
   ```javascript
   (webpackChunkdiscord_app.push([[''],{},e=>{m=[];for(let c in e.c)m.push(e.c[c])}]),m).find(m=>m?.exports?.default?.getToken!==void 0).exports.default.getToken()
   ```
5. Copy the token that appears

⚠️ **Keep this token private!** Never share it with anyone.

### Step 2: Clone and Setup

```bash
# Clone the repository
git clone https://github.com/yourusername/kythia-quest-api.git
cd kythia-quest-api

# Copy the example environment file
cp .env.example .env
```

### Step 3: Configure Environment

Edit the `.env` file and add your Discord token:

```bash
# Open .env in your favorite text editor
nano .env  # or: code .env (VS Code) | notepad .env (Windows)
```

Replace `your_discord_user_token_here` with the token you copied:

```env
DISCORD_TOKEN=your-discord-user-token-here
```

Save and close the file.

### Step 4: Start the API

```bash
docker-compose up -d
```

That's it! 🎉

### Step 5: Test the API

**Check if it's running:**
```bash
curl http://localhost:3000/health
```

**Get quest data:**
```bash
curl http://localhost:3000/v1/quests
```

You should see JSON data with all available Discord quests!

### Step 6: View Logs

```bash
# See what's happening
docker-compose logs -f api

# Stop viewing logs: Press Ctrl+C
```

### Stop the API

```bash
docker-compose down
```

---

## 🔧 Advanced Setup (Experts)

For developers who want full control and customization.

### 📦 Technology Stack

| Component | Technology | Version |
|-----------|-----------|---------|
| Language | Rust | 1.75+ |
| Web Framework | Axum | 0.7 |
| Database | MySQL | 8.0+ |
| ORM | SQLx (async) | 0.7 |
| HTTP Client | reqwest | 0.11 |
| Runtime | Tokio | 1.40 |
| Serialization | serde | 1.0 |
| Logging | tracing | 0.1 |

### 🏗️ Architecture Overview

```
┌─────────────┐
│   Client    │
└──────┬──────┘
       │ HTTP
       ▼
┌─────────────────────────────────────┐
│       Axum Web Server               │
│  ┌────────────┐   ┌──────────────┐ │
│  │  Routes    │──▶│  Middleware  │ │
│  └────────────┘   └──────────────┘ │
└────────┬────────────────────────────┘
         │
    ┌────┴─────┐
    │          │
    ▼          ▼
┌────────┐ ┌──────────────┐
│ Cache  │ │    MySQL     │
│ Layer  │ │  (Normalized)│
└────────┘ └──────────────┘
    │               │
    └───────┬───────┘
            ▼
    ┌───────────────┐
    │  Discord API  │
    └───────────────┘
```

### 🛠️ Local Development Setup

#### 1. Install Dependencies

**Rust:**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
rustc --version  # Verify installation
```

**MySQL:**
```bash
# Ubuntu/Debian
sudo apt update && sudo apt install mysql-server

# macOS
brew install mysql

# Arch Linux
sudo pacman -S mysql
```

**SQLx CLI (for migrations):**
```bash
cargo install sqlx-cli --no-default-features --features mysql
```

#### 2. Database Setup

**Start MySQL:**
```bash
sudo systemctl start mysql  # Linux
brew services start mysql    # macOS
```

**Create database and user:**
```bash
mysql -u root -p
```

```sql
CREATE DATABASE quest_db CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;
CREATE USER 'quest_user'@'localhost' IDENTIFIED BY 'your_secure_password';
GRANT ALL PRIVILEGES ON quest_db.* TO 'quest_user'@'localhost';
FLUSH PRIVILEGES;
EXIT;
```

#### 3. Environment Configuration

Create `.env` file:
```bash
cp .env.example .env
```

Configure all variables (see [Configuration Reference](#-configuration-reference)):
```env
# Discord
DISCORD_TOKEN=your_actual_discord_token_here

# Database
DATABASE_URL=mysql://quest_user:your_secure_password@localhost:3306/quest_db

# Server
PORT=3000

# Cache (in minutes)
CACHE_DURATION_MINUTES=30

# Quest Age Filter (in days)
QUEST_AGE_DAYS=30

# Logging
RUST_LOG=info,kythia_quest_api=debug
```

#### 4. Run Migrations

```bash
sqlx migrate run
```

Verify migrations:
```bash
mysql -u quest_user -p quest_db -e "SHOW TABLES;"
```

Expected output:
```
+--------------------+
| Tables_in_quest_db |
+--------------------+
| _sqlx_migrations   |
| cache_store        |
| quest_assets       |
| quest_features     |
| quest_rewards      |
| quest_tasks        |
| quest_user_status  |
| quests             |
+--------------------+
```

#### 5. Build and Run

**Development mode (with hot-reload):**
```bash
cargo install cargo-watch
cargo watch -x run
```

**Production mode:**
```bash
cargo build --release
./target/release/kythia-quest-api
```

**With custom log level:**
```bash
RUST_LOG=debug cargo run
```

### 🐳 Docker Deployment

#### Option 1: Docker Compose (Recommended)

**Full stack with MySQL:**
```bash
docker-compose up -d
```

**View logs:**
```bash
docker-compose logs -f           # All services
docker-compose logs -f api       # API only
docker-compose logs -f mysql     # Database only
```

**Restart services:**
```bash
docker-compose restart api
```

**Stop and remove:**
```bash
docker-compose down              # Stop
docker-compose down -v           # Stop and remove volumes (clears data)
```

#### Option 2: Standalone Docker

**Build image:**
```bash
docker build -t kythia-quest-api .
```

**Run container:**
```bash
docker run -d \
  --name kythia-api \
  -p 3000:3000 \
  -e DISCORD_TOKEN="your_token" \
  -e DATABASE_URL="mysql://user:pass@host:3306/db" \
  -e CACHE_DURATION_MINUTES=30 \
  -e QUEST_AGE_DAYS=30 \
  kythia-quest-api
```

**View logs:**
```bash
docker logs -f kythia-api
```

### 🔍 Development Tools

**Code formatting:**
```bash
cargo fmt
```

**Linting:**
```bash
cargo clippy -- -D warnings
```

**Testing:**
```bash
cargo test
```

**Check without building:**
```bash
cargo check
```

**Clean build artifacts:**
```bash
cargo clean
```

---

## 📡 API Documentation

### Endpoints

#### `GET /health`
Health check endpoint.

**Response:**
```json
{
  "status": "ok"
}
```

**Status Codes:**
- `200 OK` - Service healthy

---

#### `GET /v1/quests`
Fetches all active Discord quests (filtered by age).

**Response:**
```json
{
  "quests": [
    {
      "id": "1443000962024210432",
      "config": {
        "config_version": 2,
        "starts_at": "2025-12-02T18:00:39+00:00",
        "expires_at": "2025-12-15T00:00:39+00:00",
        "features": [3, 9, 13, 14, 15],
        "application": {
          "id": "1443287416030105692",
          "name": "Storm Lancers",
          "link": "https://store.steampowered.com/app/..."
        },
        "assets": {
          "hero": "quests/.../image.jpg",
          "hero_video": null,
          "game_tile": "tile.png",
          "logotype": "logo.png"
        },
        "colors": {
          "primary": "#4752C4",
          "secondary": "#000000"
        },
        "messages": {
          "quest_name": "Storm Lancers Demo",
          "game_title": "Storm Lancers Demo",
          "game_publisher": "ProbablyMonsters"
        },
        "task_config_v2": {
          "tasks": {
            "PLAY_ON_DESKTOP": {
              "type": "PLAY_ON_DESKTOP",
              "target": 900,
              "applications": [...]
            }
          },
          "join_operator": "or"
        },
        "rewards_config": {
          "assignment_method": 1,
          "rewards": [
            {
              "type": 4,
              "sku_id": "1287881739531976815",
              "messages": {
                "name": "700 Orbs"
              },
              "orb_quantity": 700
            }
          ],
          "rewards_expire_at": "2026-01-14T00:00:39+00:00",
          "platforms": [0]
        },
        "share_policy": "shareable_everywhere",
        "cta_config": {
          "link": "https://...",
          "button_label": "Get Game"
        }
      },
      "user_status": null,
      "targeted_content": [],
      "preview": false
    }
  ]
}
```

**Caching:**
- Cached for `CACHE_DURATION_MINUTES` (default: 30 minutes)
- Fresh data fetched from Discord when cache expires
- Only new quests inserted into database (optimization)

**Filtering:**
- Returns quests from last `QUEST_AGE_DAYS` days only (default: 30)
- Based on quest `expires_at` date

**Status Codes:**
- `200 OK` - Successful response
- `500 Internal Server Error` - Server error
- `502 Bad Gateway` - Discord API unavailable

---

## 🗄️ Database Schema

### Normalized Tables

The API uses a **normalized relational schema** for efficient storage and querying:

```sql
quests
├── quest_assets (1:1)
├── quest_tasks (1:N)
├── quest_rewards (1:N)
├── quest_features (1:N)
└── quest_user_status (1:N)
```

#### `quests` - Main Quest Data
```sql
CREATE TABLE quests (
    id VARCHAR(255) PRIMARY KEY,
    config_version INT NOT NULL,
    starts_at DATETIME NOT NULL,
    expires_at DATETIME NOT NULL,
    application_id VARCHAR(255) NOT NULL,
    application_name VARCHAR(255) NOT NULL,
    application_link TEXT,
    share_policy VARCHAR(100),
    preview BOOLEAN DEFAULT FALSE,
    primary_color VARCHAR(10),
    secondary_color VARCHAR(10),
    quest_name VARCHAR(255) NOT NULL,
    game_title VARCHAR(255) NOT NULL,
    game_publisher VARCHAR(255),
    cta_link TEXT,
    cta_button_label VARCHAR(100),
    task_join_operator VARCHAR(10) DEFAULT 'or',
    reward_assignment_method INT,
    rewards_expire_at DATETIME,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    INDEX idx_expires_at (expires_at),
    INDEX idx_starts_at (starts_at)
);
```

#### `quest_assets` - Quest Media
```sql
CREATE TABLE quest_assets (
    quest_id VARCHAR(255) PRIMARY KEY,
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
    FOREIGN KEY (quest_id) REFERENCES quests(id) ON DELETE CASCADE
);
```

#### `quest_tasks` - Task Requirements
```sql
CREATE TABLE quest_tasks (
    id INT AUTO_INCREMENT PRIMARY KEY,
    quest_id VARCHAR(255) NOT NULL,
    task_type VARCHAR(100) NOT NULL,
    target INT NOT NULL,
    applications JSON,
    external_ids JSON,
    FOREIGN KEY (quest_id) REFERENCES quests(id) ON DELETE CASCADE,
    INDEX idx_quest_id (quest_id)
);
```

#### `quest_rewards` - Reward Details
```sql
CREATE TABLE quest_rewards (
    id INT AUTO_INCREMENT PRIMARY KEY,
    quest_id VARCHAR(255) NOT NULL,
    reward_type INT NOT NULL,
    sku_id VARCHAR(255),
    reward_name VARCHAR(255) NOT NULL,
    reward_name_with_article VARCHAR(255),
    orb_quantity INT,
    redemption_instructions JSON,
    platform INT,
    FOREIGN KEY (quest_id) REFERENCES quests(id) ON DELETE CASCADE,
    INDEX idx_quest_id (quest_id)
);
```

#### `quest_features` - Feature Flags
```sql
CREATE TABLE quest_features (
    id INT AUTO_INCREMENT PRIMARY KEY,
    quest_id VARCHAR(255) NOT NULL,
    feature_id INT NOT NULL,
    FOREIGN KEY (quest_id) REFERENCES quests(id) ON DELETE CASCADE,
    INDEX idx_quest_id (quest_id)
);
```

#### `quest_user_status` - User Progress (Not Used)
Reserved for future user progress tracking.

#### `cache_store` - Response Cache
```sql
CREATE TABLE cache_store (
    id VARCHAR(255) PRIMARY KEY,
    data JSON NOT NULL,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    INDEX idx_updated_at (updated_at)
);
```

### Schema Benefits

✅ **Historical Tracking** - Timestamps track when quests were added/updated  
✅ **Efficient Queries** - Proper indexes on foreign keys and dates  
✅ **Data Integrity** - Foreign key constraints prevent orphaned records  
✅ **Flexible Retrieval** - Can query specific quest components  
✅ **Performance** - Only new quests are inserted (no unnecessary updates)

---

## ⚙️ Configuration Reference

All configuration is done via environment variables in the `.env` file.

### Required Variables

| Variable | Description | Example |
|----------|-------------|---------|
| `DISCORD_TOKEN` | Your Discord user token | `MTI1ODY1...` |
| `DATABASE_URL` | MySQL connection string | `mysql://user:pass@host:3306/db` |

### Optional Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `PORT` | `3000` | Server port |
| `CACHE_DURATION_MINUTES` | `30` | How long to cache responses |
| `QUEST_AGE_DAYS` | `30` | Only return quests from last N days |
| `RUST_LOG` | `info` | Log level (`trace`, `debug`, `info`, `warn`, `error`) |

### Database URL Format

```
mysql://[user]:[password]@[host]:[port]/[database]
```

**Examples:**

Local MySQL:
```
mysql://quest_user:mypassword@localhost:3306/quest_db
```

Docker Compose:
```
mysql://quest_user:quest_password@mysql:3306/quest_db
```

Remote MySQL:
```
mysql://user:pass@192.168.1.100:3306/quests
```

### Environment Examples

**Development:**
```env
DISCORD_TOKEN=your_token_here
DATABASE_URL=mysql://root:root@localhost:3306/quest_dev
PORT=3000
CACHE_DURATION_MINUTES=5
QUEST_AGE_DAYS=90
RUST_LOG=debug,kythia_quest_api=trace
```

**Production:**
```env
DISCORD_TOKEN=your_token_here
DATABASE_URL=mysql://prod_user:secure_pass@prod-db.example.com:3306/quest_prod
PORT=8080
CACHE_DURATION_MINUTES=60
QUEST_AGE_DAYS=30
RUST_LOG=info
```

---

## ⚡ Performance Optimizations

### 1. Intelligent Quest Updates

The API only inserts **new quests** to the database:

```
┌─────────────────────────────────┐
│  Fetch Discord API Response     │
└────────────┬────────────────────┘
             │
             ▼
┌─────────────────────────────────┐
│  Get Existing Quest IDs from DB │
└────────────┬────────────────────┘
             │
             ▼
┌─────────────────────────────────┐
│  Filter: Keep Only NEW Quests   │
└────────────┬────────────────────┘
             │
             ▼
┌─────────────────────────────────┐
│  Insert Only New Quests         │
│  (Skip Existing Ones)           │
└─────────────────────────────────┘
```

**Performance Impact:**
- **First fetch:** ~10-20 seconds (inserts all quests)
- **Subsequent fetches (with new quests):** ~2 seconds (inserts only new)
- **No new quests:** ~0.5 seconds (skips all inserts) 🚀

### 2. Age-Based Filtering

Only quests from the last `QUEST_AGE_DAYS` are returned:

```sql
WHERE expires_at >= DATE_SUB(NOW(), INTERVAL ? DAY)
```

**Benefits:**
- Smaller response payloads
- Faster JSON serialization
- Reduced bandwidth usage

### 3. Smart Caching

- **Cache hits:** Instant response from database
- **Cache misses:** Fresh data from Discord + database update
- **Configurable duration:** Adjust based on your needs

### 4. Startup Pre-loading

Server fetches quests on startup:
```
🚀 Starting → 📡 Fetch Discord → 💾 Save to DB → 📦 Cache → ✅ Ready
```

**Result:** First API request is instant!

---

## 🔧 Troubleshooting

### Common Issues

#### 1. "DISCORD_TOKEN must be set"

**Problem:** Missing or empty Discord token.

**Solution:**
```bash
# Make sure .env has the token
cat .env | grep DISCORD_TOKEN

# Should show:
# DISCORD_TOKEN=MTI1ODY1...
```

---

#### 2. "Database connection failed"

**Problem:** Can't connect to MySQL.

**Check if MySQL is running:**
```bash
# Linux
sudo systemctl status mysql

# macOS
brew services list

# Docker
docker-compose ps
```

**Test connection manually:**
```bash
mysql -h localhost -u quest_user -p -D quest_db
```

**Common fixes:**
- Verify `DATABASE_URL` in `.env`
- Check MySQL credentials
- Ensure database exists
- Check firewall settings

---

#### 3. "Discord API error: unauthorized"

**Problem:** Invalid Discord token.

**Solution:**
1. Get a fresh token (see [Step 1](#step-1-get-your-discord-token))
2. Update `.env`
3. Restart the server

---

#### 4. "Port 3000 already in use"

**Problem:** Another service is using the port.

**Find what's using it:**
```bash
# Linux/macOS
lsof -i :3000

# Windows
netstat -ano | findstr :3000
```

**Solutions:**
1. Stop the other service
2. Change `PORT` in `.env`:
   ```env
   PORT=3001
   ```
3. Kill the process:
   ```bash
   kill -9 <PID>
   ```

---

#### 5. "No new quests - database is up to date" (but quests are missing)

**Problem:** Database has stale data.

**Clear database:**
```bash
# Docker
docker-compose down -v
docker-compose up -d

# Local MySQL
mysql -u quest_user -p quest_db
DROP DATABASE quest_db;
CREATE DATABASE quest_db;
EXIT;
sqlx migrate run
```

---

#### 6. Compilation errors

**Problem:** Build fails with SQLx errors.

**Solution:**
```bash
# Prepare SQLx offline mode
cargo sqlx prepare

# Or use online mode
cargo check
```

---

### Debug Mode

Enable verbose logging:

```bash
# In .env
RUST_LOG=debug,kythia_quest_api=trace,sqlx=debug

# Or inline
RUST_LOG=trace cargo run
```

Logs will show:
- Detailed HTTP requests
- Database queries
- Cache operations
- Discord API responses

---

## 📜 License

This project is licensed under the CC BY-NC 4.0 License. See the [LICENSE](LICENSE) file for details.
Copyright © 2025 Kythia Labs - All rights reserved.


---

## 👤 Author

**kenndeclouv**
- Email: kenndeclouv@gmail.com
- GitHub: https://github.com/kenndeclouv

---

## 🤝 Contributing

Contributions, issues, and feature requests are welcome!

1. Fork the repository
2. Create your feature branch: `git checkout -b feature/amazing-feature`
3. Commit your changes: `git commit -m 'Add amazing feature'`
4. Push to the branch: `git push origin feature/amazing-feature`
5. Open a pull request

---

## 📚 Additional Resources

- [Rust Documentation](https://doc.rust-lang.org/)
- [Axum Guide](https://docs.rs/axum/latest/axum/)
- [SQLx Documentation](https://docs.rs/sqlx/latest/sqlx/)
- [Docker Documentation](https://docs.docker.com/)
- [MySQL Reference](https://dev.mysql.com/doc/)

---

<div align="center">
Made with ❤️ by <a href="https://github.com/kythia">Kythia Labs</a>
</div>
