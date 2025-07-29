import type React from 'react';

interface LancasterLogoProps {
  className?: string;
  size?: 'sm' | 'md' | 'lg' | 'xl';
  variant?: 'full' | 'icon';
  language?: string;
}

const LancasterLogo: React.FC<LancasterLogoProps> = ({ 
  className = '', 
  size = 'md',
  variant = 'full',
  language = 'en'
}) => {
  const sizeClasses = {
    sm: { icon: 'w-8 h-8', full: 'h-8' },
    md: { icon: 'w-12 h-12', full: 'h-12' },
    lg: { icon: 'w-16 h-16', full: 'h-16' },
    xl: { icon: 'w-20 h-20', full: 'h-20' }
  };

  // Logo text translations
  const logoTexts: Record<string, { the: string; lancaster: string; local: string }> = {
    en: { the: 'THE', lancaster: 'LANCASTER', local: 'LOCAL' },
    es: { the: 'EL', lancaster: 'LANCASTER', local: 'LOCAL' },
    de: { the: 'DAS', lancaster: 'LANCASTER', local: 'LOKAL' },
    fr: { the: 'LE', lancaster: 'LANCASTER', local: 'LOCAL' },
    zh: { the: '兰卡斯特', lancaster: '新闻', local: '地方' },
    ar: { the: 'ال', lancaster: 'لانكستر', local: 'المحلي' }
  };

  const currentText = logoTexts[language] || logoTexts.en;
  
  // Adjust viewBox dimensions for languages that need more space
  const viewBoxWidth = (language === 'zh' || language === 'ar') ? 400 : 320;
  const viewBoxHeight = (language === 'zh' || language === 'ar') ? 80 : 64;
  const centerX = viewBoxWidth / 2;
  
  // Adjust vertical positions for Chinese and Arabic
  const verticalSpacing = (language === 'zh' || language === 'ar') 
    ? { the: 22, lancaster: 50, local: 72 }
    : { the: 22, lancaster: 45, local: 59 };

  if (variant === 'icon') {
    return (
      <svg 
        className={`${sizeClasses[size].icon} ${className}`} 
        viewBox="0 0 64 64" 
        fill="none" 
        xmlns="http://www.w3.org/2000/svg"
      >
        {/* Heritage shield/crest design */}
        <g>
          {/* Shield shape */}
          <path
            d="M32 8 L48 16 L48 32 Q48 48, 32 56 Q16 48, 16 32 L16 16 Z"
            fill="#1f2937"
            stroke="#1f2937"
            strokeWidth="1"
          />
          
          {/* Inner shield */}
          <path
            d="M32 12 L44 18 L44 32 Q44 44, 32 50 Q20 44, 20 32 L20 18 Z"
            fill="#f3f4f6"
          />
          
          {/* Decorative L */}
          <path
            d="M26 22 L26 38 L38 38 L38 34 L30 34 L30 22 Z"
            fill="#1f2937"
          />
          
          {/* Small star accent */}
          <path
            d="M32 42 L33.5 45 L36.5 45 L34 47 L35 50 L32 48 L29 50 L30 47 L27.5 45 L30.5 45 Z"
            fill="#1f2937"
            transform="scale(0.6) translate(20, 20)"
          />
        </g>
      </svg>
    );
  }

  // Full logo with heritage newspaper typography
  return (
    <svg 
      className={`${sizeClasses[size].full} ${className}`} 
      viewBox={`0 0 ${viewBoxWidth} ${viewBoxHeight}`} 
      fill="none" 
      xmlns="http://www.w3.org/2000/svg"
    >
      {/* Decorative line */}
      <line x1="0" y1="8" x2={viewBoxWidth} y2="8" stroke="#1f2937" strokeWidth="1" />
      <line x1="0" y1="10" x2={viewBoxWidth} y2="10" stroke="#1f2937" strokeWidth="1" />
      
      {/* Text part with classic newspaper masthead style */}
      <g>
        {/* The */}
        <text
          x={centerX}
          y={verticalSpacing.the}
          fill="#1f2937"
          fontSize="12"
          fontWeight="400"
          fontFamily="Georgia, serif"
          textAnchor="middle"
          letterSpacing="4"
        >
          {currentText.the}
        </text>
        
        {/* Lancaster */}
        <text
          x={centerX}
          y={verticalSpacing.lancaster}
          fill="#1f2937"
          fontSize="32"
          fontWeight="700"
          fontFamily="Georgia, serif"
          textAnchor="middle"
          letterSpacing="3"
        >
          {currentText.lancaster}
        </text>
        
        {/* Local - smaller subtitle */}
        <text
          x={centerX}
          y={verticalSpacing.local}
          fill="#1f2937"
          fontSize="12"
          fontWeight="400"
          fontFamily="Georgia, serif"
          textAnchor="middle"
          letterSpacing="4"
        >
          {currentText.local}
        </text>
      </g>
      
      {/* Bottom decorative line */}
      <line x1="0" y1={viewBoxHeight - 3} x2={viewBoxWidth} y2={viewBoxHeight - 3} stroke="#1f2937" strokeWidth="1" />
      <line x1="0" y1={viewBoxHeight - 1} x2={viewBoxWidth} y2={viewBoxHeight - 1} stroke="#1f2937" strokeWidth="1" />
    </svg>
  );
};

export default LancasterLogo;