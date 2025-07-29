import type React from 'react';
import { useTranslation } from 'react-i18next';
import { format } from 'date-fns';
import { ar, de, es, fr, zhCN } from 'date-fns/locale';
import type { PostWithTranslation } from '../types/api';
import type { Locale } from 'date-fns';

const locales: Record<string, Locale | undefined> = {
  en: undefined,
  ar: ar,
  de: de,
  es: es,
  fr: fr,
  zh: zhCN,
};

interface PostCardProps {
  post: PostWithTranslation;
}

const PostCard: React.FC<PostCardProps> = ({ post }) => {
  const { t, i18n } = useTranslation();
  const isRTL = post.text_direction === 'rtl';
  const isOriginalRTL = post.original_text_direction === 'rtl';
  const locale = locales[i18n.language] ?? undefined;
  const showOriginal = post.is_translated && post.original_language !== i18n.language;

  const formatDate = (dateString: string): string => {
    const date = new Date(dateString);
    return format(date, 'PPP', locale ? { locale } : undefined);
  };

  return (
    <article className="p-6 bg-white rounded-lg shadow-sm border border-neutral-200 hover:shadow-md hover:border-primary-200 transition-all duration-200 relative overflow-hidden">
      <div className="absolute top-0 left-0 w-full h-1 bg-gradient-to-r from-primary-600 to-primary-500"></div>
      <div className="flex flex-col sm:flex-row sm:flex-row-reverse sm:justify-between gap-2 mb-4">
        <div className="flex flex-wrap items-center gap-3">
          <span 
            className="font-semibold text-[#1A1A1A] text-sm sm:text-base uppercase tracking-wide"
            dir="auto"
            style={{ fontFamily: 'var(--font-sans)' }}
          >
            {post.author_name}
          </span>
          <span className="text-[#4A4A4A]">•</span>
          <time 
            className="text-xs sm:text-sm text-[#4A4A4A] uppercase tracking-wide"
            dateTime={post.created_at}
            style={{ fontFamily: 'var(--font-sans)' }}
          >
            {formatDate(post.created_at)}
          </time>
        </div>
        
        {post.is_translated && (
          <div className="flex items-center gap-1 bg-primary-600/10 text-primary-600 px-2 py-1 rounded-full text-xs font-medium">
            <svg className="w-4 h-4 flex-shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} 
                d="M3 5h12M9 3v2m1.048 9.5A18.022 18.022 0 016.412 9m6.088 9h7M11 21l5-10 5 10M12.751 5C11.783 10.77 8.07 15.61 3 18.129" />
            </svg>
            <span className="hidden sm:inline uppercase tracking-wide" style={{ fontFamily: 'var(--font-sans)' }}>{t('language.translated', { language: post.original_language })}</span>
            <span className="sm:hidden uppercase tracking-wide" style={{ fontFamily: 'var(--font-sans)' }}>{post.original_language.toUpperCase()}</span>
          </div>
        )}
      </div>

      {showOriginal && (
        <div className="bg-neutral-50 border-l-4 border-primary-600/30 p-4 mb-4 rounded-r">
          <div className="text-xs font-semibold text-neutral-600 mb-2 uppercase tracking-wide flex items-center gap-1">
            <svg className="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M3 5h12M9 3v2m1.048 9.5A18.022 18.022 0 016.412 9m6.088 9h7M11 21l5-10 5 10M12.751 5C11.783 10.77 8.07 15.61 3 18.129" />
            </svg>
            {t('language.original', { language: post.original_language })}
          </div>
          <h3 
            className="text-base sm:text-lg font-semibold mb-2 text-neutral-700"
            dir={post.original_text_direction}
            style={{ 
              fontFamily: isOriginalRTL ? 'Noto Sans Arabic, Tahoma, Arial, sans-serif' : 'Inter, -apple-system, sans-serif',
              textAlign: isOriginalRTL ? 'right' : 'left'
            }}
          >
            {post.original_title}
          </h3>
          {post.original_content && (
            <p 
              className="text-xs sm:text-sm text-neutral-600 whitespace-pre-wrap line-clamp-2"
              dir={post.original_text_direction}
              style={{ 
                fontFamily: isOriginalRTL ? 'Noto Sans Arabic, Tahoma, Arial, sans-serif' : 'Inter, -apple-system, sans-serif',
                textAlign: isOriginalRTL ? 'right' : 'left',
                lineHeight: isOriginalRTL ? '1.8' : '1.6'
              }}
            >
              {post.original_content}
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
        className="text-xl sm:text-2xl font-bold mb-3 leading-tight hover:text-primary-600 transition-colors"
        dir={post.text_direction}
        style={{ 
          fontFamily: isRTL ? 'Noto Sans Arabic, Tahoma, Arial, sans-serif' : 'var(--font-serif)',
          textAlign: isRTL ? 'right' : 'left'
        }}
      >
        {post.title}
      </h3>

      {post.content && (
        <p 
          className="text-sm sm:text-base text-[#1A1A1A] mb-4 whitespace-pre-wrap line-clamp-4 sm:line-clamp-none leading-relaxed"
          dir={post.text_direction}
          style={{ 
            fontFamily: isRTL ? 'Noto Sans Arabic, Tahoma, Arial, sans-serif' : 'var(--font-body)',
            textAlign: isRTL ? 'right' : 'left',
            lineHeight: isRTL ? '1.8' : '1.7'
          }}
        >
          {post.content}
        </p>
      )}

      {post.image_url && (
        <div className="mb-4 -mx-6 sm:mx-0">
          <img 
            src={post.image_url} 
            alt={post.title}
            className="w-full rounded-lg"
            loading="lazy"
          />
        </div>
      )}

      {post.link_url && (
        <a 
          href={post.link_url}
          target="_blank"
          rel="noopener noreferrer"
          className="inline-flex items-center gap-2 text-sm sm:text-base text-primary-600 hover:text-primary-700 mb-4 font-medium transition-colors"
          dir="auto"
          style={{ fontFamily: 'var(--font-sans)' }}
        >
          <span className="truncate max-w-[200px] sm:max-w-none underline">{post.link_url}</span>
          <svg className="w-4 h-4 flex-shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} 
              d="M10 6H6a2 2 0 00-2 2v10a2 2 0 002 2h10a2 2 0 002-2v-4M14 4h6m0 0v6m0-6L10 14" />
          </svg>
        </a>
      )}

      <div className="flex flex-wrap gap-2 mt-auto pt-4 border-t border-neutral-200">
        <span className="inline-block px-3 py-1 bg-primary-600 text-white text-xs sm:text-sm uppercase tracking-wide font-medium rounded-full" style={{ fontFamily: 'var(--font-sans)' }}>
          {post.post_type}
        </span>
      </div>
    </article>
  );
};

export default PostCard;