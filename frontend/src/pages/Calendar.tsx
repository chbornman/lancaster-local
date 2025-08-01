import type React from 'react';
import { useState, useEffect, useCallback } from 'react';
import { Link } from 'react-router-dom';
import { useTranslation } from 'react-i18next';
import { format, startOfMonth, endOfMonth, eachDayOfInterval, getDay, isSameMonth, isSameDay, addMonths, subMonths } from 'date-fns';
import { ar, de, es, fr, zhCN } from 'date-fns/locale';
import EventCard from '../components/EventCard';
import { endpoints } from '../utils/api';
import type { EventWithTranslation } from '../types/api';
import type { Locale } from 'date-fns';
import axios from 'axios';

const locales: Record<string, Locale | undefined> = {
  en: undefined,
  ar: ar,
  de: de,
  es: es,
  fr: fr,
  zh: zhCN,
};

const Calendar: React.FC = () => {
  const { t, i18n } = useTranslation();
  const [events, setEvents] = useState<EventWithTranslation[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [currentMonth, setCurrentMonth] = useState(new Date());
  const [selectedDate, setSelectedDate] = useState<Date | null>(null);
  const [category, setCategory] = useState('');
  const [viewMode, setViewMode] = useState<'calendar' | 'list'>('calendar');
  
  const isRTL = ['ar', 'he', 'fa', 'ur'].includes(i18n.language);
  const locale = locales[i18n.language] ?? undefined;

  const fetchEvents = useCallback(async () => {
    try {
      setLoading(true);
      setError(null);
      const response = await endpoints.getEvents({
        lang: i18n.language,
        month: format(currentMonth, 'yyyy-MM'),
        ...(category && { category })
      });
      setEvents(response.data.events);
    } catch (err) {
      console.error('Failed to fetch events:', err);
      setError(axios.isAxiosError(err) ? err.message : 'Failed to fetch events');
    } finally {
      setLoading(false);
    }
  }, [i18n.language, currentMonth, category]);

  useEffect(() => {
    void fetchEvents();
  }, [fetchEvents]);

  const navigateMonth = (direction: number) => {
    if (direction === -1) {
      setCurrentMonth(subMonths(currentMonth, 1));
    } else {
      setCurrentMonth(addMonths(currentMonth, 1));
    }
  };

  const getDaysInMonth = (): (Date | null)[] => {
    const start = startOfMonth(currentMonth);
    const end = endOfMonth(currentMonth);
    const days = eachDayOfInterval({ start, end });
    
    // Add padding days from previous month
    const startDayOfWeek = getDay(start);
    const paddingDays: null[] = new Array<null>(startDayOfWeek).fill(null);
    
    return [...paddingDays, ...days];
  };

  const getEventsForDate = (date: Date | null) => {
    if (!date) return [];
    return events.filter(event => 
      isSameDay(new Date(event.event_date), date)
    );
  };

  const renderCalendarView = () => {
    const days = getDaysInMonth();
    const weekDays = isRTL ? 
      ['س', 'ج', 'خ', 'ث', 'ر', 'ن', 'ح'] : // Arabic weekdays
      ['S', 'M', 'T', 'W', 'T', 'F', 'S'];

    return (
      <div className="card p-2 sm:p-4 overflow-x-auto relative">
        <div className="absolute top-0 left-0 w-full h-1 bg-gradient-to-r from-primary-600 to-primary-500"></div>
        <div className="min-w-[280px]">
          <div className="grid grid-cols-7 gap-1 mb-2">
            {weekDays.map((day, index) => (
              <div 
                key={index} 
                className="text-center text-xs sm:text-sm font-bold text-neutral-800 py-1 sm:py-2 uppercase"
                style={{ fontFamily: 'var(--font-sans)' }}
              >
                {day}
              </div>
            ))}
          </div>
          
          <div className="grid grid-cols-7 gap-1">
            {days.map((day, index) => {
              const dayEvents = day ? getEventsForDate(day) : [];
              const isToday = day && isSameDay(day, new Date());
              const isSelected = day && selectedDate && isSameDay(day, selectedDate);
              const isCurrentMonth = day && isSameMonth(day, currentMonth);
              
              return (
                <div
                  key={index}
                  onClick={() => day && setSelectedDate(day)}
                  className={`
                    min-h-[50px] sm:min-h-[80px] p-1 sm:p-2 border rounded-lg cursor-pointer transition-all duration-200
                    ${!day ? 'invisible' : ''}
                    ${!isCurrentMonth ? 'text-gray-400' : ''}
                    ${isToday ? 'bg-primary-100 border-primary-600 border-2 shadow-sm' : 'border-neutral-200'}
                    ${isSelected ? 'bg-primary-200 border-primary-600 border-2 shadow-md' : ''}
                    ${day ? 'hover:bg-neutral-50 hover:border-neutral-300 hover:shadow-sm' : ''}
                  `}
                >
                  {day && (
                    <>
                      <div className="font-bold text-xs sm:text-sm mb-1" style={{ fontFamily: 'var(--font-serif)' }}>
                        {format(day, 'd', locale ? { locale } : undefined)}
                      </div>
                      {dayEvents.length > 0 && (
                        <div className="hidden sm:block space-y-1">
                          {dayEvents.slice(0, 2).map((event) => (
                            <div 
                              key={event.id}
                              className="text-xs p-1 bg-primary-100 text-primary-700 rounded-md truncate font-medium"
                              style={{ fontFamily: 'var(--font-sans)' }}
                              title={event.title}
                            >
                              {event.title}
                            </div>
                          ))}
                          {dayEvents.length > 2 && (
                            <div className="text-xs text-neutral-600 font-medium" style={{ fontFamily: 'var(--font-sans)' }}>
                              +{dayEvents.length - 2}
                            </div>
                          )}
                        </div>
                      )}
                      {/* Mobile indicator */}
                      {dayEvents.length > 0 && (
                        <div className="sm:hidden">
                          <div className="w-2 h-2 bg-primary-600 rounded-full mx-auto"></div>
                        </div>
                      )}
                    </>
                  )}
                </div>
              );
            })}
          </div>
        </div>
      </div>
    );
  };

  if (loading) {
    return (
      <div className="flex justify-center items-center min-h-[400px]">
        <div className="spinner"></div>
      </div>
    );
  }

  if (error) {
    return (
      <div className="text-center py-8">
        <p className="text-red-600 mb-4 font-medium">{t('common.error')}: {error}</p>
        <button 
          onClick={fetchEvents}
          className="btn btn-primary btn-md"
        >
          {t('common.retry')}
        </button>
      </div>
    );
  }

  return (
    <div className="max-w-6xl mx-auto">
      <div className="flex flex-col sm:flex-row sm:flex-row-reverse sm:justify-between sm:items-center gap-4 mb-8 pb-4 border-b-2 border-neutral-800">
        <h1 className="text-3xl sm:text-4xl font-bold" style={{ fontFamily: 'var(--font-serif)' }}>{t('events.title')}</h1>
        <Link to="/submit/event" className="btn btn-primary btn-md w-full sm:w-auto text-center">
          {t('events.submit')}
        </Link>
      </div>

      {/* Calendar Navigation */}
      <div className="flex flex-col gap-4 mb-6">
        {/* Month Navigation */}
        <div className="flex items-center justify-center gap-2 sm:gap-4">
          <button
            onClick={() => navigateMonth(isRTL ? 1 : -1)}
            className="btn btn-secondary btn-md"
          >
            {isRTL ? '←' : '→'}
          </button>
          
          <h2 className="text-lg sm:text-xl font-bold min-w-[140px] text-center" style={{ fontFamily: 'var(--font-serif)' }}>
            {format(currentMonth, 'MMM yyyy', locale ? { locale } : undefined)}
          </h2>
          
          <button
            onClick={() => navigateMonth(isRTL ? -1 : 1)}
            className="btn btn-secondary btn-md"
          >
            {isRTL ? '→' : '←'}
          </button>
        </div>

        {/* Filters and View Mode */}
        <div className="flex flex-col sm:flex-row gap-3 sm:justify-center">
          <select
            value={category}
            onChange={(e) => setCategory(e.target.value)}
            className="select w-full sm:w-auto"
          >
            <option value="">{t('events.all_categories')}</option>
            <option value="community">{t('events.categories.community')}</option>
            <option value="education">{t('events.categories.education')}</option>
            <option value="social">{t('events.categories.social')}</option>
            <option value="sports">{t('events.categories.sports')}</option>
            <option value="cultural">{t('events.categories.cultural')}</option>
            <option value="religious">{t('events.categories.religious')}</option>
          </select>

          <div className="flex overflow-hidden border border-neutral-300 rounded-lg w-full sm:w-auto shadow-sm">
            <button
              onClick={() => setViewMode('calendar')}
              className={`flex-1 sm:flex-initial px-3 sm:px-4 py-2 text-sm sm:text-base font-medium transition-all duration-200 ${
                viewMode === 'calendar' ? 'bg-primary-600 text-white' : 'bg-white text-neutral-800 hover:bg-neutral-50'
              }`}
              style={{ fontFamily: 'var(--font-sans)' }}
            >
              {t('events.calendar_view')}
            </button>
            <button
              onClick={() => setViewMode('list')}
              className={`flex-1 sm:flex-initial px-3 sm:px-4 py-2 text-sm sm:text-base font-medium transition-all duration-200 border-l border-neutral-300 ${
                viewMode === 'list' ? 'bg-primary-600 text-white' : 'bg-white text-neutral-800 hover:bg-neutral-50'
              }`}
              style={{ fontFamily: 'var(--font-sans)' }}
            >
              {t('events.list_view')}
            </button>
          </div>
        </div>
      </div>

      {/* Calendar or List View */}
      {viewMode === 'calendar' ? (
        <>
          {renderCalendarView()}
          
          {/* Selected Date Events */}
          {selectedDate && (
            <div className="mt-6">
              <h3 className="text-lg font-bold mb-4" style={{ fontFamily: 'var(--font-serif)' }}>
                {t('events.events_on')} {format(selectedDate, 'PPP', locale ? { locale } : undefined)}
              </h3>
              <div className="grid grid-cols-1 gap-4">
                {getEventsForDate(selectedDate).map(event => (
                  <EventCard key={event.id} event={event} />
                ))}
              </div>
            </div>
          )}
        </>
      ) : (
        <div className="space-y-4">
          {events.length === 0 ? (
            <div className="text-center py-12 card">
              <svg className="w-16 h-16 mx-auto mb-4 text-neutral-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={1.5} 
                  d="M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z" />
              </svg>
              <p className="text-neutral-600 mb-4 text-lg" style={{ fontFamily: 'var(--font-body)' }}>{t('events.no_events')}</p>
              <Link to="/submit/event" className="btn btn-primary btn-md">
                {t('events.submit')}
              </Link>
            </div>
          ) : (
            events.map(event => (
              <EventCard key={event.id} event={event} />
            ))
          )}
        </div>
      )}
    </div>
  );
};

export default Calendar;