# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Common Development Commands

### Development Environment
```bash
# Start all services (including database migrations and seed data)
make up  # or: docker compose up -d

# View logs
make logs  # or: docker compose logs -f

# Stop services
make down  # or: docker compose down

# Rebuild after code changes
make build  # or: docker compose build

# Clean everything (including volumes)
make clean  # or: docker compose down -v

# Restart services
make restart  # or: docker compose restart
```

### Frontend Commands
```bash
# Development server (inside container)
npm run dev

# Build for production
npm run build

# Lint TypeScript/React code
npm run lint

# Fix linting issues
npm run lint:fix
```

### Backend Commands
```bash
# Run backend (inside container with hot-reload)
cargo watch -x run

# Run migrations manually
cargo run --bin migrate

# Run seed data
cargo run --bin seed

# Build for production
cargo build --release
```

### Production Deployment
```bash
# View production logs
docker compose -f docker-compose.prod.yml logs -f

# Run migrations manually (usually automatic)
docker compose -f docker-compose.prod.yml run --rm migrate

# Run seed data (first time only)
docker compose -f docker-compose.prod.yml run --rm seed
```

## High-Level Architecture

### System Overview
Lancaster Local is a multi-language community platform with a React frontend and Rust backend, designed for community news and events with full RTL language support.

### Backend Architecture (Rust/Axum)
- **Framework**: Axum web framework with Tokio async runtime
- **Database**: PostgreSQL with SQLx for type-safe queries
- **Cache**: Redis for translation caching
- **Authentication**: Token-based admin authentication stored in Redis
- **Translation**: Google Translate API integration with caching

Key modules:
- `handlers/`: HTTP request handlers organized by feature (posts, events, admin, languages)
- `models/`: Database models and request/response types
- `services/`: Business logic including translation service and background tasks
- `db/`: Database connection pool and app state management

### Frontend Architecture (React/TypeScript)
- **Build Tool**: Vite for fast development and optimized production builds
- **Routing**: React Router v6 with nested routes
- **Internationalization**: i18next with automatic language detection
- **Styling**: Tailwind CSS with RTL-aware utilities
- **State Management**: React hooks and contexts (Direction context for RTL/LTR)

Key components:
- `pages/`: Route components (NewsFeed, Calendar, Admin, Submit forms)
- `components/`: Reusable UI components with RTL support
- `hooks/`: Custom hooks for admin auth, direction handling, infinite scroll
- `contexts/`: Global state contexts (DirectionContext for RTL/LTR)
- `i18n/`: Translation configuration and locale files

### Database Schema
The system uses PostgreSQL with the following core tables:
- `posts`: Community posts with multilingual support
- `post_translations`: Cached translations of posts
- `events`: Community events with date/time/location
- `event_translations`: Cached translations of events
- `supported_languages`: Language configuration with RTL flags
- `admin_sessions`: Token-based admin authentication

### Multi-Language & RTL Support
- Supports 9 languages: English, Spanish, German, French, Chinese, Arabic, Hebrew, Persian, Urdu
- RTL languages (Arabic, Hebrew, Persian, Urdu) have special handling:
  - Automatic text direction switching
  - RTL-aware CSS classes
  - Bidirectional text layout
- Translation flow:
  1. Content submitted in original language
  2. Google Translate API called for each supported language
  3. Translations cached in database and Redis
  4. Client requests content in preferred language

### API Design
RESTful API with clear separation:
- Public endpoints: `/api/posts`, `/api/events`, `/api/languages`
- Admin endpoints: `/api/admin/*` (require authentication token)
- All endpoints support `lang` query parameter for content language
- Automatic translation happens server-side during content creation

### Deployment Architecture
- Development: Local Docker Compose with hot-reloading
- Production: GitHub Actions CI/CD pipeline
  - Builds Docker images on push to main
  - Pushes to GitHub Container Registry
  - SSH deployment to VPS
  - Automatic migration execution
  - Environment variables from GitHub Secrets