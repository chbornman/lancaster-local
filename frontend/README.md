# Lancaster Community Platform - Frontend

A multilingual community platform for Lancaster that allows community members to share news, events, and information in multiple languages.

## Features

- **Multilingual Support**: Full support for English, Arabic, Spanish, French, German, and Chinese
- **RTL/LTR Support**: Automatic layout adjustment for right-to-left languages
- **Community News Feed**: Share and discover local news and announcements
- **Event Calendar**: Browse and submit community events
- **Responsive Design**: Mobile-first design that works on all devices
- **Admin Dashboard**: Moderation tools for managing content

## Tech Stack

- **React 18** with TypeScript
- **Vite** for fast development and building
- **React Router** for client-side routing
- **i18next** for internationalization
- **Tailwind CSS** for styling
- **React Helmet Async** for SEO management

## Prerequisites

- Node.js 18+ and npm
- Backend API running (see backend repository)

## Installation

1. Clone the repository:
```bash
git clone <repository-url>
cd frontend
```

2. Install dependencies:
```bash
npm install
```

3. Create a `.env` file based on `.env.example`:
```bash
cp .env.example .env
```

4. Configure environment variables:
```env
VITE_API_URL=http://localhost:8000
VITE_RTL_LANGUAGES=ar,he,fa,ur
```

## Development

Run the development server:
```bash
npm run dev
```

The application will be available at `http://localhost:5173`

## Building for Production

Build the application:
```bash
npm run build
```

Preview the production build:
```bash
npm run preview
```

## Project Structure

```
src/
├── components/       # Reusable UI components
├── hooks/           # Custom React hooks
├── i18n/            # Internationalization configuration
│   └── locales/     # Translation files for each language
├── pages/           # Page components
├── styles/          # Global styles and CSS modules
├── types/           # TypeScript type definitions
├── utils/           # Utility functions and API client
├── App.tsx          # Main application component
└── main.tsx         # Application entry point
```

## Available Scripts

- `npm run dev` - Start development server
- `npm run build` - Build for production
- `npm run preview` - Preview production build
- `npm run lint` - Run ESLint

## Key Components

### Pages
- **HomePage**: Landing page with overview of news and events
- **NewsFeed**: Browse all community posts
- **Calendar**: View and filter community events
- **SubmitPost**: Form to submit news/announcements
- **SubmitEvent**: Form to submit events
- **AdminDashboard**: Content moderation interface

### Features
- **Language Selector**: Switch between supported languages
- **Direction Support**: Automatic RTL/LTR layout switching
- **Infinite Scroll**: Load more content as you scroll
- **Responsive Layout**: Mobile-optimized navigation and content

## API Integration

The frontend communicates with the backend API for all data operations. The API client is configured in `src/utils/api.ts` and includes:

- Post management (create, read, update status)
- Event management (create, read, update status)
- Admin authentication
- Content moderation endpoints

## Internationalization

The application uses i18next for translations. Translation files are located in `src/i18n/locales/`. To add a new language:

1. Create a new JSON file in `src/i18n/locales/`
2. Add the language code to the language selector
3. If RTL, add to `VITE_RTL_LANGUAGES` environment variable

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run linting: `npm run lint`
5. Submit a pull request

## License

[Add your license information here]