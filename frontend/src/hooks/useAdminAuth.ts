import { useState, useEffect } from 'react';

export const useAdminAuth = () => {
  const [isAdmin, setIsAdmin] = useState<boolean>(false);

  useEffect(() => {
    const checkAdminStatus = () => {
      const token = localStorage.getItem('adminToken');
      setIsAdmin(!!token);
    };

    checkAdminStatus();

    const handleStorageChange = () => {
      checkAdminStatus();
    };

    window.addEventListener('storage', handleStorageChange);
    
    const originalSetItem = localStorage.setItem;
    const originalRemoveItem = localStorage.removeItem;
    
    localStorage.setItem = function(key: string, value: string) {
      originalSetItem.call(this, key, value);
      if (key === 'adminToken') {
        checkAdminStatus();
      }
    };
    
    localStorage.removeItem = function(key: string) {
      originalRemoveItem.call(this, key);
      if (key === 'adminToken') {
        checkAdminStatus();
      }
    };

    return () => {
      window.removeEventListener('storage', handleStorageChange);
      localStorage.setItem = originalSetItem;
      localStorage.removeItem = originalRemoveItem;
    };
  }, []);

  return isAdmin;
};