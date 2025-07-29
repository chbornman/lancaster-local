import type React from 'react';
import { useState } from 'react';
import { Outlet, Link, useLocation } from 'react-router-dom';
import { useTranslation } from 'react-i18next';
import LanguageSelector from './LanguageSelector';
import NewspaperLogo from './NewspaperLogo';

interface NavLink {
  path: string;
  label: string;
}

const Layout: React.FC = () => {
  const { t } = useTranslation();
  const location = useLocation();
  const [mobileMenuOpen, setMobileMenuOpen] = useState(false);

  const isActive = (path: string): boolean => location.pathname === path;

  const navLinks: NavLink[] = [
    { path: '/', label: t('nav.news') },
    { path: '/calendar', label: t('nav.calendar') },
  ];

  return (
    <div className="min-h-screen bg-[#FDFBF7]">
      <header className="bg-[#FDFBF7] border-b-2 border-[#1A1A1A] sticky top-0 z-40">
        <div className="container mx-auto px-3 md:px-4">
          {/* Desktop Navigation */}
          <nav className="hidden md:flex flex-row-reverse items-center justify-between h-20">
            <div className="flex flex-row-reverse items-center gap-8">
              {navLinks.map(link => (
                <Link 
                  key={link.path}
                  to={link.path} 
                  className={`text-lg font-semibold hover:text-[#8B4513] transition-colors ${
                    isActive(link.path) ? 'text-[#8B4513] border-b-2 border-[#8B4513]' : 'text-[#1A1A1A]'
                  }`}
                  style={{ fontFamily: 'var(--font-serif)' }}
                >
                  {link.label}
                </Link>
              ))}
            </div>
            
            <div className="flex flex-row-reverse items-center gap-4">
              <div className="flex items-center gap-3">
                <NewspaperLogo size="md" />
                <h1 className="text-2xl font-bold" style={{ fontFamily: 'var(--font-serif)' }}>Lancaster Local</h1>
              </div>
              <div className="flex items-center gap-4">
                <LanguageSelector />
                <Link 
                  to="/submit/post" 
                  className="px-4 py-2 bg-[#8B4513] text-white hover:bg-[#6B3410] transition-all font-medium shadow-sm hover:shadow-md"
                  style={{ fontFamily: 'var(--font-sans)' }}
                >
                  {t('nav.submit')}
                </Link>
              </div>
            </div>
          </nav>

          {/* Mobile Navigation */}
          <nav className="md:hidden flex items-center justify-between h-16">
            <Link to="/" className="flex items-center gap-2">
              <NewspaperLogo size="sm" />
              <span className="text-xl font-bold text-[#1A1A1A]" style={{ fontFamily: 'var(--font-serif)' }}>Lancaster Local</span>
            </Link>
            
            <div className="flex items-center gap-2">
              <LanguageSelector />
              <button
                onClick={() => setMobileMenuOpen(!mobileMenuOpen)}
                className="p-2 text-gray-700 hover:bg-gray-100 rounded-md"
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
            </div>
          </nav>
        </div>

        {/* Mobile Menu Dropdown */}
        {mobileMenuOpen && (
          <div className="md:hidden bg-[#FDFBF7] border-t-2 border-[#1A1A1A] shadow-lg">
            <div className="container mx-auto px-3 py-4 space-y-3">
              {navLinks.map(link => (
                <Link
                  key={link.path}
                  to={link.path}
                  onClick={() => setMobileMenuOpen(false)}
                  className={`block py-2 px-4 font-medium transition-colors ${
                    isActive(link.path) 
                      ? 'bg-[#8B4513] text-white' 
                      : 'text-[#1A1A1A] hover:bg-gray-100'
                  }`}
                  style={{ fontFamily: 'var(--font-serif)' }}
                >
                  {link.label}
                </Link>
              ))}
              
              <div className="pt-3 border-t border-[#1A1A1A]">
                <Link
                  to="/submit/post"
                  onClick={() => setMobileMenuOpen(false)}
                  className="block w-full px-4 py-2 bg-[#8B4513] text-white hover:bg-[#6B3410] transition-all text-center font-medium shadow-sm"
                  style={{ fontFamily: 'var(--font-sans)' }}
                >
                  {t('nav.submit')}
                </Link>
              </div>
            </div>
          </div>
        )}
      </header>

      <main className="container mx-auto px-3 md:px-4 py-6 md:py-8">
        <Outlet />
      </main>

      <footer className="bg-white border-t-2 border-[#1A1A1A] mt-12 md:mt-16">
        <div className="container mx-auto px-3 md:px-4 py-6 md:py-8">
          <p className="text-center text-[#4A4A4A] mb-2 text-sm md:text-base" style={{ fontFamily: 'var(--font-serif)' }}>
            © 2024 Lancaster Local
          </p>
          <p className="text-center">
            <Link to="/admin" className="text-[#4A4A4A] hover:text-[#1A1A1A] text-sm">
              {t('nav.admin')}
            </Link>
          </p>
        </div>
      </footer>
    </div>
  );
};

export default Layout;