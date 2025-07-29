import type React from 'react';
import { useState, useEffect, useCallback } from 'react';
import { Link } from 'react-router-dom';
import { useTranslation } from 'react-i18next';
import PostCard from '../components/PostCard';
import PostCardSkeleton from '../components/PostCardSkeleton';
import { endpoints } from '../utils/api';
import { useInfiniteScroll } from '../hooks/useInfiniteScroll';
import type { PostWithTranslation } from '../types/api';
import axios from 'axios';

const NewsFeed: React.FC = () => {
  const { t, i18n } = useTranslation();
  const [posts, setPosts] = useState<PostWithTranslation[]>([]);
  const [loading, setLoading] = useState(true);
  const [loadingMore, setLoadingMore] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [page, setPage] = useState(1);
  const [hasMore, setHasMore] = useState(true);
  const [refreshing, setRefreshing] = useState(false);

  const fetchPosts = useCallback(async (pageNum: number, isInitial = false) => {
    try {
      if (isInitial) {
        setLoading(true);
      } else {
        setLoadingMore(true);
      }
      setError(null);
      
      const response = await endpoints.getPosts({ 
        lang: i18n.language, 
        page: pageNum, 
        limit: 10 
      });
      
      const newPosts = response.data.posts;
      const pagination = response.data.pagination;
      
      if (isInitial) {
        setPosts(newPosts);
      } else {
        setPosts(prev => [...prev, ...newPosts]);
      }
      
      setHasMore(pageNum < pagination.total_pages);
      setPage(pageNum + 1);
    } catch (err) {
      console.error('Failed to fetch posts:', err);
      setError(axios.isAxiosError(err) ? err.message : 'Failed to fetch posts');
    } finally {
      setLoading(false);
      setLoadingMore(false);
      setRefreshing(false);
    }
  }, [i18n.language]);

  // Reset when language changes
  useEffect(() => {
    setPosts([]);
    setPage(1);
    setHasMore(true);
    void fetchPosts(1, true);
  }, [i18n.language, fetchPosts]);

  const loadMore = useCallback(() => {
    if (!loadingMore && hasMore) {
      void fetchPosts(page, false);
    }
  }, [page, loadingMore, hasMore, fetchPosts]);

  const handleRefresh = useCallback(async () => {
    setRefreshing(true);
    setPosts([]);
    setPage(1);
    setHasMore(true);
    await fetchPosts(1, true);
  }, [fetchPosts]);

  const lastPostRef = useInfiniteScroll(loadMore, hasMore);

  // Pull to refresh for mobile
  useEffect(() => {
    let startY = 0;
    let pullDistance = 0;
    
    const handleTouchStart = (e: TouchEvent) => {
      if (window.scrollY === 0) {
        startY = e.touches[0].clientY;
      }
    };
    
    const handleTouchMove = (e: TouchEvent) => {
      if (startY === 0) return;
      
      pullDistance = e.touches[0].clientY - startY;
      if (pullDistance > 0 && window.scrollY === 0) {
        e.preventDefault();
      }
    };
    
    const handleTouchEnd = () => {
      if (pullDistance > 100 && window.scrollY === 0) {
        void handleRefresh();
      }
      startY = 0;
      pullDistance = 0;
    };
    
    if ('ontouchstart' in window) {
      document.addEventListener('touchstart', handleTouchStart, { passive: true });
      document.addEventListener('touchmove', handleTouchMove, { passive: false });
      document.addEventListener('touchend', handleTouchEnd);
      
      return () => {
        document.removeEventListener('touchstart', handleTouchStart);
        document.removeEventListener('touchmove', handleTouchMove);
        document.removeEventListener('touchend', handleTouchEnd);
      };
    }
  }, [handleRefresh]);

  if (loading && posts.length === 0) {
    return (
      <div className="max-w-4xl mx-auto">
        <div className="flex flex-row-reverse justify-between items-center mb-8 pb-4 border-b-2 border-[#1A1A1A]">
          <h1 className="text-3xl sm:text-4xl font-bold" style={{ fontFamily: 'var(--font-serif)' }}>{t('posts.title')}</h1>
          <Link to="/submit/post" className="px-4 py-2 bg-[#8B4513] text-white hover:bg-[#6B3410] transition-all font-medium hidden sm:inline-flex shadow-sm hover:shadow-md" style={{ fontFamily: 'var(--font-sans)' }}>
            {t('posts.submit')}
          </Link>
        </div>
        <div className="space-y-4">
          {Array.from({ length: 3 }).map((_, i) => (
            <PostCardSkeleton key={i} />
          ))}
        </div>
      </div>
    );
  }

  if (error && posts.length === 0) {
    return (
      <div className="text-center py-8">
        <p className="text-[#8B4513] mb-4 font-medium">{t('common.error')}: {error}</p>
        <button 
          onClick={() => fetchPosts(1, true)}
          className="px-4 py-2 bg-[#8B4513] text-white hover:bg-[#6B3410] transition-all font-medium shadow-sm hover:shadow-md"
          style={{ fontFamily: 'var(--font-sans)' }}
        >
          {t('common.retry')}
        </button>
      </div>
    );
  }

  return (
    <div className="max-w-4xl mx-auto">
      <div className="flex flex-row-reverse justify-between items-center mb-8 pb-4 border-b-2 border-[#1A1A1A]">
        <h1 className="text-3xl sm:text-4xl font-bold" style={{ fontFamily: 'var(--font-serif)' }}>{t('posts.title')}</h1>
        <Link to="/submit/post" className="px-3 py-1.5 sm:px-4 sm:py-2 bg-[#8B4513] text-white hover:bg-[#6B3410] transition-all font-medium text-sm sm:text-base shadow-sm hover:shadow-md" style={{ fontFamily: 'var(--font-sans)' }}>
          <span className="hidden sm:inline">{t('posts.submit')}</span>
          <svg className="w-5 h-5 sm:hidden" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 4v16m8-8H4" />
          </svg>
        </Link>
      </div>

      {refreshing && (
        <div className="flex justify-center py-4">
          <div className="w-8 h-8 border-4 border-[#8B4513] border-t-transparent rounded-full animate-spin"></div>
        </div>
      )}

      {posts.length === 0 && !loading ? (
        <div className="text-center py-12">
          <p className="text-[#4A4A4A] mb-4 text-lg" style={{ fontFamily: 'var(--font-body)' }}>{t('posts.no_posts')}</p>
          <Link to="/submit/post" className="px-4 py-2 bg-[#8B4513] text-white hover:bg-[#6B3410] transition-all font-medium shadow-sm hover:shadow-md" style={{ fontFamily: 'var(--font-sans)' }}>
            {t('posts.submit')}
          </Link>
        </div>
      ) : (
        <div className="space-y-4 sm:space-y-6">
          {posts.map((post, index) => (
            <div
              key={post.id}
              ref={index === posts.length - 1 ? lastPostRef : null}
            >
              <PostCard post={post} />
            </div>
          ))}
          
          {loadingMore && (
            <div className="space-y-4">
              {Array.from({ length: 2 }).map((_, i) => (
                <PostCardSkeleton key={`skeleton-${i}`} />
              ))}
            </div>
          )}
          
          {!hasMore && posts.length > 0 && (
            <div className="text-center py-8 text-[#4A4A4A]">
              <p className="text-sm uppercase tracking-wide" style={{ fontFamily: 'var(--font-sans)' }}>{t('posts.no_more_posts')}</p>
            </div>
          )}
        </div>
      )}
    </div>
  );
};

export default NewsFeed;