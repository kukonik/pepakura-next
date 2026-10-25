import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import path from 'path'

// Режим работы: Tauri или веб
const isTauri = process.env.TAURI_ENV_PLATFORM != null
const apiProxy = { '/api': { target: 'http://127.0.0.1:8000', changeOrigin: true } }

export default defineConfig({
  plugins: [vue()],
  resolve: {
    alias: {
      '@': path.resolve(__dirname, './src'),
      '@shared': path.resolve(__dirname, './packages/shared/src'),
      '@core': path.resolve(__dirname, './packages/core/src'),
      '@frontend': path.resolve(__dirname, './frontend/src'),
      // '@pepakura_wasm' оставить только если код реально импортирует его — проверка ниже
    },
  },
  server: {
    port: 5174,
    strictPort: true,
    headers: {
      'Cross-Origin-Opener-Policy': 'same-origin',
      'Cross-Origin-Embedder-Policy': 'require-corp',
    },
    proxy: apiProxy, // ветки isTauri/web были идентичны — оставлен один блок
  },
  worker: { format: 'es' },
  build: {
    target: isTauri ? 'es2021' : 'es2020',
    outDir: isTauri ? 'dist' : 'dist-web',
    // manualChunks для несуществующего пакета '@pepakura_wasm' удалён.
    // WASM подключать после wasm-pack build из crates/pepakura_wasm/pkg.
  },
  define: {
    'process.env.TAURI_ENV_PLATFORM': JSON.stringify(process.env.TAURI_ENV_PLATFORM),
    '__TAURI_PLATFORM__': JSON.stringify(process.env.TAURI_ENV_PLATFORM ?? 'web'),
  },
})