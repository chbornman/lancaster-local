-- Convert all TIMESTAMP columns to TIMESTAMPTZ

-- supported_languages table
ALTER TABLE supported_languages 
ALTER COLUMN created_at TYPE TIMESTAMPTZ USING created_at AT TIME ZONE 'UTC';

-- posts table
ALTER TABLE posts 
ALTER COLUMN created_at TYPE TIMESTAMPTZ USING created_at AT TIME ZONE 'UTC',
ALTER COLUMN updated_at TYPE TIMESTAMPTZ USING updated_at AT TIME ZONE 'UTC';

-- post_translations table
ALTER TABLE post_translations 
ALTER COLUMN translated_at TYPE TIMESTAMPTZ USING translated_at AT TIME ZONE 'UTC';

-- events table  
ALTER TABLE events
ALTER COLUMN created_at TYPE TIMESTAMPTZ USING created_at AT TIME ZONE 'UTC',
ALTER COLUMN updated_at TYPE TIMESTAMPTZ USING updated_at AT TIME ZONE 'UTC';

-- event_translations table
ALTER TABLE event_translations
ALTER COLUMN translated_at TYPE TIMESTAMPTZ USING translated_at AT TIME ZONE 'UTC';

-- admin_sessions table
ALTER TABLE admin_sessions
ALTER COLUMN created_at TYPE TIMESTAMPTZ USING created_at AT TIME ZONE 'UTC',
ALTER COLUMN expires_at TYPE TIMESTAMPTZ USING expires_at AT TIME ZONE 'UTC';