import type { RefCallback } from 'react';
import { useEffect, useCallback, useRef } from 'react';

export function useInfiniteScroll(
  callback: () => void,
  hasMore: boolean
): RefCallback<HTMLElement> {
  const observer = useRef<IntersectionObserver | null>(null);
  
  const lastElementRef = useCallback<RefCallback<HTMLElement>>(node => {
    if (!hasMore) return;
    
    if (observer.current) observer.current.disconnect();
    
    observer.current = new IntersectionObserver(entries => {
      if (entries[0].isIntersecting && hasMore) {
        callback();
      }
    }, {
      root: null,
      rootMargin: '100px',
      threshold: 0.1
    });
    
    if (node) observer.current.observe(node);
  }, [callback, hasMore]);
  
  useEffect(() => {
    return () => {
      if (observer.current) {
        observer.current.disconnect();
      }
    };
  }, []);
  
  return lastElementRef;
}