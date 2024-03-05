const colors = require("tailwindcss/colors")

/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./web/templates/**/*.html", "./web/content/**/*.md"],
  darkMode: 'selector',
  theme: {
    colors: {
        'gray': colors.gray,
        'slate': colors.slate,
        'zinc': colors.zinc,
        'white': colors.white,
        'bitcoin': '#ff9900',
    },
    extend: {},
  },
  plugins: [
    require('@tailwindcss/forms'),
    require('@tailwindcss/typography'),
  ],
}
