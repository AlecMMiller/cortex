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
      'background-outline': 'hsl(240, 6%, 3%)',
      'background-base-default': 'hsl(240, 6%, 10%)',
      'background-context': 'hsl(240, 6%, 15%)',
      'background-context-hover': 'hsl(240, 6%, 20%)',
      'outline-context': 'hsl(240, 6%, 22%)',
      'background-code': 'hsl(240, 6%, 20%)',
      'text-normal': 'hsl(240, 5%, 84%)',
      'text-hover': 'hsl(240, 5%, 100%)',
      'text-bold': 'hsl(240, 70%, 84%)',
      'text-soft': 'hsl(240, 5%, 60%)',
      'quote': 'hsl(240, 70%, 84%)',
      'separator': 'hsl(240, 6%, 20%)',
    }
  },
  plugins: [require("tailwindcss-animate")],
}

