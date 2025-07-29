.PHONY: help up down build logs backend-logs frontend-logs db-logs clean restart

help:
	@echo "Available commands:"
	@echo "  make up         - Start all services"
	@echo "  make down       - Stop all services"
	@echo "  make build      - Build all services"
	@echo "  make logs       - View all logs"
	@echo "  make clean      - Clean up containers and volumes"
	@echo "  make restart    - Restart all services"

up:
	docker compose up -d

down:
	docker compose down

build:
	docker compose build

logs:
	docker compose logs -f

backend-logs:
	docker compose logs -f backend

frontend-logs:
	docker compose logs -f frontend

db-logs:
	docker compose logs -f db

clean:
	docker compose down -v
	docker system prune -f

restart:
	docker compose restart

# Development shortcuts
dev-backend:
	cd backend && cargo run

dev-frontend:
	cd frontend && npm run dev

install-frontend:
	cd frontend && npm install

install-backend:
	cd backend && cargo build