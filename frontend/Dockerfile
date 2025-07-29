# Build stage
FROM node:20-alpine as builder

WORKDIR /app

# Copy package files
COPY package*.json ./

# Install dependencies
RUN npm ci

# Copy source code
COPY . .

# Accept build arguments with defaults
ARG VITE_API_URL=/api
ARG VITE_DEFAULT_LANGUAGE=en
ARG VITE_RTL_LANGUAGES=ar,he,fa,ur

# Set environment variables from build arguments
ENV VITE_API_URL=$VITE_API_URL
ENV VITE_DEFAULT_LANGUAGE=$VITE_DEFAULT_LANGUAGE
ENV VITE_RTL_LANGUAGES=$VITE_RTL_LANGUAGES

# Build for production
RUN npm run build

# Production stage
FROM nginx:alpine

# Copy built app from builder stage
COPY --from=builder /app/dist /usr/share/nginx/html

# Copy nginx configuration
COPY nginx.conf /etc/nginx/conf.d/default.conf

# Create non-root user
RUN addgroup -g 1001 -S appuser && \
    adduser -S appuser -u 1001

EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]