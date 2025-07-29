import type React from 'react';

const PostCardSkeleton: React.FC = () => {
  return (
    <div className="bg-white rounded-lg shadow-sm border border-gray-200 p-4 animate-pulse">
      <div className="flex flex-col sm:flex-row sm:flex-row-reverse sm:justify-between gap-2 mb-3">
        <div className="flex items-center gap-2">
          <div className="h-4 w-24 bg-gray-200 rounded"></div>
          <div className="h-4 w-32 bg-gray-200 rounded"></div>
        </div>
      </div>
      
      <div className="h-6 w-3/4 bg-gray-200 rounded mb-2"></div>
      
      <div className="space-y-2 mb-4">
        <div className="h-4 w-full bg-gray-200 rounded"></div>
        <div className="h-4 w-full bg-gray-200 rounded"></div>
        <div className="h-4 w-2/3 bg-gray-200 rounded"></div>
      </div>
      
      <div className="flex gap-2">
        <div className="h-6 w-16 bg-gray-200 rounded-full"></div>
      </div>
    </div>
  );
};

export default PostCardSkeleton;