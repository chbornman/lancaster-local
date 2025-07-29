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
      <div className="bg-white shadow-sm border-2 border-[#1A1A1A] p-2 sm:p-4 overflow-x-auto">
        <div className="min-w-[280px]">
          <div className="grid grid-cols-7 gap-1 mb-2">
            {weekDays.map((day, index) => (
              <div 
                key={index} 
                className="text-center text-xs sm:text-sm font-bold text-[#1A1A1A] py-1 sm:py-2 uppercase"
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
                    min-h-[50px] sm:min-h-[80px] p-1 sm:p-2 border cursor-pointer transition-all
                    ${!day ? 'invisible' : ''}
                    ${!isCurrentMonth ? 'text-gray-400' : ''}
                    ${isToday ? 'bg-[#8B4513]/10 border-[#8B4513]' : 'border-[#1A1A1A]'}
                    ${isSelected ? 'bg-[#8B4513]/20 border-[#8B4513] border-2' : ''}
                    ${day ? 'hover:bg-gray-50' : ''}
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
                              className="text-xs p-1 bg-[#8B4513]/10 text-[#8B4513] truncate font-medium"
                              style={{ fontFamily: 'var(--font-sans)' }}
                              title={event.title}
                            >
                              {event.title}
                            </div>
                          ))}
                          {dayEvents.length > 2 && (
                            <div className="text-xs text-[#4A4A4A] font-medium" style={{ fontFamily: 'var(--font-sans)' }}>
                              +{dayEvents.length - 2}
                            </div>
                          )}
                        </div>
                      )}
                      {/* Mobile indicator */}
                      {dayEvents.length > 0 && (
                        <div className="sm:hidden">
                          <div className="w-2 h-2 bg-[#8B4513] rounded-full mx-auto"></div>
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
        <div className="w-8 h-8 border-4 border-[#8B4513] border-t-transparent rounded-full animate-spin"></div>
      </div>
    );
  }

  if (error) {
    return (
      <div className="text-center py-8">
        <p className="text-[#8B4513] mb-4 font-medium">{t('common.error')}: {error}</p>
        <button 
          onClick={fetchEvents}
          className="px-4 py-2 bg-[#8B4513] text-white hover:bg-[#6B3410] transition-all font-medium shadow-sm hover:shadow-md"
          style={{ fontFamily: 'var(--font-sans)' }}
        >
          {t('common.retry')}
        </button>
      </div>
    );
  }

  return (
    <div className="max-w-6xl mx-auto">
      <div className="flex flex-col sm:flex-row sm:flex-row-reverse sm:justify-between sm:items-center gap-4 mb-8 pb-4 border-b-2 border-[#1A1A1A]">
        <h1 className="text-3xl sm:text-4xl font-bold" style={{ fontFamily: 'var(--font-serif)' }}>{t('events.title')}</h1>
        <Link to="/submit/event" className="px-4 py-2 bg-[#8B4513] text-white hover:bg-[#6B3410] transition-all font-medium w-full sm:w-auto text-center shadow-sm hover:shadow-md" style={{ fontFamily: 'var(--font-sans)' }}>
          {t('events.submit')}
        </Link>
      </div>

      {/* Calendar Navigation */}
      <div className="flex flex-col gap-4 mb-6">
        {/* Month Navigation */}
        <div className="flex items-center justify-center gap-2 sm:gap-4">
          <button
            onClick={() => navigateMonth(isRTL ? 1 : -1)}
            className="px-3 py-2 bg-white text-[#1A1A1A] border border-[#1A1A1A] hover:bg-gray-50 transition-all font-bold"
            style={{ fontFamily: 'var(--font-sans)' }}
          >
            {isRTL ? '←' : '→'}
          </button>
          
          <h2 className="text-lg sm:text-xl font-bold min-w-[140px] text-center" style={{ fontFamily: 'var(--font-serif)' }}>
            {format(currentMonth, 'MMM yyyy', locale ? { locale } : undefined)}
          </h2>
          
          <button
            onClick={() => navigateMonth(isRTL ? -1 : 1)}
            className="px-3 py-2 bg-white text-[#1A1A1A] border border-[#1A1A1A] hover:bg-gray-50 transition-all font-bold"
            style={{ fontFamily: 'var(--font-sans)' }}
          >
            {isRTL ? '→' : '←'}
          </button>
        </div>

        {/* Filters and View Mode */}
        <div className="flex flex-col sm:flex-row gap-3 sm:justify-center">
          <select
            value={category}
            onChange={(e) => setCategory(e.target.value)}
            className="w-full sm:w-auto px-3 py-2 border border-[#1A1A1A] text-sm sm:text-base bg-white"
            style={{ fontFamily: 'var(--font-sans)' }}
          >
            <option value="">{t('events.all_categories')}</option>
            <option value="community">{t('events.categories.community')}</option>
            <option value="education">{t('events.categories.education')}</option>
            <option value="social">{t('events.categories.social')}</option>
            <option value="sports">{t('events.categories.sports')}</option>
            <option value="cultural">{t('events.categories.cultural')}</option>
            <option value="religious">{t('events.categories.religious')}</option>
          </select>

          <div className="flex overflow-hidden border border-[#1A1A1A] w-full sm:w-auto">
            <button
              onClick={() => setViewMode('calendar')}
              className={`flex-1 sm:flex-initial px-3 sm:px-4 py-2 text-sm sm:text-base font-medium transition-all ${
                viewMode === 'calendar' ? 'bg-[#8B4513] text-white' : 'bg-white text-[#1A1A1A]'
              }`}
              style={{ fontFamily: 'var(--font-sans)' }}
            >
              {t('events.calendar_view')}
            </button>
            <button
              onClick={() => setViewMode('list')}
              className={`flex-1 sm:flex-initial px-3 sm:px-4 py-2 text-sm sm:text-base font-medium transition-all ${
                viewMode === 'list' ? 'bg-[#8B4513] text-white' : 'bg-white text-[#1A1A1A]'
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
            <div className="text-center py-12">
              <p className="text-[#4A4A4A] mb-4 text-lg" style={{ fontFamily: 'var(--font-body)' }}>{t('events.no_events')}</p>
              <Link to="/submit/event" className="px-4 py-2 bg-[#8B4513] text-white hover:bg-[#6B3410] transition-all font-medium shadow-sm hover:shadow-md" style={{ fontFamily: 'var(--font-sans)' }}>
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