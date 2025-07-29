/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        primary: {
          DEFAULT: '#2563eb',
          dark: '#1d4ed8',
          light: '#3b82f6'
        },
        secondary: '#64748b',
      },
      fontFamily: {
        'sans': ['Inter', '-apple-system', 'BlinkMacSystemFont', 'Segoe UI', 'sans-serif'],
        'arabic': ['Noto Sans Arabic', 'Tahoma', 'Arial', 'sans-serif'],
      },
      animation: {
        'pulse': 'pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite',
      }
    },
  },
  plugins: [
    function({ addUtilities }) {
      addUtilities({
        '.direction-ltr': {
          direction: 'ltr',
        },
        '.direction-rtl': {
          direction: 'rtl',
        },
        '.text-start': {
          textAlign: 'start',
        },
        '.text-end': {
          textAlign: 'end',
        },
      })
    }
  ],
}