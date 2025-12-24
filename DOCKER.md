# Docker Deployment Guide

This document provides detailed instructions for deploying the Kythia Quest API using Docker.

## 📦 Overview

The project includes:
- **Multi-stage Dockerfile**: Optimized build with minimal runtime image
- **docker-compose.yml**: Complete setup with MySQL and API service
- **Health checks**: Automatic monitoring of service health
- **Persistent volumes**: Data persists across container restarts

## 🏗️ Architecture

```
┌─────────────────┐
│   Docker Host   │
│                 │
│  ┌───────────┐  │
│  │   API     │  │ Port 3000
│  │ Container │◄─┼─── HTTP Requests
│  └─────┬─────┘  │
│        │        │
│  ┌─────▼─────┐  │
│  │   MySQL   │  │ Port 3306
│  │ Container │  │
│  └─────┬─────┘  │
│        │        │
│  ┌─────▼─────┐  │
│  │  Volume   │  │
│  │mysql_data │  │
│  └───────────┘  │
└─────────────────┘
```

## 🚀 Quick Start

### 1. Prerequisites

Install Docker and Docker Compose:

**Linux (Ubuntu/Debian):**
```bash
# Install Docker
curl -fsSL https://get.docker.com -o get-docker.sh
sudo sh get-docker.sh

# Install Docker Compose
sudo apt-get install docker-compose-plugin

# Add user to docker group
sudo usermod -aG docker $USER
```

**macOS:**
```bash
brew install --cask docker
```

**Windows:**
Download and install Docker Desktop from https://www.docker.com/products/docker-desktop

### 2. Configuration

Create and configure your environment file:

```bash
cp .env.example .env
```

Edit `.env` and set your Discord token:
```env
DISCORD_TOKEN=your_actual_discord_token_here
```

### 3. Deploy

Start all services:
```bash
docker-compose up -d
```

Verify services are running:
```bash
docker-compose ps
```

Expected output:
```
NAME                IMAGE               STATUS
quest-api           kythia-quest-api    Up (healthy)
quest-api-mysql     mysql:8.0           Up (healthy)
```

### 4. Verify Deployment

Check health:
```bash
curl http://localhost:3000/health
```

Expected response:
```json
{"status":"ok"}
```

Check API:
```bash
curl http://localhost:3000/v1/quests
```

## 📁 Project Structure

```
kythia-quest-api/
├── Dockerfile              # Multi-stage build configuration
├── docker-compose.yml      # Service orchestration
├── .dockerignore          # Files to exclude from build
├── .env.example           # Environment template
├── migrations/            # Database migrations
│   └── 20241224000001_initial.sql
└── src/                   # Rust source code
```

## 🔧 Dockerfile Details

### Multi-Stage Build

#### Stage 1: Builder
```dockerfile
FROM rust:1.75 as builder
```
- Uses official Rust image
- Compiles the application in release mode
- Caches dependencies for faster rebuilds

#### Stage 2: Runtime
```dockerfile
FROM debian:bookworm-slim
```
- Minimal base image (~80MB vs ~1.5GB)
- Only includes runtime dependencies
- Non-root user for security
- Includes health check

### Build Optimizations

- **Layer caching**: Dependencies compiled separately
- **LTO**: Link-time optimization enabled
- **Strip**: Debug symbols removed
- **Result**: ~50MB final image size

## 🐳 Docker Compose Configuration

### Services

#### MySQL Service
```yaml
mysql:
  image: mysql:8.0
  environment:
    MYSQL_ROOT_PASSWORD: rootpassword
    MYSQL_DATABASE: quest_db
    MYSQL_USER: quest_user
    MYSQL_PASSWORD: quest_password
```

**Features:**
- Persistent data volume
- Health checks
- Automatic restart
- Isolated network

#### API Service
```yaml
api:
  build: .
  depends_on:
    mysql:
      condition: service_healthy
```

**Features:**
- Waits for MySQL to be healthy
- Automatic migrations on startup
- Health monitoring
- Graceful shutdown

### Volumes

```yaml
volumes:
  mysql_data:
    driver: local
```

Data persists at `/var/lib/docker/volumes/kythia-quest-api_mysql_data/`

### Networks

```yaml
networks:
  quest-network:
    driver: bridge
```

Isolated network for service communication.

## 🛠️ Common Operations

### View Logs

```bash
# All services
docker-compose logs -f

# Specific service
docker-compose logs -f api
docker-compose logs -f mysql

# Last 100 lines
docker-compose logs --tail=100 api
```

### Restart Services

```bash
# Restart all
docker-compose restart

# Restart specific service
docker-compose restart api
```

### Update Application

```bash
# Pull latest code
git pull

# Rebuild and restart
docker-compose up -d --build
```

### Access Container Shell

```bash
# API container
docker-compose exec api /bin/bash

# MySQL container
docker-compose exec mysql mysql -u quest_user -p quest_db
```

### Database Operations

#### Backup Database
```bash
docker-compose exec mysql mysqldump -u quest_user -pquest_password quest_db > backup.sql
```

#### Restore Database
```bash
docker-compose exec -T mysql mysql -u quest_user -pquest_password quest_db < backup.sql
```

#### Reset Database
```bash
# Stop services
docker-compose down -v

# Start fresh
docker-compose up -d
```

## 📊 Monitoring

### Health Checks

Both services have health checks:

**API:**
- Endpoint: `http://localhost:3000/health`
- Interval: 30s
- Timeout: 3s
- Retries: 3

**MySQL:**
- Command: `mysqladmin ping`
- Interval: 10s
- Timeout: 5s
- Retries: 5

### Check Health Status

```bash
docker-compose ps
docker inspect --format='{{.State.Health.Status}}' quest-api
docker inspect --format='{{.State.Health.Status}}' quest-api-mysql
```

### Resource Usage

```bash
# Real-time stats
docker stats quest-api quest-api-mysql

# Container info
docker-compose top
```

## 🔒 Security Best Practices

### 1. Environment Variables

Never commit `.env` to version control:
```bash
# .gitignore already includes
.env
```

### 2. Database Credentials

Change default passwords in production:
```env
MYSQL_ROOT_PASSWORD=strong_random_password_here
MYSQL_PASSWORD=another_strong_password_here
```

### 3. Network Exposure

Limit MySQL exposure:
```yaml
# Comment out to disable external access
# ports:
#   - "3306:3306"
```

### 4. Volume Permissions

Set appropriate permissions:
```bash
docker-compose exec api chown -R appuser:appuser /app
```

## 🚀 Production Deployment

### Using Docker Swarm

```bash
# Initialize swarm
docker swarm init

# Deploy stack
docker stack deploy -c docker-compose.yml quest-api

# Check services
docker stack services quest-api
```

### Using Kubernetes

Convert compose to Kubernetes:
```bash
kompose convert -f docker-compose.yml
kubectl apply -f .
```

### Environment-Specific Configs

```bash
# Development
docker-compose -f docker-compose.yml up

# Production
docker-compose -f docker-compose.yml -f docker-compose.prod.yml up -d
```

## 🐛 Troubleshooting

### Container Won't Start

```bash
# Check logs
docker-compose logs api

# Check events
docker events --filter container=quest-api

# Inspect container
docker inspect quest-api
```

### Database Connection Issues

```bash
# Verify MySQL is healthy
docker-compose exec mysql mysqladmin ping -h localhost

# Test connection
docker-compose exec api sh -c 'mysql -h mysql -u quest_user -pquest_password quest_db -e "SHOW TABLES;"'
```

### Port Conflicts

```bash
# Find process using port
sudo lsof -i :3000
sudo lsof -i :3306

# Change port in .env
PORT=3001
MYSQL_PORT=3307
```

### Out of Disk Space

```bash
# Clean up
docker system prune -a
docker volume prune

# Check usage
docker system df
```

### Migration Failures

```bash
# Check migration status
docker-compose exec api sh -c 'ls -la migrations/'

# Manual migration
docker-compose exec mysql mysql -u quest_user -pquest_password quest_db < migrations/20241224000001_initial.sql
```

## 📈 Scaling

### Horizontal Scaling

```bash
# Scale API instances
docker-compose up -d --scale api=3

# Add load balancer (nginx)
docker-compose -f docker-compose.yml -f docker-compose.lb.yml up -d
```

### Vertical Scaling

```yaml
# docker-compose.yml
services:
  api:
    deploy:
      resources:
        limits:
          cpus: '2'
          memory: 2G
        reservations:
          cpus: '1'
          memory: 1G
```

## 🔄 CI/CD Integration

### GitHub Actions Example

```yaml
name: Deploy

on:
  push:
    branches: [main]

jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Build and push
        run: |
          docker build -t myregistry/quest-api:${{ github.sha }} .
          docker push myregistry/quest-api:${{ github.sha }}
      
      - name: Deploy
        run: |
          docker-compose pull
          docker-compose up -d
```

## 📚 Additional Resources

- [Docker Documentation](https://docs.docker.com/)
- [Docker Compose Reference](https://docs.docker.com/compose/)
- [MySQL Docker Hub](https://hub.docker.com/_/mysql)
- [Rust Official Docker Images](https://hub.docker.com/_/rust)

## 💡 Tips

1. **Use `.dockerignore`**: Speeds up builds by excluding unnecessary files
2. **Multi-stage builds**: Keeps final image small
3. **Health checks**: Enable automatic recovery
4. **Volumes**: Always use volumes for persistent data
5. **Networks**: Isolate services for security
6. **Logging**: Configure log rotation to prevent disk fills

## 📞 Support

For issues and questions:
- Open an issue on GitHub
- Check existing documentation
- Review Docker logs

---

**Last Updated**: 2024-12-24
