import type { FormEvent } from 'react';
import type React from 'react';
import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { useTranslation } from 'react-i18next';
import { endpoints } from '../utils/api';
import axios from 'axios';

const AdminLogin: React.FC = () => {
  const { t } = useTranslation();
  const navigate = useNavigate();
  const [password, setPassword] = useState('');
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const handleSubmit = async (e: FormEvent) => {
    e.preventDefault();
    setLoading(true);
    setError(null);

    try {
      const response = await endpoints.adminLogin({ password });
      localStorage.setItem('adminToken', response.data.token);
      navigate('/admin/dashboard');
    } catch (err) {
      console.error('Login failed:', err);
      setError(axios.isAxiosError(err) && err.response?.status === 401 ? 'Invalid password' : axios.isAxiosError(err) ? err.message : 'Login failed');
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="max-w-md mx-auto mt-16">
      <div className="bg-white rounded-lg shadow-sm border border-gray-200 p-6">
        <h1 className="text-3xl font-bold mb-6 text-center pb-4 border-b-2 border-gray-200" style={{ fontFamily: 'var(--font-serif)' }}>{t('admin.login')}</h1>

        {error && (
          <div className="bg-red-50 border border-red-600 text-red-700 px-4 py-3 mb-4 font-medium rounded-lg">
            {error}
          </div>
        )}

        <form onSubmit={handleSubmit} className="space-y-4">
          <div>
            <label className="block text-sm font-medium mb-1">
              {t('admin.password')}
            </label>
            <input
              type="password"
              value={password}
              onChange={(e) => setPassword(e.target.value)}
              required
              className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary-600/20 focus:border-primary-600 focus:outline-none bg-white transition-colors"
              autoFocus
            />
          </div>

          <button
            type="submit"
            disabled={loading}
            className="w-full btn btn-primary"
          >
            {loading ? t('common.loading') : t('admin.login_button')}
          </button>
        </form>
      </div>
    </div>
  );
};

export default AdminLogin;