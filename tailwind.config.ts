import { Config } from "tailwindcss";

const config: Config = {
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
};
export default config;
