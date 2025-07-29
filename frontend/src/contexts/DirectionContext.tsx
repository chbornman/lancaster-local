import type { ReactNode } from 'react';
import { useState, useCallback, createContext } from 'react';
import type { Direction, DirectionContextValue } from '../types';

export const DirectionContext = createContext<DirectionContextValue | undefined>(undefined);

interface DirectionProviderProps {
  children: ReactNode;
}

export function DirectionProvider({ children }: DirectionProviderProps) {
  const [direction, setDirectionState] = useState<Direction>('ltr');

  const setDirection = useCallback((dir: Direction) => {
    setDirectionState(dir);
    document.documentElement.dir = dir;
  }, []);

  return (
    <DirectionContext.Provider value={{ direction, setDirection }}>
      {children}
    </DirectionContext.Provider>
  );
}