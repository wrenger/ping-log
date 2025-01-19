/** @type {import('tailwindcss').Config} */
module.exports = {
  mode: "all",
  content: ["./src/**/*.{rs,html,css}", "./dist/**/*.html"],
  theme: {
    extend: {},
  },
  plugins: [
    require('daisyui'),
  ],
  // daisyUI config (optional - here are the default values)
  daisyui: {
    themes: [
      "light",
      {
        custom: {
          "primary": "#0081d7",
          "secondary": "#a78bfa",
          "accent": "#424242",
          "neutral": "#202020",
          "base-100": "#000000",
          "info": "#0081d7",
          "success": "#4ade80",
          "warning": "#fde047",
          "error": "#f87171",
          "--padding-card": "1.5rem",
        }
      },
    ], // false: only light + dark | true: all themes | array: specific themes like this ["light", "dark", "cupcake"]
    darkTheme: "custom", // name of one of the included themes for dark mode
    base: true, // applies background color and foreground color for root element by default
    styled: true, // include daisyUI colors and design decisions for all components
    utils: true, // adds responsive and modifier utility classes
    prefix: "", // prefix for daisyUI classnames (components, modifiers and responsive class names. Not colors)
    logs: true, // Shows info about daisyUI version and used config in the console when building your CSS
    themeRoot: ":root", // The element that receives theme color CSS variables
  },
};
