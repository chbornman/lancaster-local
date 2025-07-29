import type { ChangeEvent, FormEvent } from 'react';
import type React from 'react';
import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { useTranslation } from 'react-i18next';
import { endpoints } from '../utils/api';
import axios from 'axios';

interface PostFormData {
  author_name: string;
  author_email: string;
  title: string;
  content: string;
  link_url: string;
  image_url: string;
  post_type: string;
}

const SubmitPost: React.FC = () => {
  const { t, i18n } = useTranslation();
  const navigate = useNavigate();
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [textDirection, setTextDirection] = useState<'ltr' | 'rtl'>('ltr');
  
  const [formData, setFormData] = useState<PostFormData>({
    author_name: '',
    author_email: '',
    title: '',
    content: '',
    link_url: '',
    image_url: '',
    post_type: 'text'
  });

  // RTL detection helper
  const detectTextDirection = (text: string): 'rtl' | 'ltr' => {
    const rtlRegex = /[\u0591-\u07FF\u200F\u202B\u202E\uFB1D-\uFDFD\uFE70-\uFEFC]/;
    return rtlRegex.test(text) ? 'rtl' : 'ltr';
  };

  const handleInputChange = (e: ChangeEvent<HTMLInputElement | HTMLTextAreaElement | HTMLSelectElement>) => {
    const { name, value } = e.target;
    setFormData(prev => ({ ...prev, [name]: value }));
    
    // Auto-detect direction for title and content
    if (name === 'title' || name === 'content') {
      setTextDirection(detectTextDirection(value));
    }
  };

  const handleSubmit = async (e: FormEvent) => {
    e.preventDefault();
    setLoading(true);
    setError(null);

    try {
      await endpoints.createPost({
        ...formData,
        language: i18n.language,
        text_direction: textDirection
      });
      
      alert(t('posts.submit_success'));
      navigate('/news');
    } catch (err) {
      console.error('Failed to submit post:', err);
      setError(
        axios.isAxiosError(err) && err.response?.data ? 
          String((err.response.data as { message?: string }).message || 'Failed to submit post') : 
          axios.isAxiosError(err) ? err.message : 'Failed to submit post'
      );
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="max-w-2xl mx-auto">
      <h1 className="text-2xl sm:text-3xl font-bold mb-4 sm:mb-6">{t('posts.submit')}</h1>

      {error && (
        <div className="bg-red-50 border border-red-200 text-red-700 px-3 py-2 sm:px-4 sm:py-3 rounded mb-4 text-sm sm:text-base">
          {error}
        </div>
      )}

      <form onSubmit={handleSubmit} className="space-y-4">
        <div>
          <label className="block text-sm font-medium mb-1">
            {t('posts.author_name')} *
          </label>
          <input
            type="text"
            name="author_name"
            value={formData.author_name}
            onChange={handleInputChange}
            required
            className="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-blue-500 focus:border-blue-500"
            dir="auto"
          />
        </div>

        <div>
          <label className="block text-sm font-medium mb-1">
            {t('posts.author_email')}
          </label>
          <input
            type="email"
            name="author_email"
            value={formData.author_email}
            onChange={handleInputChange}
            className="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-blue-500 focus:border-blue-500"
            dir="ltr"
          />
        </div>

        <div>
          <label className="block text-sm font-medium mb-1">
            {t('posts.post_title')} *
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
            {t('posts.content')}
          </label>
          <textarea
            name="content"
            value={formData.content}
            onChange={handleInputChange}
            rows={6}
            className="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-blue-500 focus:border-blue-500"
            dir={textDirection}
            style={{
              fontFamily: textDirection === 'rtl' ? 'Noto Sans Arabic, Tahoma, Arial, sans-serif' : 'Inter, -apple-system, sans-serif',
              textAlign: textDirection === 'rtl' ? 'right' : 'left',
              lineHeight: textDirection === 'rtl' ? '1.8' : '1.6'
            }}
          />
        </div>

        <div>
          <label className="block text-sm font-medium mb-1">
            {t('posts.type')}
          </label>
          <select
            name="post_type"
            value={formData.post_type}
            onChange={handleInputChange}
            className="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-blue-500 focus:border-blue-500"
          >
            <option value="text">{t('posts.types.text')}</option>
            <option value="link">{t('posts.types.link')}</option>
            <option value="announcement">{t('posts.types.announcement')}</option>
          </select>
        </div>

        <div>
          <label className="block text-sm font-medium mb-1">
            {t('posts.link_url')}
          </label>
          <input
            type="url"
            name="link_url"
            value={formData.link_url}
            onChange={handleInputChange}
            className="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-blue-500 focus:border-blue-500"
            dir="ltr"
          />
        </div>

        <div>
          <label className="block text-sm font-medium mb-1">
            {t('posts.image_url')}
          </label>
          <input
            type="url"
            name="image_url"
            value={formData.image_url}
            onChange={handleInputChange}
            className="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-blue-500 focus:border-blue-500"
            dir="ltr"
          />
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
            onClick={() => navigate('/news')}
            className="px-4 py-2 bg-gray-100 text-gray-700 rounded-md hover:bg-gray-200 transition-colors font-medium order-1 sm:order-2"
          >
            {t('common.cancel')}
          </button>
        </div>
      </form>
    </div>
  );
};

export default SubmitPost;