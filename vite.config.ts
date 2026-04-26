import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import path from 'path'

// Определяем режим работы
const isTauri = process.env.TAURI_ENV_PLATFORM != null;

export default defineConfig({
  plugins: [vue()],
  resolve: {
    alias: {
      '@': path.resolve(__dirname, './src'),
      '@shared': path.resolve(__dirname, './packages/shared/src'),
      '@core': path.resolve(__dirname, './packages/core/src'),
      '@pepakura_wasm': path.resolve(__dirname, './crates/pepakura_wasm'),
      '@frontend': path.resolve(__dirname, './frontend/src'),
    }
  },
  server: {
    port: 5174,
    strictPort: true,
    // Настройки для Web Workers и WASM
    headers: {
      // Требуется для SharedArrayBuffer и кросс-оригинных воркеров
      'Cross-Origin-Opener-Policy': 'same-origin',
      'Cross-Origin-Embedder-Policy': 'require-corp',
    },
    // Proxy для Tauri IPC (если запущен Tauri dev server)
    proxy: isTauri ? {
      // API proxy для бэкенда (если нужен)
      '/api': {
        target: 'http://127.0.0.1:8000',
        changeOrigin: true,
      },
      // Tauri IPC вызовы обрабатываются через @tauri-apps/api
    } : {
      // Web режим
      '/api': {
        target: 'http://127.0.0.1:8000',
        changeOrigin: true,
      },
    }
  },
  worker: {
    // Настройки для Web Workers
    format: 'es',
    plugins: [],
  },
  build: {
    // Раздельные сборки для Tauri и Web
    target: isTauri ? 'es2021' : 'es2020',
    outDir: isTauri ? 'dist' : 'dist-web',
    // Настройки для WASM
    rollupOptions: {
      // Исключаем WASM из bundling для загрузки по требованию
      output: {
        manualChunks: {
          'pepakura-wasm': ['@pepakura_wasm'],
        },
      },
    },
  },
  define: {
    // Переменные окружения для платформ
    'process.env.TAURI_ENV_PLATFORM': JSON.stringify(process.env.TAURI_ENV_PLATFORM),
    '__TAURI_PLATFORM__': JSON.stringify(process.env.TAURI_ENV_PLATFORM ?? 'web'),
  },
})
