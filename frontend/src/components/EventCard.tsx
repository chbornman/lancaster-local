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
    <article className="bg-white shadow-sm border border-[#1A1A1A] p-6 hover:shadow-md transition-all group">
      <div className="flex flex-col gap-2 mb-3">
        <div className="flex flex-col sm:flex-row sm:flex-row-reverse sm:justify-between gap-2">
          <div className="flex flex-col">
            <div className="text-sm font-bold text-[#1A1A1A] uppercase tracking-wide" style={{ fontFamily: 'var(--font-sans)' }}>
              {formatDate(event.event_date)}
              {event.event_time && (
                <span className="ms-2 text-[#8B4513]">
                  {formatTime(event.event_time)}
                </span>
              )}
            </div>
            <div className="text-xs text-[#4A4A4A] mt-1 uppercase tracking-wide" style={{ fontFamily: 'var(--font-sans)' }}>
              {getCategoryTranslation(event.category)}
            </div>
          </div>

          {event.is_translated && (
            <div className="flex items-center gap-1 text-xs text-[#8B4513] font-medium">
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
        <div className="mb-3">
          <div className="text-xs text-gray-500 mb-1">{t('language.original', { language: event.original_language })}</div>
          <h3 
            className="text-base sm:text-lg font-semibold mb-2 text-gray-600"
            dir={event.original_text_direction}
            style={{ 
              fontFamily: isOriginalRTL ? 'Noto Sans Arabic, Tahoma, Arial, sans-serif' : 'Inter, -apple-system, sans-serif',
              textAlign: isOriginalRTL ? 'right' : 'left'
            }}
          >
            {event.original_title}
          </h3>
        </div>
      )}

      <h3 
        className="text-xl sm:text-2xl font-bold mb-3 leading-tight group-hover:text-[#8B4513] transition-colors"
        dir={event.text_direction}
        style={{ 
          fontFamily: isRTL ? 'Noto Sans Arabic, Tahoma, Arial, sans-serif' : 'var(--font-serif)',
          textAlign: isRTL ? 'right' : 'left'
        }}
      >
        {event.title}
      </h3>

      {showOriginal && event.original_description && (
        <p 
          className="text-xs sm:text-sm text-gray-600 mb-2 whitespace-pre-wrap line-clamp-2"
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

      {event.description && (
        <p 
          className="text-sm sm:text-base text-[#1A1A1A] mb-4 whitespace-pre-wrap line-clamp-3 sm:line-clamp-none leading-relaxed"
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
        <div className="flex items-center gap-2 mb-3 text-sm text-[#4A4A4A] font-medium" dir="auto" style={{ fontFamily: 'var(--font-sans)' }}>
          <svg className="w-4 h-4 flex-shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} 
              d="M17.657 16.657L13.414 20.9a1.998 1.998 0 01-2.827 0l-4.244-4.243a8 8 0 1111.314 0z" />
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} 
              d="M15 11a3 3 0 11-6 0 3 3 0 016 0z" />
          </svg>
          <span className="truncate">{event.location}</span>
        </div>
      )}

      <div className="flex flex-col sm:flex-row sm:flex-row-reverse sm:justify-between gap-3 items-start sm:items-center mt-4 pt-4 border-t border-gray-200">
        <div className="flex items-center gap-2">
          {event.is_free ? (
            <span className="inline-block px-3 py-1 bg-[#8B4513] text-white text-xs sm:text-sm uppercase tracking-wide font-medium" style={{ fontFamily: 'var(--font-sans)' }}>
              {t('events.is_free')}
            </span>
          ) : event.ticket_price ? (
            <span className="inline-block px-3 py-1 bg-white border border-[#1A1A1A] text-[#1A1A1A] text-xs sm:text-sm font-bold" style={{ fontFamily: 'var(--font-sans)' }}>
              ${event.ticket_price}
            </span>
          ) : null}
        </div>

        {event.ticket_url && (
          <a 
            href={event.ticket_url}
            target="_blank"
            rel="noopener noreferrer"
            className="px-4 py-2 bg-[#8B4513] text-white hover:bg-[#6B3410] transition-all text-center w-full sm:w-auto text-sm font-medium shadow-sm hover:shadow-md"
            style={{ fontFamily: 'var(--font-sans)' }}
          >
            {t('events.get_tickets')}
          </a>
        )}
      </div>

      <div className="text-xs text-[#4A4A4A] mt-3 uppercase tracking-wide" style={{ fontFamily: 'var(--font-sans)' }}>
        {t('events.organized_by')}: 
        <span className="font-bold ms-1" dir="auto">
          {event.organizer_name}
        </span>
      </div>
    </article>
  );
};

export default EventCard;