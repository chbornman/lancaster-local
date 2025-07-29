-- Remove ticket_price column from events table
ALTER TABLE events DROP COLUMN IF EXISTS ticket_price;