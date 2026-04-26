import { fileURLToPath, URL } from 'node:url'

import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [
    vue(),
  ],
  resolve: {
    alias: {
      '@': fileURLToPath(new URL('./src', import.meta.url)),
      '@pepakura/shared': fileURLToPath(new URL('../../shared/src', import.meta.url))
    }
  },
  server: {
    port: 3000
  },
  worker: {
    format: 'es',
  },
  optimizeDeps: {
    // Исключаем WASM из оптимизации зависимостей
    exclude: ['pepakura_core_wasm']
  },
  build: {
    target: 'es2022',
    rollupOptions: {
      output: {
        // Убедимся, что worker файлы правильно обрабатываются
        manualChunks: undefined
      }
    }
  }
})