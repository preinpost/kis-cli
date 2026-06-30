import { defineConfig } from 'vite'
import { tanstackRouter } from '@tanstack/router-plugin/vite'
import react from '@vitejs/plugin-react'
import tailwindcss from '@tailwindcss/vite'

// https://vite.dev/config/
export default defineConfig({
  base: '/',
  plugins: [
    // tanstackRouter must come before the React plugin.
    tanstackRouter({ target: 'react', autoCodeSplitting: true }),
    react(),
    tailwindcss(),
  ],
  server: {
    port: 5180,
    proxy: {
      '/api': { target: 'http://127.0.0.1:8088', changeOrigin: true },
      '/openapi.json': { target: 'http://127.0.0.1:8088', changeOrigin: true },
      '/docs': { target: 'http://127.0.0.1:8088', changeOrigin: true },
    },
  },
  build: {
    outDir: 'dist',
  },
})
