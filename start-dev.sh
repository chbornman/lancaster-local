#!/bin/bash

# Lancaster Community Platform - Development Start Script

echo "🚀 Starting Lancaster Community Platform..."

# Check if Docker is running
if ! docker info > /dev/null 2>&1; then
    echo "❌ Docker is not running. Please start Docker first."
    exit 1
fi

# Check if .env file exists
if [ ! -f .env ]; then
    echo "⚠️  .env file not found. Creating from example..."
    cat > .env << EOF
# Backend
DATABASE_URL=postgresql://lancaster:lancaster_pass@localhost:5432/lancaster_community
REDIS_URL=redis://localhost:6379
ADMIN_PASSWORD=secure_admin_password_2024
GOOGLE_TRANSLATE_API_KEY=your-google-translate-api-key-here
DEFAULT_LANGUAGE=en
SUPPORTED_LANGUAGES=en,es,de,fr,zh,ar,he,fa,ur
RTL_LANGUAGES=ar,he,fa,ur
PORT=3000

# Frontend
VITE_API_URL=http://localhost:3000/api
VITE_DEFAULT_LANGUAGE=en
VITE_RTL_LANGUAGES=ar,he,fa,ur
EOF
    echo "✅ .env file created. Please add your Google Translate API key."
fi

# Build and start containers
echo "🏗️  Building containers..."
docker compose build

echo "🚀 Starting services..."
docker compose up -d

# Wait for services to be ready
echo "⏳ Waiting for services to be ready..."
sleep 10

# Check service health
echo "🔍 Checking service status..."
docker compose ps

echo "
✅ Lancaster Community Platform is running!

🌐 Frontend: http://localhost
📡 API: http://localhost:3000/api
🗄️  Database: localhost:5432
📦 Redis: localhost:6379

👤 Admin Panel: http://localhost/admin
   Password: secure_admin_password_2024

📝 To view logs: make logs
🛑 To stop: make down

⚠️  Remember to add your Google Translate API key to .env
"