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
      colors: {
        'pm-black': '#1F2528',
        'pm-blue-left': '#5B8DCA',
        'pm-blue-right': '#85D1D4',
        'pm-red-left': '#EF5280',
        'pm-red-right': '#F15A5B',
      },
      fontFamily: {
        'sans': ['Inter', 'Avenir', 'Helvetica', 'Arial', ...defaultTheme.fontFamily.sans],
      },
    },
  },
  plugins: []
}