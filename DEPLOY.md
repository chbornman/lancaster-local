# Production Deployment Guide

## Quick VPS Deployment

### 1. Clone and Setup
```bash
git clone <your-repo-url>
cd lancal

# Copy and configure environment
cp .env.example .env
# Edit .env with your values:
# - Add your Google Translate API key
# - Set secure passwords
# - Set your domain
```

### 2. Deploy with Docker
```bash
# Start all services (includes migrations and seeding)
docker compose -f docker-compose.prod.yml up -d

# Check status
docker compose -f docker-compose.prod.yml ps
```

### 3. Configure Your VPS Nginx

Add this to your nginx site configuration:

```nginx
server {
    listen 80;
    server_name your-domain.com;

    # Frontend (React app)
    location / {
        proxy_pass http://localhost:3001;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection 'upgrade';
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
        proxy_cache_bypass $http_upgrade;
    }

    # Backend API
    location /api/ {
        proxy_pass http://localhost:3000;
        proxy_http_version 1.1;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }
}
```

Then reload nginx:
```bash
sudo nginx -t
sudo systemctl reload nginx
```

## Services Overview

- **Frontend**: Available on port 3001 (React/Vite production build)
- **Backend**: Available on port 3000 (Rust/Axum API)
- **Database**: PostgreSQL (internal only)
- **Redis**: Cache (internal only)

## Environment Variables

Required in your `.env` file:

```env
GOOGLE_TRANSLATE_API_KEY=your_api_key_here
ADMIN_PASSWORD=your_secure_admin_password
POSTGRES_USER=lancaster
POSTGRES_PASSWORD=your_secure_db_password
POSTGRES_DB=lancaster_community
DEFAULT_LANGUAGE=en
SUPPORTED_LANGUAGES=en,es,de,fr,zh,ar,he,fa,ur
RTL_LANGUAGES=ar,he,fa,ur
```

## Maintenance Commands

```bash
# View logs
docker compose -f docker-compose.prod.yml logs -f

# Restart services
docker compose -f docker-compose.prod.yml restart

# Update application
git pull
docker compose -f docker-compose.prod.yml up -d --build

# Run migrations manually (if needed)
docker compose -f docker-compose.prod.yml run --rm migrate

# Re-seed database (if needed)
docker compose -f docker-compose.prod.yml run --rm seed
```

## Admin Access

Once deployed, access the admin panel at:
`https://your-domain.com/admin`

Use the `ADMIN_PASSWORD` from your `.env` file to log in.

## Troubleshooting

- Check service status: `docker compose -f docker-compose.prod.yml ps`
- View logs: `docker compose -f docker-compose.prod.yml logs [service-name]`
- Restart all services: `docker compose -f docker-compose.prod.yml restart`