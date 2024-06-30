/** @type {import('tailwindcss').Config} */
export default {
  content: ["./index.html", "./src/**/*.{js,ts,jsx,tsx}"],
  theme: {
    extend: {
      fontFamily: {
        prose: ["Montserrat", "sans-serif"]
     }
    },
    colors: {
      'background-base-default': 'hsl(240, 6%, 10%)',
      'text-normal': 'hsl(240, 5%, 84%)',
      'text-bold': 'hsl(240, 70%, 84%)'
    }
  },
  plugins: [],
}

