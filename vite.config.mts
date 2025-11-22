import { defineConfig } from "vite";
import tailwindcss from "@tailwindcss/vite";
import serveStatic from "serve-static";
import path from "path";

export default defineConfig({
  clearScreen: false,
  server: {
    strictPort: true,
    host: "127.0.0.1",
  },
  css: {
    transformer: "lightningcss",
  },
  plugins: [
    tailwindcss(),
    {
      name: "serve-fonts",
      configureServer(server) {
        server.middlewares.use(
          "/assets/fonts",
          serveStatic(path.resolve(__dirname, "./static/assets/fonts")),
        );
      },
    },
  ],
  envPrefix: ["VITE_"],
  experimental: {
    enableNativePlugin: true,
  },
  build: {
    emptyOutDir: false,
    sourcemap: false,
    target: "esnext",
    cssMinify: "lightningcss",
    rolldownOptions: {
      input: {
        index: "./frontend/ts/index.ts",
        code: "./frontend/ts/code.ts",
        index_css: "./frontend/css/index.css",
      },
      output: {
        entryFileNames: "javascript/[name].js",
        chunkFileNames: "javascript/[name].js",
        assetFileNames: (assetInfo) => {
          const fileName = assetInfo.originalFileNames[0] || "unknown";
          if (fileName.endsWith("_css.css")) {
            return `${fileName.replace("_css.css", ".css")}`;
          }
          return `${fileName}`;
        },
      },
    },
    outDir: "../static/assets",
  },
  root: "./frontend",
});
