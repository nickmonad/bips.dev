/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./web/templates/**/*.html", "./web/content/**/*.md"],
  darkMode: 'selector',
  theme: {
    extend: {
      colors: {
        'bitcoin': '#ff9900',
      }
    },
  },
  plugins: [
    require('@tailwindcss/forms'),
    require('@tailwindcss/typography'),
  ],
}
