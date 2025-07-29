# Lancaster Community Platform - Seed Data

## Overview

The seed data script populates the database with realistic Lancaster-themed content including:

- **12 published posts** in multiple languages (English, Spanish, Arabic, French)
- **3 unpublished posts** awaiting review
- **4 published events** 
- **1 unpublished event** awaiting review
- **Translations** for all published content

## Running the Seed Script

From the backend directory:

```bash
cd backend
cargo run --bin seed
```

This will:
1. Clear existing posts and events
2. Ensure supported languages are configured
3. Create posts with different original languages
4. Add translations for all published posts
5. Create upcoming events
6. Add translations for all published events

## Content Details

### Posts with Non-English Origins

1. **Spanish Posts** (by Maria Hernandez & Carlos Rodriguez)
   - Digital literacy program for seniors
   - International food festival announcement

2. **Arabic Posts** (by أحمد حسن & فاطمة الزهراء)
   - Refugee support program
   - Arabic cooking classes

3. **French Posts** (by Jean-Pierre Dubois & Marie Laurent)
   - Free French classes at the library
   - Monthly flea market

### Features Demonstrated

- **Infinite Scroll**: The news feed loads 10 posts at a time
- **Original + Translation Display**: Posts show both original language and translation
- **RTL Support**: Arabic posts display with proper right-to-left formatting
- **Mixed Languages**: Feed contains posts from various language communities

## Testing the Features

1. Visit http://localhost:5173
2. Switch between languages using the language selector
3. Notice how posts show:
   - Original content (if different from current language)
   - Translated content below
   - Proper text direction for RTL languages
4. Scroll down to trigger infinite loading
5. Check the admin panel to see unpublished content awaiting review