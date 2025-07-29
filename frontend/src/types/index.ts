// Global type definitions

export type Direction = 'ltr' | 'rtl';

export interface RouteParams {
  id?: string;
}

// Component Props
export interface LayoutProps {
  children?: React.ReactNode;
}

// Form Types
export interface FormErrors {
  [key: string]: string;
}

// Hook Return Types
export interface DirectionContextValue {
  direction: Direction;
  setDirection: (direction: Direction) => void;
}

// Utility Types
export type Optional<T> = T | null | undefined;