import { Config } from "tailwindcss";

export default {
  content: [
    "frontend/templates/*.html",
    "frontend/css/*.css",
    "static/assets/javascript/*.js",
  ],
  theme: {
    extend: {
      colors: {
        dark: {
          1: "#303030",
          2: "#202020",
        },
      },
    },
  },
} satisfies Config;
