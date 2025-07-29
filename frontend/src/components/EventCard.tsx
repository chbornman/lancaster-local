import type React from 'react';
import { useTranslation } from 'react-i18next';
import { format } from 'date-fns';
import { ar, de, es, fr, zhCN } from 'date-fns/locale';
import type { EventWithTranslation } from '../types/api';
import type { Locale } from 'date-fns';

const locales: Record<string, Locale | undefined> = {
  en: undefined,
  ar: ar,
  de: de,
  es: es,
  fr: fr,
  zh: zhCN,
};

interface EventCardProps {
  event: EventWithTranslation;
}

const EventCard: React.FC<EventCardProps> = ({ event }) => {
  const { t, i18n } = useTranslation();
  
  // Type-safe category translation
  const getCategoryTranslation = (category?: string): string => {
    const categoryKey = category || 'community';
    const validCategories = ['community', 'education', 'social', 'sports', 'cultural', 'religious'] as const;
    type ValidCategory = typeof validCategories[number];
    
    if (validCategories.includes(categoryKey as ValidCategory)) {
      // Type-safe translation key construction
      const translationKey = `events.categories.${categoryKey}` as const;
      return t(translationKey);
    }
    return categoryKey; // Fallback to raw category if not found
  };
  const isRTL = event.text_direction === 'rtl';
  const isOriginalRTL = event.original_text_direction === 'rtl';
  const locale = locales[i18n.language] ?? undefined;
  const showOriginal = event.is_translated && event.original_language !== i18n.language;

  const formatDate = (dateString: string): string => {
    const date = new Date(dateString);
    return format(date, 'PPP', locale ? { locale } : undefined);
  };

  const formatTime = (timeString?: string): string => {
    if (!timeString) return '';
    const [hours, minutes] = timeString.split(':');
    const date = new Date();
    date.setHours(parseInt(hours), parseInt(minutes));
    return format(date, 'p', locale ? { locale } : undefined);
  };

  return (
    <article className="card card-hover rounded-lg shadow-sm border border-neutral-200 relative overflow-hidden">
      <div className="absolute top-0 left-0 w-2 h-full bg-gradient-to-b from-primary-600 to-primary-400"></div>
      <div className="flex flex-col gap-2 mb-3">
        <div className="flex flex-col sm:flex-row sm:flex-row-reverse sm:justify-between gap-2">
          <div className="flex flex-col">
            <div className="text-sm font-bold text-neutral-900 uppercase tracking-wide" style={{ fontFamily: 'var(--font-sans)' }}>
              {formatDate(event.event_date)}
              {event.event_time && (
                <span className="ms-2 text-primary-600">
                  {formatTime(event.event_time)}
                </span>
              )}
            </div>
            <div className="inline-flex mt-1">
              <span className="px-2 py-1 bg-primary-100 text-primary-700 text-xs font-medium rounded-md uppercase tracking-wide" style={{ fontFamily: 'var(--font-sans)' }}>
                {getCategoryTranslation(event.category)}
              </span>
            </div>
          </div>

          {event.is_translated && (
            <div className="flex items-center gap-1 text-xs text-primary-600 font-medium">
              <svg className="w-4 h-4 flex-shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} 
                  d="M3 5h12M9 3v2m1.048 9.5A18.022 18.022 0 016.412 9m6.088 9h7M11 21l5-10 5 10M12.751 5C11.783 10.77 8.07 15.61 3 18.129" />
              </svg>
              <span className="hidden sm:inline uppercase tracking-wide" style={{ fontFamily: 'var(--font-sans)' }}>{t('language.translated', { language: event.original_language })}</span>
              <span className="sm:hidden uppercase tracking-wide" style={{ fontFamily: 'var(--font-sans)' }}>{event.original_language.toUpperCase()}</span>
            </div>
          )}
        </div>
      </div>

      {showOriginal && (
        <div className="bg-neutral-50 border-l-4 border-primary-600/30 p-4 mb-4 rounded-r">
          <div className="text-xs font-semibold text-neutral-600 mb-2 uppercase tracking-wide flex items-center gap-1">
            <svg className="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M3 5h12M9 3v2m1.048 9.5A18.022 18.022 0 016.412 9m6.088 9h7M11 21l5-10 5 10M12.751 5C11.783 10.77 8.07 15.61 3 18.129" />
            </svg>
            {t('language.original', { language: event.original_language })}
          </div>
          <h3 
            className="text-base sm:text-lg font-semibold mb-2 text-neutral-700"
            dir={event.original_text_direction}
            style={{ 
              fontFamily: isOriginalRTL ? 'Noto Sans Arabic, Tahoma, Arial, sans-serif' : 'Inter, -apple-system, sans-serif',
              textAlign: isOriginalRTL ? 'right' : 'left'
            }}
          >
            {event.original_title}
          </h3>
          {event.original_description && (
            <p 
              className="text-xs sm:text-sm text-neutral-600 whitespace-pre-wrap line-clamp-2"
              dir={event.original_text_direction}
              style={{ 
                fontFamily: isOriginalRTL ? 'Noto Sans Arabic, Tahoma, Arial, sans-serif' : 'Inter, -apple-system, sans-serif',
                textAlign: isOriginalRTL ? 'right' : 'left',
                lineHeight: isOriginalRTL ? '1.8' : '1.6'
              }}
            >
              {event.original_description}
            </p>
          )}
        </div>
      )}

      {showOriginal && (
        <div className="mb-2">
          <span className="inline-flex items-center gap-1 text-xs font-semibold text-primary-600 uppercase tracking-wide">
            <svg className="w-3 h-3" fill="currentColor" viewBox="0 0 20 20">
              <path fillRule="evenodd" d="M9 2a1 1 0 000 2h2a1 1 0 100-2H9z" clipRule="evenodd" />
              <path fillRule="evenodd" d="M4 5a2 2 0 012-2 1 1 0 000 2H6a2 2 0 00-2 2v6a2 2 0 002 2h2a1 1 0 100-2H6V5z" clipRule="evenodd" />
            </svg>
            {t('language.translation')}
          </span>
        </div>
      )}

      <h3 
        className="text-xl sm:text-2xl font-bold mb-3 leading-tight group-hover:text-primary-600 transition-colors"
        dir={event.text_direction}
        style={{ 
          fontFamily: isRTL ? 'Noto Sans Arabic, Tahoma, Arial, sans-serif' : 'var(--font-serif)',
          textAlign: isRTL ? 'right' : 'left'
        }}
      >
        {event.title}
      </h3>

      {event.description && (
        <p 
          className="text-sm sm:text-base text-neutral-700 mb-4 whitespace-pre-wrap line-clamp-3 sm:line-clamp-none leading-relaxed"
          dir={event.text_direction}
          style={{ 
            fontFamily: isRTL ? 'Noto Sans Arabic, Tahoma, Arial, sans-serif' : 'var(--font-body)',
            textAlign: isRTL ? 'right' : 'left',
            lineHeight: isRTL ? '1.8' : '1.7'
          }}
        >
          {event.description}
        </p>
      )}

      {event.location && (
        <div className="flex items-center gap-2 mb-3 text-sm text-neutral-600 font-medium" dir="auto" style={{ fontFamily: 'var(--font-sans)' }}>
          <svg className="w-4 h-4 flex-shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} 
              d="M17.657 16.657L13.414 20.9a1.998 1.998 0 01-2.827 0l-4.244-4.243a8 8 0 1111.314 0z" />
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} 
              d="M15 11a3 3 0 11-6 0 3 3 0 016 0z" />
          </svg>
          <span className="truncate">{event.location}</span>
        </div>
      )}

      <div className="flex flex-col sm:flex-row sm:flex-row-reverse sm:justify-between gap-3 items-start sm:items-center mt-4 pt-4 border-t border-neutral-200">
        <div className="flex items-center gap-2">
          {event.is_free ? (
            <span className="inline-block px-3 py-1 bg-primary-600 text-white text-xs sm:text-sm uppercase tracking-wide font-medium rounded-full" style={{ fontFamily: 'var(--font-sans)' }}>
              {t('events.is_free')}
            </span>
          ) : event.ticket_price ? (
            <span className="inline-block px-3 py-1 bg-white border-2 border-primary-600 text-primary-700 text-xs sm:text-sm font-bold rounded-md" style={{ fontFamily: 'var(--font-sans)' }}>
              ${event.ticket_price}
            </span>
          ) : null}
        </div>

        {event.ticket_url && (
          <a 
            href={event.ticket_url}
            target="_blank"
            rel="noopener noreferrer"
            className="btn btn-primary btn-md w-full sm:w-auto"
            style={{ fontFamily: 'var(--font-sans)' }}
          >
            {t('events.get_tickets')}
          </a>
        )}
      </div>

      <div className="text-xs text-neutral-600 mt-3 uppercase tracking-wide" style={{ fontFamily: 'var(--font-sans)' }}>
        {t('events.organized_by')}: 
        <span className="font-bold ms-1" dir="auto">
          {event.organizer_name}
        </span>
      </div>
    </article>
  );
};

export default EventCard;