import type React from 'react';

interface NewspaperLogoProps {
  className?: string;
  size?: 'sm' | 'md' | 'lg';
}

const NewspaperLogo: React.FC<NewspaperLogoProps> = ({ className = '', size = 'md' }) => {
  const sizeClasses = {
    sm: 'w-8 h-8',
    md: 'w-12 h-12',
    lg: 'w-16 h-16'
  };

  return (
    <svg 
      className={`${sizeClasses[size]} ${className}`} 
      viewBox="0 0 64 64" 
      fill="none" 
      xmlns="http://www.w3.org/2000/svg"
    >
      {/* Wind swoosh lines */}
      <path
        d="M5 20 Q15 18, 25 20 T45 20"
        stroke="#16a34a"
        strokeWidth="1.5"
        opacity="0.3"
        strokeLinecap="round"
      />
      <path
        d="M3 26 Q13 24, 23 26 T43 26"
        stroke="#16a34a"
        strokeWidth="1.5"
        opacity="0.5"
        strokeLinecap="round"
      />
      <path
        d="M7 32 Q17 30, 27 32 T47 32"
        stroke="#16a34a"
        strokeWidth="1.5"
        opacity="0.3"
        strokeLinecap="round"
      />
      
      {/* Rolled newspaper */}
      <g transform="translate(20, 16)">
        {/* Paper roll */}
        <path
          d="M0 8 Q0 0, 8 0 L20 0 Q28 0, 28 8 L28 24 Q28 32, 20 32 L8 32 Q0 32, 0 24 Z"
          fill="#FDFBF7"
          stroke="#1A1A1A"
          strokeWidth="2"
        />
        
        {/* Rolled edge */}
        <ellipse
          cx="28"
          cy="16"
          rx="4"
          ry="16"
          fill="#FDFBF7"
          stroke="#1A1A1A"
          strokeWidth="2"
        />
        
        {/* Text lines on newspaper */}
        <line x1="4" y1="6" x2="20" y2="6" stroke="#1A1A1A" strokeWidth="1.5" opacity="0.8" />
        <line x1="4" y1="10" x2="18" y2="10" stroke="#1A1A1A" strokeWidth="1" opacity="0.6" />
        <line x1="4" y1="14" x2="16" y2="14" stroke="#1A1A1A" strokeWidth="1" opacity="0.6" />
        <line x1="4" y1="18" x2="18" y2="18" stroke="#1A1A1A" strokeWidth="1" opacity="0.6" />
        <line x1="4" y1="22" x2="14" y2="22" stroke="#1A1A1A" strokeWidth="1" opacity="0.6" />
        <line x1="4" y1="26" x2="16" y2="26" stroke="#1A1A1A" strokeWidth="1" opacity="0.6" />
      </g>
      
      {/* Additional wind effect */}
      <path
        d="M48 24 Q52 23, 56 24"
        stroke="#16a34a"
        strokeWidth="1.5"
        opacity="0.4"
        strokeLinecap="round"
      />
      <path
        d="M50 28 Q54 27, 58 28"
        stroke="#16a34a"
        strokeWidth="1.5"
        opacity="0.3"
        strokeLinecap="round"
      />
    </svg>
  );
};

export default NewspaperLogo;