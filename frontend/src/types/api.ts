// API Types matching Rust backend models

export interface Language {
  code: string;
  name: string;
  native_name: string;
  is_rtl: boolean;
  text_direction: 'ltr' | 'rtl';
  enabled: boolean;
}

export interface Post {
  id: number;
  author_name: string;
  author_email?: string;
  title: string;
  content?: string;
  link_url?: string;
  image_url?: string;
  post_type: 'text' | 'link' | 'announcement' | 'article';
  original_language: string;
  text_direction: 'ltr' | 'rtl';
  published: boolean;
  created_at: string;
  updated_at: string;
}

export interface PostWithTranslation {
  id: number;
  author_name: string;
  title: string;
  content?: string;
  original_title: string;
  original_content?: string;
  link_url?: string;
  image_url?: string;
  post_type: string;
  original_language: string;
  original_text_direction: 'ltr' | 'rtl';
  text_direction: 'ltr' | 'rtl';
  is_translated: boolean;
  created_at: string;
}

export interface CreatePostRequest {
  author_name: string;
  author_email?: string;
  title: string;
  content?: string;
  link_url?: string;
  image_url?: string;
  post_type: string;
  language?: string;
  text_direction?: 'ltr' | 'rtl';
}

export interface Event {
  id: number;
  organizer_name: string;
  organizer_email?: string;
  title: string;
  description?: string;
  event_date: string;
  event_time?: string;
  location?: string;
  category?: string;
  is_free: boolean;
  ticket_price?: number;
  ticket_url?: string;
  original_language: string;
  text_direction: 'ltr' | 'rtl';
  published: boolean;
  created_at: string;
}

export interface EventWithTranslation {
  id: number;
  organizer_name: string;
  title: string;
  description?: string;
  original_title: string;
  original_description?: string;
  event_date: string;
  event_time?: string;
  location?: string;
  category?: string;
  is_free: boolean;
  ticket_price?: number;
  ticket_url?: string;
  original_language: string;
  original_text_direction: 'ltr' | 'rtl';
  text_direction: 'ltr' | 'rtl';
  is_translated: boolean;
  created_at: string;
}

export interface CreateEventRequest {
  organizer_name: string;
  organizer_email?: string;
  title: string;
  description?: string;
  event_date: string;
  event_time?: string;
  location?: string;
  category?: string;
  is_free: boolean;
  ticket_price?: number;
  ticket_url?: string;
  language?: string;
  text_direction?: 'ltr' | 'rtl';
}

export interface AdminLoginRequest {
  password: string;
}

export interface AdminLoginResponse {
  token: string;
}

// API Response Types
export interface PaginationInfo {
  page: number;
  limit: number;
  total: number;
  total_pages: number;
}

export interface PostsResponse {
  posts: PostWithTranslation[];
  pagination: PaginationInfo;
}

export interface EventsResponse {
  events: EventWithTranslation[];
}

export interface ApiError {
  message: string;
  status?: number;
}