import i18n from 'i18next';
import { initReactI18next } from 'react-i18next';
import LanguageDetector from 'i18next-browser-languagedetector';

// Import translations
import enTranslations from './locales/en.json';
import esTranslations from './locales/es.json';
import deTranslations from './locales/de.json';
import frTranslations from './locales/fr.json';
import zhTranslations from './locales/zh.json';
import arTranslations from './locales/ar.json';

const resources = {
  en: { translation: enTranslations },
  es: { translation: esTranslations },
  de: { translation: deTranslations },
  fr: { translation: frTranslations },
  zh: { translation: zhTranslations },
  ar: { translation: arTranslations },
} as const;

void i18n
  .use(LanguageDetector)
  .use(initReactI18next)
  .init({
    resources,
    fallbackLng: (import.meta.env.VITE_DEFAULT_LANGUAGE as string) || 'en',
    debug: false,
    
    interpolation: {
      escapeValue: false,
    },

    detection: {
      order: ['localStorage', 'navigator', 'htmlTag'],
      caches: ['localStorage'],
    },
  });

export default i18n;