import type React from 'react';

interface MegaphoneIconProps {
  size?: 'sm' | 'md' | 'lg';
  className?: string;
}

const MegaphoneIcon: React.FC<MegaphoneIconProps> = ({ size = 'md', className = '' }) => {
  const sizeMap = {
    sm: 24,
    md: 32,
    lg: 48,
  };

  const dimensions = sizeMap[size];

  return (
    <svg
      width={dimensions}
      height={dimensions}
      viewBox="0 0 64 64"
      fill="none"
      xmlns="http://www.w3.org/2000/svg"
      className={className}
    >
      {/* Megaphone cone */}
      <path
        d="M20 20 L45 10 L45 54 L20 44 Z"
        fill="#16a34a"
        stroke="#1A1A1A"
        strokeWidth="2"
        strokeLinejoin="round"
      />
      
      {/* Handle */}
      <rect
        x="12"
        y="26"
        width="12"
        height="12"
        fill="#16a34a"
        stroke="#1A1A1A"
        strokeWidth="2"
        rx="2"
      />
      
      {/* Sound waves */}
      <path
        d="M48 20 Q52 32 48 44"
        stroke="#1A1A1A"
        strokeWidth="2"
        fill="none"
        strokeLinecap="round"
      />
      <path
        d="M52 16 Q58 32 52 48"
        stroke="#1A1A1A"
        strokeWidth="2"
        fill="none"
        strokeLinecap="round"
        opacity="0.6"
      />
      
      {/* Trigger/Button */}
      <circle
        cx="18"
        cy="32"
        r="3"
        fill="#1A1A1A"
      />
      
      {/* Decorative detail on cone */}
      <line
        x1="30"
        y1="18"
        x2="30"
        y2="46"
        stroke="#16a34a"
        strokeWidth="1"
        opacity="0.5"
      />
      <line
        x1="35"
        y1="16"
        x2="35"
        y2="48"
        stroke="#16a34a"
        strokeWidth="1"
        opacity="0.5"
      />
    </svg>
  );
};

export default MegaphoneIcon;