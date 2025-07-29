# Lancaster Community Platform

A multi-language community platform with news feed and calendar, featuring full RTL support for Arabic and other RTL languages.

## Features

- 📰 Community news feed with public submission
- 📅 Event calendar with category filtering
- 🌍 Multi-language support (English, Spanish, German, French, Chinese, Arabic)
- 🔄 Automatic translation using Google Translate API
- ↔️ Full RTL/LTR support with proper typography
- 👨‍💼 Simple admin interface for content moderation
- 📱 Mobile responsive design

## Tech Stack

- **Backend**: Rust with Axum framework
- **Frontend**: React with Vite, React Router, and i18next
- **Database**: PostgreSQL
- **Cache**: Redis
- **Proxy**: Nginx
- **Container**: Docker & Docker Compose

## Quick Start

### Development Environment

1. Clone the repository
2. Copy `.env.example` to `.env` and update values:
   ```bash
   cp .env.example .env
   # Edit .env to set your passwords and API keys
   ```
3. Start the development environment:
   ```bash
   docker compose up -d
   # Or use: make up
   ```
4. Access the app at:
   - Frontend: http://localhost:5173
   - Backend API: http://localhost:3000

### Production Environment

Production deployment is automated via GitHub Actions. When you push to the `main` branch:

1. Docker images are built and pushed to GitHub Container Registry
2. The application is deployed to your VPS automatically
3. Environment variables are securely managed through GitHub Secrets

See [DEPLOY.md](DEPLOY.md) for detailed production setup instructions.

## Development vs Production

This project uses different configurations for development and production environments:

### Development Environment

Uses `docker-compose.yml` for local development:
- Builds services from local source code
- Exposes database and Redis ports for debugging
- Uses development credentials
- Hot-reloading enabled for faster development

### Production Environment

Uses `docker-compose.prod.yml` for production deployment:
- Uses pre-built images from GitHub Container Registry
- No ports exposed except web services
- Secure credentials from environment variables
- Automated deployment via GitHub Actions

## Development Setup

### Prerequisites

- Docker and Docker Compose
- Make (optional, for convenience commands)
- Google Cloud Translation API key

### Environment Variables

Copy `.env.example` to `.env` and configure:

```bash
# Google Translate API
GOOGLE_TRANSLATE_API_KEY=your-api-key-here

# Admin credentials
ADMIN_PASSWORD=your-secure-password

# Database (for production)
POSTGRES_USER=your-db-user
POSTGRES_PASSWORD=your-db-password
POSTGRES_DB=your-db-name

# Application settings
DEFAULT_LANGUAGE=en
SUPPORTED_LANGUAGES=en,es,de,fr,zh,ar,he,fa,ur
RTL_LANGUAGES=ar,he,fa,ur

# Production domain
DOMAIN=your-domain.com
```

**Note**: The development `docker-compose.yml` uses default credentials for database. Production uses the values from `.env` file.

### Development Commands

```bash
# Start development environment
make up         # or: docker compose up -d

# View logs
make logs       # or: docker compose logs -f

# Stop services
make down       # or: docker compose down

# Rebuild after code changes
make build      # or: docker compose build

# Clean everything (including volumes)
make clean      # or: docker compose down -v

# Run migrations (development)
docker compose run --rm migrate

# Run seed data (development - first time only)
docker compose run --rm seed
```

## Production Deployment

### Initial Setup

1. Set up GitHub Secrets for deployment:
   - `VPS_HOST` - Your server IP/hostname
   - `VPS_USERNAME` - SSH username (e.g., deployer)
   - `VPS_SSH_KEY` - Private SSH key for authentication
   - `VPS_APP_PATH` - Path to application (e.g., /home/deployer/lancaster-local)
   - `GOOGLE_TRANSLATE_API_KEY` - Google Translate API key
   - `ADMIN_PASSWORD` - Admin panel password
   - `POSTGRES_USER` - Database username
   - `POSTGRES_PASSWORD` - Database password
   - `POSTGRES_DB` - Database name
   - `DOMAIN` - Your domain name

2. Clone repository on production server:
   ```bash
   git clone https://github.com/yourusername/lancaster-local.git
   cd lancaster-local
   ```

3. Push to main branch to trigger deployment

4. After first deployment, run seed data manually:
   ```bash
   docker compose -f docker-compose.prod.yml run --rm seed
   ```

### Production Commands

```bash
# View production logs
docker compose -f docker-compose.prod.yml logs -f

# Stop production services
docker compose -f docker-compose.prod.yml down

# Start production services
docker compose -f docker-compose.prod.yml up -d

# Run migrations manually (if needed)
docker compose -f docker-compose.prod.yml run --rm migrate

# Run seed data (first time only)
docker compose -f docker-compose.prod.yml run --rm seed

# View database tables
source .env && docker compose -f docker-compose.prod.yml exec db psql -U $POSTGRES_USER -d $POSTGRES_DB

# Clean and restart fresh
docker compose -f docker-compose.prod.yml down -v
docker compose -f docker-compose.prod.yml up -d
docker compose -f docker-compose.prod.yml run --rm migrate
docker compose -f docker-compose.prod.yml run --rm seed
```

### Deployment Process

1. Code changes pushed to `main` branch trigger GitHub Actions
2. GitHub Actions builds new Docker images and pushes to registry
3. Deployment script SSHs to production server
4. Updates `.env` file with secrets
5. Pulls latest images
6. Restarts services with `docker-compose.prod.yml`
7. Automatically runs migrations
8. Old images are cleaned up

### Important Notes

- Migrations run automatically on each deployment
- Seed data must be run manually (only needed once)
- Production uses `.env` file (not `.env.production`)
- All sensitive data comes from GitHub Secrets
- Database volume persists between deployments

### API Endpoints

- `GET /api/health` - Health check
- `GET /api/languages` - Get supported languages
- `GET /api/posts?lang=xx` - Get posts in specified language
- `POST /api/posts` - Submit new post
- `GET /api/events?lang=xx&month=YYYY-MM` - Get events
- `POST /api/events` - Submit new event
- `POST /api/admin/login` - Admin authentication
- Admin endpoints require authentication token

## RTL Support

The platform fully supports RTL languages including:

- Arabic (العربية)
- Hebrew (עברית)
- Persian (فارسی)
- Urdu (اردو)

RTL features include:

- Automatic text direction detection
- Bidirectional layout switching
- Proper typography and font rendering
- Mixed LTR/RTL content handling

## Contributing

1. Fork the repository
2. Create your feature branch
3. Commit your changes
4. Push to the branch
5. Create a Pull Request

## License

This project is licensed under the MIT License.
