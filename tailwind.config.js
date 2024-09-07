/** @type {import('tailwindcss').Config} */
export default {
  content: ["./index.html", "./src/**/*.{js,ts,jsx,tsx}"],
  theme: {
    extend: {
      fontFamily: {
        prose: ["Montserrat", "sans-serif"]
     }
    },
  },
  plugins: [require("@catppuccin/tailwindcss")({
    prefix: false,
    defaultFlavour: "mocha",
  })],
}

