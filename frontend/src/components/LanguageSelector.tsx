import type React from 'react';
import { useEffect, useState } from 'react';
import { useTranslation } from 'react-i18next';
import { useDirection } from '../hooks/useDirection';

interface Language {
  code: string;
  name: string;
  nativeName: string;
  isRtl: boolean;
}

const languages: Language[] = [
  { code: 'en', name: 'English', nativeName: 'English', isRtl: false },
  { code: 'es', name: 'Spanish', nativeName: 'Español', isRtl: false },
  { code: 'de', name: 'German', nativeName: 'Deutsch', isRtl: false },
  { code: 'fr', name: 'French', nativeName: 'Français', isRtl: false },
  { code: 'zh', name: 'Chinese', nativeName: '中文', isRtl: false },
  { code: 'ar', name: 'Arabic', nativeName: 'العربية', isRtl: true },
];

const LanguageSelector: React.FC = () => {
  const { i18n, t } = useTranslation();
  const { setDirection } = useDirection();
  const [isOpen, setIsOpen] = useState(false);

  const currentLanguage = languages.find(lang => lang.code === i18n.language) || languages[0];

  const changeLanguage = async (langCode: string) => {
    await i18n.changeLanguage(langCode);
    const lang = languages.find(l => l.code === langCode);
    const isRTL = lang?.isRtl || false;
    setDirection(isRTL ? 'rtl' : 'ltr');
    setIsOpen(false);
  };

  // Close dropdown when clicking outside
  useEffect(() => {
    const handleClickOutside = (event: MouseEvent) => {
      const target = event.target as HTMLElement;
      if (!target.closest('.language-selector')) {
        setIsOpen(false);
      }
    };

    document.addEventListener('click', handleClickOutside);
    return () => document.removeEventListener('click', handleClickOutside);
  }, []);

  return (
    <div className="language-selector relative">
      <button
        onClick={() => setIsOpen(!isOpen)}
        className="flex items-center gap-1 sm:gap-2 px-2 sm:px-3 py-2 text-xs sm:text-sm font-medium text-neutral-800 bg-white border border-neutral-800 rounded-lg hover:bg-neutral-50 focus:outline-none focus:ring-2 focus:ring-primary-600"
        aria-label={t('language.select')}
        style={{ fontFamily: 'var(--font-sans)' }}
      >
        <span className="hidden sm:inline">{currentLanguage.nativeName}</span>
        <span className="sm:hidden font-bold">{currentLanguage.code.toUpperCase()}</span>
        <svg className="w-3 h-3 sm:w-4 sm:h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M19 9l-7 7-7-7" />
        </svg>
      </button>

      {isOpen && (
        <div className="absolute top-full mt-1 end-0 w-32 bg-white border border-neutral-800 rounded-lg shadow-lg z-50">
          {languages.map((lang) => (
            <button
              key={lang.code}
              onClick={() => changeLanguage(lang.code)}
              className={`block w-full px-3 py-2 text-sm text-center hover:bg-neutral-50 ${
                lang.code === i18n.language ? 'bg-primary-100 text-primary-700 font-medium' : 'text-neutral-800'
              } ${languages.indexOf(lang) === 0 ? 'rounded-t-lg' : ''} ${languages.indexOf(lang) === languages.length - 1 ? 'rounded-b-lg' : ''}`}
              style={{
                fontFamily: lang.isRtl ? 'Noto Sans Arabic, Tahoma, Arial, sans-serif' : 'var(--font-sans)',
                direction: lang.isRtl ? 'rtl' : 'ltr'
              }}
            >
              <div>{lang.nativeName}</div>
              <div className="text-xs text-neutral-600">{lang.name}</div>
            </button>
          ))}
        </div>
      )}
    </div>
  );
};

export default LanguageSelector;