import type React from 'react';
import { useState, useEffect } from 'react';
import { useNavigate } from 'react-router-dom';
import { useTranslation } from 'react-i18next';
import { endpoints } from '../utils/api';
import type { Post, Event } from '../types/api';
import axios from 'axios';

const AdminDashboard: React.FC = () => {
  const { t } = useTranslation();
  const navigate = useNavigate();
  const [posts, setPosts] = useState<Post[]>([]);
  const [events, setEvents] = useState<Event[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [activeTab, setActiveTab] = useState<'posts' | 'events'>('posts');

  useEffect(() => {
    // Check if admin is logged in
    if (!localStorage.getItem('adminToken')) {
      navigate('/admin');
      return;
    }

    void fetchData();
  }, [navigate]);

  const fetchData = async () => {
    try {
      setLoading(true);
      setError(null);
      
      const [postsRes, eventsRes] = await Promise.all([
        endpoints.getAdminPosts(),
        endpoints.getAdminEvents()
      ]);
      
      setPosts(postsRes.data.posts);
      setEvents(eventsRes.data.events);
    } catch (err) {
      console.error('Failed to fetch admin data:', err);
      setError(axios.isAxiosError(err) ? err.message : 'Failed to fetch data');
    } finally {
      setLoading(false);
    }
  };

  const handlePublishPost = async (postId: number) => {
    try {
      await endpoints.publishPost(postId);
      await fetchData(); // Refresh data
    } catch (err) {
      console.error('Failed to publish post:', err);
      alert('Failed to publish post');
    }
  };

  const handlePublishEvent = async (eventId: number) => {
    try {
      await endpoints.publishEvent(eventId);
      await fetchData(); // Refresh data
    } catch (err) {
      console.error('Failed to publish event:', err);
      alert('Failed to publish event');
    }
  };

  const handleDeletePost = async (postId: number) => {
    if (!confirm('Are you sure you want to delete this post?')) {
      return;
    }
    
    try {
      await endpoints.deletePost(postId);
      await fetchData(); // Refresh data
    } catch (err) {
      console.error('Failed to delete post:', err);
      alert('Failed to delete post');
    }
  };

  const handleDeleteEvent = async (eventId: number) => {
    if (!confirm('Are you sure you want to delete this event?')) {
      return;
    }
    
    try {
      await endpoints.deleteEvent(eventId);
      await fetchData(); // Refresh data
    } catch (err) {
      console.error('Failed to delete event:', err);
      alert('Failed to delete event');
    }
  };

  const handleLogout = async () => {
    try {
      await endpoints.adminLogout();
    } catch {
      // Ignore errors
    }
    localStorage.removeItem('adminToken');
    navigate('/admin');
  };

  if (loading) {
    return (
      <div className="flex justify-center items-center min-h-[400px]">
        <div className="w-8 h-8 border-4 border-primary-600 border-t-transparent rounded-full animate-spin"></div>
      </div>
    );
  }

  if (error) {
    return (
      <div className="text-center py-8">
        <p className="text-red-600 mb-4">{t('common.error')}: {error}</p>
        <button onClick={fetchData} className="px-4 py-2 bg-primary-600 text-white rounded-lg hover:bg-primary-700 transition-colors font-medium shadow-sm hover:shadow-md">
          {t('common.retry')}
        </button>
      </div>
    );
  }

  const unpublishedPosts = posts.filter(p => !p.published);
  const publishedPosts = posts.filter(p => p.published);
  const unpublishedEvents = events.filter(e => !e.published);
  const publishedEvents = events.filter(e => e.published);

  return (
    <div className="max-w-6xl mx-auto">
      <div className="flex justify-between items-center mb-6">
        <h1 className="text-3xl font-bold">{t('admin.dashboard')}</h1>
        <button onClick={handleLogout} className="btn btn-secondary btn-md">
          {t('admin.logout')}
        </button>
      </div>

      {/* Tab Navigation */}
      <div className="flex gap-4 mb-6 border-b">
        <button
          onClick={() => setActiveTab('posts')}
          className={`pb-2 px-4 font-medium transition-colors ${
            activeTab === 'posts' 
              ? 'text-primary-600 border-b-2 border-primary-600' 
              : 'text-neutral-600 hover:text-neutral-800'
          }`}
        >
          {t('admin.posts')} ({unpublishedPosts.length} unpublished)
        </button>
        <button
          onClick={() => setActiveTab('events')}
          className={`pb-2 px-4 font-medium transition-colors ${
            activeTab === 'events' 
              ? 'text-primary-600 border-b-2 border-primary-600' 
              : 'text-neutral-600 hover:text-neutral-800'
          }`}
        >
          {t('admin.events')} ({unpublishedEvents.length} unpublished)
        </button>
      </div>

      {/* Content */}
      {activeTab === 'posts' ? (
        <div className="space-y-6">
          <section>
            <h2 className="text-xl font-semibold mb-4">
              {t('admin.unpublished')} {t('admin.posts')}
            </h2>
            {unpublishedPosts.length === 0 ? (
              <p className="text-neutral-600">No unpublished posts</p>
            ) : (
              <div className="space-y-4">
                {unpublishedPosts.map(post => (
                  <div key={post.id} className="card rounded-lg shadow-sm">
                    <div className="flex justify-between items-start mb-2">
                      <div>
                        <h3 className="font-semibold">{post.title}</h3>
                        <p className="text-sm text-neutral-600">
                          By: {post.author_name} | 
                          Language: {post.original_language} |
                          Direction: {post.text_direction}
                        </p>
                      </div>
                      <div className="flex gap-2">
                        <button
                          onClick={() => handlePublishPost(post.id)}
                          className="btn btn-primary btn-sm"
                        >
                          {t('admin.publish')}
                        </button>
                        <button
                          onClick={() => handleDeletePost(post.id)}
                          className="btn btn-danger btn-sm"
                        >
                          {t('admin.delete')}
                        </button>
                      </div>
                    </div>
                    {post.content && (
                      <p className="text-neutral-700 mt-2">{post.content}</p>
                    )}
                  </div>
                ))}
              </div>
            )}
          </section>

          <section>
            <h2 className="text-xl font-semibold mb-4">
              {t('admin.published')} {t('admin.posts')}
            </h2>
            {publishedPosts.length === 0 ? (
              <p className="text-neutral-600">No published posts</p>
            ) : (
              <div className="space-y-4">
                {publishedPosts.map(post => (
                  <div key={post.id} className="card rounded-lg shadow-sm opacity-75">
                    <div className="flex justify-between items-start">
                      <div>
                        <h3 className="font-semibold">{post.title}</h3>
                        <p className="text-sm text-neutral-600">
                          By: {post.author_name} | Published
                        </p>
                      </div>
                      <button
                        onClick={() => handleDeletePost(post.id)}
                        className="btn btn-danger btn-sm"
                      >
                        {t('admin.delete')}
                      </button>
                    </div>
                  </div>
                ))}
              </div>
            )}
          </section>
        </div>
      ) : (
        <div className="space-y-6">
          <section>
            <h2 className="text-xl font-semibold mb-4">
              {t('admin.unpublished')} {t('admin.events')}
            </h2>
            {unpublishedEvents.length === 0 ? (
              <p className="text-neutral-600">No unpublished events</p>
            ) : (
              <div className="space-y-4">
                {unpublishedEvents.map(event => (
                  <div key={event.id} className="card rounded-lg shadow-sm">
                    <div className="flex justify-between items-start mb-2">
                      <div>
                        <h3 className="font-semibold">{event.title}</h3>
                        <p className="text-sm text-neutral-600">
                          Date: {event.event_date} | 
                          Organizer: {event.organizer_name} |
                          Language: {event.original_language}
                        </p>
                      </div>
                      <div className="flex gap-2">
                        <button
                          onClick={() => handlePublishEvent(event.id)}
                          className="btn btn-primary btn-sm"
                        >
                          {t('admin.publish')}
                        </button>
                        <button
                          onClick={() => handleDeleteEvent(event.id)}
                          className="btn btn-danger btn-sm"
                        >
                          {t('admin.delete')}
                        </button>
                      </div>
                    </div>
                    {event.description && (
                      <p className="text-neutral-700 mt-2">{event.description}</p>
                    )}
                  </div>
                ))}
              </div>
            )}
          </section>

          <section>
            <h2 className="text-xl font-semibold mb-4">
              {t('admin.published')} {t('admin.events')}
            </h2>
            {publishedEvents.length === 0 ? (
              <p className="text-neutral-600">No published events</p>
            ) : (
              <div className="space-y-4">
                {publishedEvents.map(event => (
                  <div key={event.id} className="card rounded-lg shadow-sm opacity-75">
                    <div className="flex justify-between items-start">
                      <div>
                        <h3 className="font-semibold">{event.title}</h3>
                        <p className="text-sm text-neutral-600">
                          Date: {event.event_date} | Published
                        </p>
                      </div>
                      <button
                        onClick={() => handleDeleteEvent(event.id)}
                        className="btn btn-danger btn-sm"
                      >
                        {t('admin.delete')}
                      </button>
                    </div>
                  </div>
                ))}
              </div>
            )}
          </section>
        </div>
      )}
    </div>
  );
};

export default AdminDashboard;