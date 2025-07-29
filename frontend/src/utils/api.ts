import type { AxiosInstance, AxiosResponse, InternalAxiosRequestConfig } from 'axios';
import axios from 'axios';
import type {
  Language,
  PostsResponse,
  EventsResponse,
  CreatePostRequest,
  CreateEventRequest,
  AdminLoginRequest,
  AdminLoginResponse,
  Post,
  Event,
} from '../types/api';

const API_URL = import.meta.env.VITE_API_URL || 'http://localhost:3000/api';

const api: AxiosInstance = axios.create({
  baseURL: API_URL,
  headers: {
    'Content-Type': 'application/json',
  },
});

// Add auth token to requests if it exists
api.interceptors.request.use((config: InternalAxiosRequestConfig) => {
  const token = localStorage.getItem('adminToken');
  if (token && config.headers) {
    config.headers.Authorization = `Bearer ${token}`;
  }
  return config;
});

// Handle auth errors
api.interceptors.response.use(
  (response) => response,
  (error: unknown) => {
    if (axios.isAxiosError(error) && error.response?.status === 401) {
      localStorage.removeItem('adminToken');
      window.location.href = '/admin';
    }
    return Promise.reject(error instanceof Error ? error : new Error('Request failed'));
  }
);

export default api;

// Query parameter types
interface GetPostsParams {
  lang?: string;
  page?: number;
  limit?: number;
}

interface GetEventsParams {
  lang?: string;
  month?: string;
  category?: string;
}

// API endpoints with proper typing
export const endpoints = {
  // Language
  getLanguages: (): Promise<AxiosResponse<{ languages: Language[] }>> => 
    api.get('/languages'),
  
  // Posts
  getPosts: (params?: GetPostsParams): Promise<AxiosResponse<PostsResponse>> => 
    api.get('/posts', { params }),
  createPost: (data: CreatePostRequest): Promise<AxiosResponse<{ post: Post; message: string }>> => 
    api.post('/posts', data),
  publishPost: (id: number): Promise<AxiosResponse<{ message: string }>> => 
    api.post(`/posts/${id}/publish`),
  
  // Events
  getEvents: (params?: GetEventsParams): Promise<AxiosResponse<EventsResponse>> => 
    api.get('/events', { params }),
  createEvent: (data: CreateEventRequest): Promise<AxiosResponse<{ event: Event; message: string }>> => 
    api.post('/events', data),
  publishEvent: (id: number): Promise<AxiosResponse<{ message: string }>> => 
    api.post(`/events/${id}/publish`),
  
  // Admin
  adminLogin: (data: AdminLoginRequest): Promise<AxiosResponse<AdminLoginResponse>> => 
    api.post('/admin/login', data),
  adminLogout: (): Promise<AxiosResponse<void>> => 
    api.post('/admin/logout'),
  getAdminPosts: (): Promise<AxiosResponse<{ posts: Post[] }>> => 
    api.get('/admin/posts'),
  getAdminEvents: (): Promise<AxiosResponse<{ events: Event[] }>> => 
    api.get('/admin/events'),
  deletePost: (id: number): Promise<AxiosResponse<{ message: string }>> => 
    api.delete(`/admin/posts/${id}`),
  deleteEvent: (id: number): Promise<AxiosResponse<{ message: string }>> => 
    api.delete(`/admin/events/${id}`),
};