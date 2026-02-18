/** @type {import('tailwindcss').Config} */
export default {
  content: ['./src/**/*.{html,js,svelte,ts}'],
  theme: {
    extend: {
      colors: {
        primary: '#D4B235',
        rust: '#C23B22',
        background: {
          light: '#f8f7f6',
          dark: '#121212'
        },
        neutral: {
          light: '#E6E6E6',
          dark: '#1A1A1A'
        },
        kora: {
          bg: '#121212',
          gold: '#D4B235',
          rust: '#C23B22',
          surface: '#1E1E1E',
          border: '#333333'
        }
      },
      fontFamily: {
        sans: ['"Space Grotesk"', 'sans-serif'],
        mono: ['"JetBrains Mono"', 'monospace'],
        display: ['"Space Grotesk"', 'sans-serif']
      }
    }
  },
  plugins: []
};
