import 'i18next';

// Define the shape of our translation resources
interface TranslationResource {
  nav: {
    home: string;
    news: string;
    calendar: string;
    submit: string;
    admin: string;
  };
  language: {
    select: string;
    translated: string;
    original: string;
  };
  posts: {
    title: string;
    submit: string;
    author_name: string;
    author_email: string;
    post_title: string;
    content: string;
    link_url: string;
    image_url: string;
    type: string;
    types: {
      text: string;
      link: string;
      announcement: string;
    };
    submit_success: string;
    no_posts: string;
    no_more_posts: string;
  };
  events: {
    title: string;
    submit: string;
    organizer_name: string;
    organizer_email: string;
    event_title: string;
    description: string;
    date: string;
    time: string;
    location: string;
    category: string;
    categories: {
      community: string;
      education: string;
      social: string;
      sports: string;
      cultural: string;
      religious: string;
    };
    is_free: string;
    ticket_price: string;
    ticket_url: string;
    get_tickets: string;
    organized_by: string;
    all_categories: string;
    calendar_view: string;
    list_view: string;
    events_on: string;
    submit_success: string;
    no_events: string;
  };
  admin: {
    login: string;
    password: string;
    login_button: string;
    logout: string;
    dashboard: string;
    posts: string;
    events: string;
    publish: string;
    unpublished: string;
    published: string;
  };
  common: {
    loading: string;
    error: string;
    submit: string;
    cancel: string;
    save: string;
    delete: string;
    edit: string;
    close: string;
    retry: string;
  };
}

declare module 'i18next' {
  interface CustomTypeOptions {
    defaultNS: 'translation';
    resources: {
      translation: TranslationResource;
    };
  }
}