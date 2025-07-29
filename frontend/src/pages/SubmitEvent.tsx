import type { ChangeEvent, FormEvent } from 'react';
import type React from 'react';
import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { useTranslation } from 'react-i18next';
import { endpoints } from '../utils/api';
import axios from 'axios';

interface EventFormData {
  organizer_name: string;
  organizer_email: string;
  title: string;
  description: string;
  event_date: string;
  event_time: string;
  location: string;
  category: string;
  is_free: boolean;
  ticket_price: string;
  ticket_url: string;
}

const SubmitEvent: React.FC = () => {
  const { t, i18n } = useTranslation();
  const navigate = useNavigate();
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [textDirection, setTextDirection] = useState<'ltr' | 'rtl'>('ltr');
  
  const [formData, setFormData] = useState<EventFormData>({
    organizer_name: '',
    organizer_email: '',
    title: '',
    description: '',
    event_date: '',
    event_time: '',
    location: '',
    category: 'community',
    is_free: true,
    ticket_price: '',
    ticket_url: ''
  });

  // RTL detection helper
  const detectTextDirection = (text: string): 'rtl' | 'ltr' => {
    const rtlRegex = /[\u0591-\u07FF\u200F\u202B\u202E\uFB1D-\uFDFD\uFE70-\uFEFC]/;
    return rtlRegex.test(text) ? 'rtl' : 'ltr';
  };

  const handleInputChange = (e: ChangeEvent<HTMLInputElement | HTMLTextAreaElement | HTMLSelectElement>) => {
    const { name, value, type } = e.target;
    const checked = (e.target as HTMLInputElement).checked;
    const newValue = type === 'checkbox' ? checked : value;
    
    setFormData(prev => ({ ...prev, [name]: newValue }));
    
    // Auto-detect direction for title and description
    if (name === 'title' || name === 'description') {
      setTextDirection(detectTextDirection(value));
    }
  };

  const handleSubmit = async (e: FormEvent) => {
    e.preventDefault();
    setLoading(true);
    setError(null);

    try {
      const eventData = {
        ...formData,
        language: i18n.language,
        text_direction: textDirection,
        ticket_price: formData.is_free ? undefined : parseFloat(formData.ticket_price) || undefined
      };

      await endpoints.createEvent(eventData);
      
      alert(t('events.submit_success'));
      navigate('/calendar');
    } catch (err) {
      console.error('Failed to submit event:', err);
      setError(
        axios.isAxiosError(err) && err.response?.data ? 
          String((err.response.data as { message?: string }).message || 'Failed to submit event') : 
          axios.isAxiosError(err) ? err.message : 'Failed to submit event'
      );
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="max-w-2xl mx-auto">
      <h1 className="text-2xl sm:text-3xl font-bold mb-4 sm:mb-6">{t('events.submit')}</h1>

      {error && (
        <div className="bg-red-50 border border-red-200 text-red-700 px-3 py-2 sm:px-4 sm:py-3 rounded mb-4 text-sm sm:text-base">
          {error}
        </div>
      )}

      <form onSubmit={handleSubmit} className="space-y-4">
        <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
          <div>
            <label className="block text-sm font-medium mb-1">
              {t('events.organizer_name')} *
            </label>
            <input
              type="text"
              name="organizer_name"
              value={formData.organizer_name}
              onChange={handleInputChange}
              required
              className="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-blue-500 focus:border-blue-500"
              dir="auto"
            />
          </div>

          <div>
            <label className="block text-sm font-medium mb-1">
              {t('events.organizer_email')}
            </label>
            <input
              type="email"
              name="organizer_email"
              value={formData.organizer_email}
              onChange={handleInputChange}
              className="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-blue-500 focus:border-blue-500"
              dir="ltr"
            />
          </div>
        </div>

        <div>
          <label className="block text-sm font-medium mb-1">
            {t('events.event_title')} *
          </label>
          <input
            type="text"
            name="title"
            value={formData.title}
            onChange={handleInputChange}
            required
            className="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-blue-500 focus:border-blue-500"
            dir={textDirection}
            style={{
              fontFamily: textDirection === 'rtl' ? 'Noto Sans Arabic, Tahoma, Arial, sans-serif' : 'Inter, -apple-system, sans-serif',
              textAlign: textDirection === 'rtl' ? 'right' : 'left'
            }}
          />
        </div>

        <div>
          <label className="block text-sm font-medium mb-1">
            {t('events.description')}
          </label>
          <textarea
            name="description"
            value={formData.description}
            onChange={handleInputChange}
            rows={4}
            className="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-blue-500 focus:border-blue-500"
            dir={textDirection}
            style={{
              fontFamily: textDirection === 'rtl' ? 'Noto Sans Arabic, Tahoma, Arial, sans-serif' : 'Inter, -apple-system, sans-serif',
              textAlign: textDirection === 'rtl' ? 'right' : 'left',
              lineHeight: textDirection === 'rtl' ? '1.8' : '1.6'
            }}
          />
        </div>

        <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
          <div>
            <label className="block text-sm font-medium mb-1">
              {t('events.date')} *
            </label>
            <input
              type="date"
              name="event_date"
              value={formData.event_date}
              onChange={handleInputChange}
              required
              className="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-blue-500 focus:border-blue-500"
              min={new Date().toISOString().split('T')[0]}
            />
          </div>

          <div>
            <label className="block text-sm font-medium mb-1">
              {t('events.time')}
            </label>
            <input
              type="time"
              name="event_time"
              value={formData.event_time}
              onChange={handleInputChange}
              className="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-blue-500 focus:border-blue-500"
            />
          </div>
        </div>

        <div>
          <label className="block text-sm font-medium mb-1">
            {t('events.location')}
          </label>
          <input
            type="text"
            name="location"
            value={formData.location}
            onChange={handleInputChange}
            className="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-blue-500 focus:border-blue-500"
            dir="auto"
          />
        </div>

        <div>
          <label className="block text-sm font-medium mb-1">
            {t('events.category')}
          </label>
          <select
            name="category"
            value={formData.category}
            onChange={handleInputChange}
            className="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-blue-500 focus:border-blue-500"
          >
            <option value="community">{t('events.categories.community')}</option>
            <option value="education">{t('events.categories.education')}</option>
            <option value="social">{t('events.categories.social')}</option>
            <option value="sports">{t('events.categories.sports')}</option>
            <option value="cultural">{t('events.categories.cultural')}</option>
            <option value="religious">{t('events.categories.religious')}</option>
          </select>
        </div>

        <div className="space-y-4">
          <label className="flex items-center gap-2">
            <input
              type="checkbox"
              name="is_free"
              checked={formData.is_free}
              onChange={handleInputChange}
              className="w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 rounded focus:ring-blue-500"
            />
            <span>{t('events.is_free')}</span>
          </label>

          {!formData.is_free && (
            <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div>
                <label className="block text-sm font-medium mb-1">
                  {t('events.ticket_price')}
                </label>
                <input
                  type="number"
                  name="ticket_price"
                  value={formData.ticket_price}
                  onChange={handleInputChange}
                  step="0.01"
                  min="0"
                  className="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-blue-500 focus:border-blue-500"
                />
              </div>

              <div>
                <label className="block text-sm font-medium mb-1">
                  {t('events.ticket_url')}
                </label>
                <input
                  type="url"
                  name="ticket_url"
                  value={formData.ticket_url}
                  onChange={handleInputChange}
                  className="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-blue-500 focus:border-blue-500"
                  dir="ltr"
                />
              </div>
            </div>
          )}
        </div>

        <div className="flex flex-col sm:flex-row gap-3 pt-4">
          <button
            type="submit"
            disabled={loading}
            className="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 transition-colors font-medium disabled:opacity-50 disabled:cursor-not-allowed flex-1 order-2 sm:order-1"
          >
            {loading ? t('common.loading') : t('common.submit')}
          </button>
          <button
            type="button"
            onClick={() => navigate('/calendar')}
            className="px-4 py-2 bg-gray-100 text-gray-700 rounded-md hover:bg-gray-200 transition-colors font-medium order-1 sm:order-2"
          >
            {t('common.cancel')}
          </button>
        </div>
      </form>
    </div>
  );
};

export default SubmitEvent;