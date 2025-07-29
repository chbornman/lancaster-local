#!/bin/bash
set -e

echo "🔨 Building backend Docker image locally..."
docker build -t lancaster-backend-local ./backend

echo "🏷️  Tagging image for production..."
docker tag lancaster-backend-local ghcr.io/chbornman/lancaster-local-backend:main

echo "🛑 Stopping backend container..."
docker compose -f docker-compose.prod.yml stop backend

echo "🔄 Starting backend with new image..."
docker compose -f docker-compose.prod.yml up -d backend

echo "⏳ Waiting for backend to be ready..."
sleep 5

echo "🌱 Running seed..."
docker compose -f docker-compose.prod.yml run --rm seed

echo "✅ Done! Checking if seed worked..."
docker compose -f docker-compose.prod.yml exec db psql -U lancaster_user -d lancaster_local_db -c "SELECT 'Posts:' as table_name, COUNT(*) as count FROM posts UNION ALL SELECT 'Events:', COUNT(*) FROM events UNION ALL SELECT 'Languages:', COUNT(*) FROM supported_languages;"