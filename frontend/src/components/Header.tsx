import type React from 'react';
import { useState } from 'react';
import { Link, useLocation } from 'react-router-dom';
import { useTranslation } from 'react-i18next';
import LanguageSelector from './LanguageSelector';
import LancasterLogo from './LancasterLogo';
import { useAdminAuth } from '../hooks/useAdminAuth';

interface NavLink {
  path: string;
  label: string;
}

const Header: React.FC = () => {
  const { t, i18n } = useTranslation();
  const location = useLocation();
  const [mobileMenuOpen, setMobileMenuOpen] = useState(false);
  const isAdmin = useAdminAuth();

  const isActive = (path: string): boolean => location.pathname === path;

  const navLinks: NavLink[] = [
    { path: '/', label: t('nav.news') },
    { path: '/calendar', label: t('nav.calendar') },
  ];

  return (
    <header className="bg-gray-50 border-b-2 border-gray-900 sticky top-0 z-40">
      <div className="container mx-auto px-3 md:px-4">
        {/* Logo - Centered with responsive sizing */}
        <div className="flex justify-center py-6 md:py-12 border-b border-gray-300">
          <Link to="/" className="block">
            <LancasterLogo size="lg" variant="full" language={i18n.language} className="md:hidden" />
            <LancasterLogo size="xl" variant="full" language={i18n.language} className="hidden md:block" />
          </Link>
        </div>

        {/* Desktop Navigation */}
        <nav className="hidden md:flex items-center justify-between h-16">
          {/* Left side - Navigation links */}
          <div className="flex items-center gap-8">
            {navLinks.map(link => (
              <Link
                key={link.path}
                to={link.path}
                className={`text-lg font-semibold hover:text-primary-600 transition-colors ${
                  isActive(link.path) ? 'text-primary-600 border-b-2 border-primary-600' : 'text-gray-900'
                }`}
                style={{ fontFamily: 'var(--font-serif)' }}
              >
                {link.label}
              </Link>
            ))}
          </div>

          {/* Right side - Language selector and admin */}
          <div className="flex items-center gap-4">
            <LanguageSelector />
            {isAdmin && (
              <Link
                to="/admin/dashboard"
                className="btn btn-md bg-neutral-700 text-white hover:bg-neutral-800 rounded-lg shadow-sm hover:shadow-md active:scale-[0.98] transition-shadow"
              >
                {t('nav.admin')}
              </Link>
            )}
          </div>
        </nav>

        {/* Mobile Navigation */}
        <nav className="md:hidden flex items-center justify-between h-14 px-2">
          <button
            onClick={() => setMobileMenuOpen(!mobileMenuOpen)}
            className="p-2 text-gray-700 hover:bg-gray-100 rounded-md transition-all"
            aria-label="Toggle menu"
          >
            {mobileMenuOpen ? (
              <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
              </svg>
            ) : (
              <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M4 6h16M4 12h16M4 18h16" />
              </svg>
            )}
          </button>

          <LanguageSelector />
        </nav>
      </div>

      {/* Mobile Menu Dropdown */}
      {mobileMenuOpen && (
        <div className="md:hidden bg-white border-t border-gray-300 shadow-lg">
          <div className="container mx-auto px-4 py-4 space-y-2">
            {navLinks.map(link => (
              <Link
                key={link.path}
                to={link.path}
                onClick={() => setMobileMenuOpen(false)}
                className={`block py-2 px-4 font-medium transition-colors ${
                  isActive(link.path)
                    ? 'bg-primary-600 text-white rounded-lg'
                    : 'text-gray-900 hover:bg-gray-100 rounded-lg'
                }`}
                style={{ fontFamily: 'var(--font-serif)' }}
              >
                {link.label}
              </Link>
            ))}

            <div className="pt-3 border-t border-gray-900 space-y-3">
              <Link
                to="/submit/post"
                onClick={() => setMobileMenuOpen(false)}
                className="btn btn-primary btn-md w-full rounded-lg shadow-sm hover:shadow-md transition-shadow"
              >
                {t('nav.submit')}
              </Link>
              {isAdmin && (
                <Link
                  to="/admin/dashboard"
                  onClick={() => setMobileMenuOpen(false)}
                  className="btn btn-md bg-gray-700 text-white hover:bg-gray-800 rounded-lg shadow-sm hover:shadow-md active:scale-[0.98] w-full transition-shadow"
                >
                  {t('nav.admin')}
                </Link>
              )}
            </div>
          </div>
        </div>
      )}
    </header>
  );
};

export default Header;