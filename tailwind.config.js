/** @type {import('tailwindcss').Config} */
module.exports = {
  content: {
    files: ["*.html", "./src/**/*.rs"],
  },
  darkMode: "class",
  theme: {
    extend: {
      keyframes: {
        gradientSwipe: {
          "0%": { "background-position": "0% 50%" },
          "100%": { "background-position": "100% 50%" },
        },
      },
      animation: {
        gradientSwipe: "gradientSwipe 0.5s ease-in forwards",
      },
    },
  },
  plugins: [],
};
