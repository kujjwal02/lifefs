import { defineConfig } from "vite";

// Tauri loads the dev server from a fixed port, so a busy port fails instead of moving.
export default defineConfig({
  root: "crates/app/ui",
  clearScreen: false,
  server: { port: 5173, strictPort: true },
  build: { outDir: "dist", emptyOutDir: true },
});
