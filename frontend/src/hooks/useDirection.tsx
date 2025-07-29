import { useContext } from 'react';
import { DirectionContext } from '../contexts/DirectionContext';
import type { DirectionContextValue } from '../types';

export function useDirection(): DirectionContextValue {
  const context = useContext(DirectionContext);
  if (!context) {
    throw new Error('useDirection must be used within DirectionProvider');
  }
  return context;
}