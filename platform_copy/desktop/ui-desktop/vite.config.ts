import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import { fileURLToPath, URL } from 'node:url'
import { tauriPlugin } from '@tauri-apps/plugin-vite'

export default defineConfig({
  plugins: [
    vue(),
    tauriPlugin(),
    // Плагин для исправления путей в index.html после сборки
    {
      name: 'fix-relative-paths',
      transformIndexHtml(html) {
        return html
          .replace(/src="\/assets\//g, 'src="./assets/')
          .replace(/href="\/assets\//g, 'href="./assets/')
      }
    }
  ],
  resolve: {
    alias: {
      '@': fileURLToPath(new URL('./src', import.meta.url)),
      '@components': fileURLToPath(new URL('./src/components', import.meta.url)),
      '@stores': fileURLToPath(new URL('./src/stores', import.meta.url)),
      '@views': fileURLToPath(new URL('./src/views', import.meta.url)),
      '@modules': fileURLToPath(new URL('./src/modules', import.meta.url)),
      '@composables': fileURLToPath(new URL('./src/composables', import.meta.url)),
      '@types': fileURLToPath(new URL('./src/types', import.meta.url)),
      '@shared': fileURLToPath(new URL('../../../shared/src', import.meta.url)),
      '@pepakura/shared': fileURLToPath(new URL('../../../shared/src', import.meta.url))
    }
  },
  optimizeDeps: {
    exclude: ['@tauri-apps/api', 'three', '*.svg', '*.html']
  },
  server: {
    port: 5173,
    strictPort: true,
    hmr: {
      overlay: true
    }
  },
  build: {
    outDir: 'dist',
    base: './',
    rollupOptions: {
      output: {
        manualChunks: {
          'vendor': ['vue', 'pinia'],
          'three': ['three']
        },
        // Исправляем пути к ассетам для относительных ссылок
        assetFileNames: 'assets/[name]-[hash][extname]',
        chunkFileNames: 'assets/[name]-[hash].js',
        entryFileNames: 'assets/[name]-[hash].js'
      }
    }
  }
}