import { defineConfig } from "vite";
import { svelte } from "@sveltejs/vite-plugin-svelte";

// https://vite.dev/config/
export default defineConfig({
  plugins: [svelte()],
  server: {
    proxy: {
      "/api": {
        ws: true,
        target: "http://localhost:3000",
        changeOrigin: true,
        rewriteWsOrigin: true,
        secure: false,
        rewrite: (path) => path,
      },
    },
  },
});
