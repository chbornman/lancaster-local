# Development and Production Environment Setup

## Overview

This project uses Docker Compose for both development and production environments, with clear separation of concerns.

## Development Environment

### Configuration
- Uses `docker-compose.yml`
- Reads environment variables from `.env` file
- All services run locally with hot-reloading

### Setup
1. Copy `.env.example` to `.env`
2. Update passwords and API keys in `.env`
3. Run `docker compose up -d`

### Access Points
- Frontend: http://localhost:5173 (Vite dev server)
- Backend API: http://localhost:3000
- PostgreSQL: localhost:5432
- Redis: localhost:6379

### Features
- Hot reloading for both frontend (Vite) and backend (cargo-watch)
- Volume mounts for source code
- Database migrations run automatically
- Seed data loaded on first start

## Production Environment

### Configuration
- Uses `docker-compose.prod.yml`
- Environment variables from GitHub Secrets
- Pre-built Docker images from GitHub Container Registry

### Deployment
- Automated via GitHub Actions on push to `main` branch
- Images built and pushed to `ghcr.io`
- Deployed to VPS with secure environment variables

### Key Differences from Development
1. **Images**: Uses pre-built images instead of building locally
2. **Environment**: Variables from GitHub Secrets, not local `.env`
3. **Ports**: Different ports (3010 for backend, 3011 for frontend)
4. **Optimization**: Production builds with optimizations enabled
5. **Security**: No source code volumes mounted

## Environment Variables

### Required for Both Environments
- `GOOGLE_TRANSLATE_API_KEY` - Google Translate API key
- `ADMIN_PASSWORD` - Admin panel password
- `POSTGRES_USER` - Database username
- `POSTGRES_PASSWORD` - Database password
- `POSTGRES_DB` - Database name

### Additional for Production
- `DOMAIN` - Your production domain
- Set as GitHub Secrets for security

## Database Management

### Development
- Migrations run automatically via `migrate` service
- Seed data loaded via `seed` service
- Can reset with `docker compose down -v`

### Production
- Migrations run during deployment
- Database persisted between deployments
- Backup strategy recommended

## Security Notes

1. Never commit `.env` file (it's in `.gitignore`)
2. Use strong passwords in production
3. All sensitive values should be GitHub Secrets
4. Database not exposed externally in production

## Troubleshooting

### Backend not reading environment variables
- Ensure variables are in `docker-compose.yml` environment section
- The backend uses `dotenv` but in Docker, env vars come from compose

### Events endpoint error
- Check if migrations ran successfully
- Ensure backend code matches database schema

### Fresh start
```bash
docker compose down -v  # Remove everything including volumes
docker compose up -d    # Start fresh
```