/* global require */
const defaultTheme = require('tailwindcss/defaultTheme')

/* global module */
module.exports = {
  content: [
    "./src/**/*.{js,ts,jsx,tsx}",
    "./src/pages/**/*.{js,ts,jsx,tsx}",
    "./src/components/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      fontFamily: {
        'sans': ['Inter', 'Avenir', 'Helvetica', 'Arial', ...defaultTheme.fontFamily.sans],
      },
    },
  },
  plugins: []
}