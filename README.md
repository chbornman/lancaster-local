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

## Development

### Prerequisites

- Docker and Docker Compose
- Make (optional, for convenience commands)
- Google Cloud Translation API key

### Environment Variables

Copy `.env.example` to `.env` and configure:

- `GOOGLE_TRANSLATE_API_KEY` - Your Google Translate API key
- `ADMIN_PASSWORD` - Admin panel password
- `POSTGRES_USER/PASSWORD/DB` - Database credentials (defaults provided for dev)
- Other settings have sensible defaults for development

**Note**: The development `docker-compose.yml` uses these `.env` values. Production uses GitHub Secrets.

### Available Commands

```bash
make up         # Start all services
make down       # Stop all services
make build      # Build all services
make logs       # View all logs
make clean      # Clean up containers and volumes
make restart    # Restart all services
```

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
