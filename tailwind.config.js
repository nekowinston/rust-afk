/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./web/main.css", "./web/dist/*.html"],
  safelist: ["mocha", "macchiato", "frappe", "latte"],
  plugins: [require("@tailwindcss/forms"), require("@catppuccin/tailwindcss")],
}
