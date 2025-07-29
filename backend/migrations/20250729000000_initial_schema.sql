-- Supported languages configuration with RTL info
CREATE TABLE supported_languages (
    code VARCHAR(10) PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    native_name VARCHAR(100) NOT NULL,
    is_rtl BOOLEAN DEFAULT false,
    text_direction VARCHAR(10) DEFAULT 'ltr',
    enabled BOOLEAN DEFAULT true,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Insert default supported languages with RTL info
INSERT INTO supported_languages (code, name, native_name, is_rtl, text_direction) VALUES
('en', 'English', 'English', false, 'ltr'),
('es', 'Spanish', 'Español', false, 'ltr'),
('de', 'German', 'Deutsch', false, 'ltr'),
('fr', 'French', 'Français', false, 'ltr'),
('zh', 'Chinese', '中文', false, 'ltr'),
('ar', 'Arabic', 'العربية', true, 'rtl'),
('he', 'Hebrew', 'עברית', true, 'rtl'),
('fa', 'Persian', 'فارسی', true, 'rtl'),
('ur', 'Urdu', 'اردو', true, 'rtl');

-- Posts table with RTL support
CREATE TABLE posts (
    id SERIAL PRIMARY KEY,
    author_name VARCHAR(255) NOT NULL,
    author_email VARCHAR(255),
    title VARCHAR(255) NOT NULL,
    content TEXT,
    link_url VARCHAR(500),
    image_url VARCHAR(500),
    post_type VARCHAR(50) DEFAULT 'text',
    original_language VARCHAR(10) DEFAULT 'en',
    text_direction VARCHAR(10) DEFAULT 'ltr',
    published BOOLEAN DEFAULT false,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Post translations table with RTL info
CREATE TABLE post_translations (
    id SERIAL PRIMARY KEY,
    post_id INTEGER REFERENCES posts(id) ON DELETE CASCADE,
    language_code VARCHAR(10) NOT NULL,
    title VARCHAR(255) NOT NULL,
    content TEXT,
    text_direction VARCHAR(10) DEFAULT 'ltr',
    translated_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(post_id, language_code)
);

-- Events table with RTL support
CREATE TABLE events (
    id SERIAL PRIMARY KEY,
    organizer_name VARCHAR(255) NOT NULL,
    organizer_email VARCHAR(255),
    title VARCHAR(255) NOT NULL,
    description TEXT,
    event_date DATE NOT NULL,
    event_time TIME,
    location VARCHAR(255),
    category VARCHAR(100),
    is_free BOOLEAN DEFAULT true,
    ticket_price DECIMAL(10,2),
    ticket_url VARCHAR(500),
    original_language VARCHAR(10) DEFAULT 'en',
    text_direction VARCHAR(10) DEFAULT 'ltr',
    published BOOLEAN DEFAULT false,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Event translations table with RTL info
CREATE TABLE event_translations (
    id SERIAL PRIMARY KEY,
    event_id INTEGER REFERENCES events(id) ON DELETE CASCADE,
    language_code VARCHAR(10) NOT NULL,
    title VARCHAR(255) NOT NULL,
    description TEXT,
    text_direction VARCHAR(10) DEFAULT 'ltr',
    translated_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(event_id, language_code)
);

-- Admin sessions
CREATE TABLE admin_sessions (
    id SERIAL PRIMARY KEY,
    session_token VARCHAR(255) UNIQUE NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    expires_at TIMESTAMPTZ NOT NULL
);

-- Create indexes
CREATE INDEX idx_posts_published ON posts(published);
CREATE INDEX idx_posts_created_at ON posts(created_at DESC);
CREATE INDEX idx_events_published ON events(published);
CREATE INDEX idx_events_date ON events(event_date);
CREATE INDEX idx_post_translations_post_id ON post_translations(post_id);
CREATE INDEX idx_event_translations_event_id ON event_translations(event_id);