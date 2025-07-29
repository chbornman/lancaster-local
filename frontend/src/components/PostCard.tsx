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
    <article className="bg-white shadow-sm border border-[#1A1A1A] p-6 hover:shadow-md transition-all group">
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
          <div className="flex items-center gap-1 text-xs text-[#8B4513] font-medium">
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
        <div className="mb-3">
          <div className="text-xs text-gray-500 mb-1">{t('language.original', { language: post.original_language })}</div>
          <h3 
            className="text-base sm:text-lg font-semibold mb-2 text-gray-600"
            dir={post.original_text_direction}
            style={{ 
              fontFamily: isOriginalRTL ? 'Noto Sans Arabic, Tahoma, Arial, sans-serif' : 'Inter, -apple-system, sans-serif',
              textAlign: isOriginalRTL ? 'right' : 'left'
            }}
          >
            {post.original_title}
          </h3>
        </div>
      )}

      <h3 
        className="text-xl sm:text-2xl font-bold mb-3 leading-tight group-hover:text-[#8B4513] transition-colors"
        dir={post.text_direction}
        style={{ 
          fontFamily: isRTL ? 'Noto Sans Arabic, Tahoma, Arial, sans-serif' : 'var(--font-serif)',
          textAlign: isRTL ? 'right' : 'left'
        }}
      >
        {post.title}
      </h3>

      {showOriginal && post.original_content && (
        <p 
          className="text-xs sm:text-sm text-gray-600 mb-2 whitespace-pre-wrap line-clamp-2"
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
            className="w-full"
            loading="lazy"
          />
        </div>
      )}

      {post.link_url && (
        <a 
          href={post.link_url}
          target="_blank"
          rel="noopener noreferrer"
          className="inline-flex items-center gap-2 text-sm sm:text-base text-[#8B4513] hover:text-[#6B3410] mb-4 font-medium"
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

      <div className="flex flex-wrap gap-2 mt-auto pt-4 border-t border-gray-200">
        <span className="inline-block px-3 py-1 bg-[#8B4513] text-white text-xs sm:text-sm uppercase tracking-wide font-medium" style={{ fontFamily: 'var(--font-sans)' }}>
          {post.post_type}
        </span>
      </div>
    </article>
  );
};

export default PostCard;