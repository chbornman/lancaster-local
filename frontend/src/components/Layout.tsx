import type React from 'react';
import { Outlet, Link } from 'react-router-dom';
import { useTranslation } from 'react-i18next';
import Header from './Header';

const Layout: React.FC = () => {
  const { t } = useTranslation();

  return (
    <div className="min-h-screen bg-paper">
      <Header />

      <main className="container mx-auto px-3 md:px-4 py-6 md:py-8">
        <Outlet />
      </main>

      <footer className="bg-white border-t-2 border-neutral-900 mt-12 md:mt-16">
        <div className="container mx-auto px-3 md:px-4 py-6 md:py-8">
          <p className="text-center text-neutral-600 mb-2 text-sm md:text-base" style={{ fontFamily: 'var(--font-serif)' }}>
            © 2025 The Lancaster Local
          </p>
          <p className="text-center">
            <Link to="/admin" className="text-neutral-600 hover:text-primary-600 text-sm transition-colors">
              {t('nav.admin')}
            </Link>
          </p>
        </div>
      </footer>
    </div>
  );
};

export default Layout;
