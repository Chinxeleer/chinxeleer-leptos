/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["*.html", "./src/**/*.rs", 'node_modules/preline/dist/*.js'],
  theme: {
    extend: {
      fontFamily: {
        body: ["Fira Code"]
      },
    },
  },
  plugins: [require('preline/plugin'),],
}

