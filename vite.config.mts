import { defineConfig } from "vite";

export default defineConfig({
  clearScreen: false,
  server: {
    strictPort: true,
    host: "127.0.0.1",
  },
  envPrefix: ["VITE_"],
  build: {
    emptyOutDir: false,
    sourcemap: false,
    target: "esnext",
    rollupOptions: {
      input: {
        index: "./frontend/ts/index.ts",
        code: "./frontend/ts/code.ts",
      },
      output: {
        entryFileNames: "[name].js",
        chunkFileNames: "[name].js",
        compact: true,
      },
    },
    outDir: "../../static/assets/javascript",
  },
  root: "./frontend/ts",
});
