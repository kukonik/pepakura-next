// vite.config.js
import { defineConfig } from "file:///E:/Dev/pepakura-next/node_modules/.pnpm/vite@5.4.21_@types+node@20.19.39/node_modules/vite/dist/node/index.js";
import vue from "file:///E:/Dev/pepakura-next/node_modules/.pnpm/@vitejs+plugin-vue@5.2.4_vi_8387039c0a598881f736fe947955aa9f/node_modules/@vitejs/plugin-vue/dist/index.mjs";
import { fileURLToPath, URL } from "node:url";
var __vite_injected_original_import_meta_url = "file:///E:/Dev/pepakura-next/platform/desktop/ui-desktop/vite.config.js";
var vite_config_default = defineConfig(async () => ({
  plugins: [vue()],
  resolve: {
    alias: {
      "@": fileURLToPath(new URL("./src", __vite_injected_original_import_meta_url)),
      "@components": fileURLToPath(new URL("./src/components", __vite_injected_original_import_meta_url)),
      "@stores": fileURLToPath(new URL("./src/stores", __vite_injected_original_import_meta_url)),
      "@views": fileURLToPath(new URL("./src/views", __vite_injected_original_import_meta_url)),
      "@modules": fileURLToPath(new URL("./src/modules", __vite_injected_original_import_meta_url)),
      "@composables": fileURLToPath(new URL("./src/composables", __vite_injected_original_import_meta_url)),
      "@types": fileURLToPath(new URL("./src/types", __vite_injected_original_import_meta_url)),
      "@shared": fileURLToPath(new URL("../../../shared/src", __vite_injected_original_import_meta_url)),
      "@pepakura/shared": fileURLToPath(new URL("../../../shared/src", __vite_injected_original_import_meta_url))
    }
  },
  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  //
  // 1. prevent vite from obscuring rust errors
  clearScreen: false,
  // 2. tauri expects a fixed port, fail if that port is not available
  server: {
    port: 5173,
    strictPort: true,
    watch: {
      // 3. tell vite to ignore watching `src-tauri`
      ignored: ["**/src-tauri/**"]
    }
  }
}));
export {
  vite_config_default as default
};
//# sourceMappingURL=data:application/json;base64,ewogICJ2ZXJzaW9uIjogMywKICAic291cmNlcyI6IFsidml0ZS5jb25maWcuanMiXSwKICAic291cmNlc0NvbnRlbnQiOiBbImNvbnN0IF9fdml0ZV9pbmplY3RlZF9vcmlnaW5hbF9kaXJuYW1lID0gXCJFOlxcXFxEZXZcXFxccGVwYWt1cmEtbmV4dFxcXFxwbGF0Zm9ybVxcXFxkZXNrdG9wXFxcXHVpLWRlc2t0b3BcIjtjb25zdCBfX3ZpdGVfaW5qZWN0ZWRfb3JpZ2luYWxfZmlsZW5hbWUgPSBcIkU6XFxcXERldlxcXFxwZXBha3VyYS1uZXh0XFxcXHBsYXRmb3JtXFxcXGRlc2t0b3BcXFxcdWktZGVza3RvcFxcXFx2aXRlLmNvbmZpZy5qc1wiO2NvbnN0IF9fdml0ZV9pbmplY3RlZF9vcmlnaW5hbF9pbXBvcnRfbWV0YV91cmwgPSBcImZpbGU6Ly8vRTovRGV2L3BlcGFrdXJhLW5leHQvcGxhdGZvcm0vZGVza3RvcC91aS1kZXNrdG9wL3ZpdGUuY29uZmlnLmpzXCI7aW1wb3J0IHsgZGVmaW5lQ29uZmlnIH0gZnJvbSBcInZpdGVcIjtcbmltcG9ydCB2dWUgZnJvbSBcIkB2aXRlanMvcGx1Z2luLXZ1ZVwiO1xuaW1wb3J0IHsgZmlsZVVSTFRvUGF0aCwgVVJMIH0gZnJvbSBcIm5vZGU6dXJsXCI7XG5cbi8vIGh0dHBzOi8vdml0ZWpzLmRldi9jb25maWcvXG5leHBvcnQgZGVmYXVsdCBkZWZpbmVDb25maWcoYXN5bmMgKCkgPT4gKHtcbiAgcGx1Z2luczogW3Z1ZSgpXSxcblxuICByZXNvbHZlOiB7XG4gICAgYWxpYXM6IHtcbiAgICAgIFwiQFwiOiBmaWxlVVJMVG9QYXRoKG5ldyBVUkwoXCIuL3NyY1wiLCBpbXBvcnQubWV0YS51cmwpKSxcbiAgICAgIFwiQGNvbXBvbmVudHNcIjogZmlsZVVSTFRvUGF0aChuZXcgVVJMKFwiLi9zcmMvY29tcG9uZW50c1wiLCBpbXBvcnQubWV0YS51cmwpKSxcbiAgICAgIFwiQHN0b3Jlc1wiOiBmaWxlVVJMVG9QYXRoKG5ldyBVUkwoXCIuL3NyYy9zdG9yZXNcIiwgaW1wb3J0Lm1ldGEudXJsKSksXG4gICAgICBcIkB2aWV3c1wiOiBmaWxlVVJMVG9QYXRoKG5ldyBVUkwoXCIuL3NyYy92aWV3c1wiLCBpbXBvcnQubWV0YS51cmwpKSxcbiAgICAgIFwiQG1vZHVsZXNcIjogZmlsZVVSTFRvUGF0aChuZXcgVVJMKFwiLi9zcmMvbW9kdWxlc1wiLCBpbXBvcnQubWV0YS51cmwpKSxcbiAgICAgIFwiQGNvbXBvc2FibGVzXCI6IGZpbGVVUkxUb1BhdGgobmV3IFVSTChcIi4vc3JjL2NvbXBvc2FibGVzXCIsIGltcG9ydC5tZXRhLnVybCkpLFxuICAgICAgXCJAdHlwZXNcIjogZmlsZVVSTFRvUGF0aChuZXcgVVJMKFwiLi9zcmMvdHlwZXNcIiwgaW1wb3J0Lm1ldGEudXJsKSksXG4gICAgICBcIkBzaGFyZWRcIjogZmlsZVVSTFRvUGF0aChuZXcgVVJMKFwiLi4vLi4vLi4vc2hhcmVkL3NyY1wiLCBpbXBvcnQubWV0YS51cmwpKSxcbiAgICAgIFwiQHBlcGFrdXJhL3NoYXJlZFwiOiBmaWxlVVJMVG9QYXRoKG5ldyBVUkwoXCIuLi8uLi8uLi9zaGFyZWQvc3JjXCIsIGltcG9ydC5tZXRhLnVybCkpLFxuICAgIH0sXG4gIH0sXG5cbiAgLy8gVml0ZSBvcHRpb25zIHRhaWxvcmVkIGZvciBUYXVyaSBkZXZlbG9wbWVudCBhbmQgb25seSBhcHBsaWVkIGluIGB0YXVyaSBkZXZgIG9yIGB0YXVyaSBidWlsZGBcbiAgLy9cbiAgLy8gMS4gcHJldmVudCB2aXRlIGZyb20gb2JzY3VyaW5nIHJ1c3QgZXJyb3JzXG4gIGNsZWFyU2NyZWVuOiBmYWxzZSxcbiAgLy8gMi4gdGF1cmkgZXhwZWN0cyBhIGZpeGVkIHBvcnQsIGZhaWwgaWYgdGhhdCBwb3J0IGlzIG5vdCBhdmFpbGFibGVcbiAgc2VydmVyOiB7XG4gICAgcG9ydDogNTE3MyxcbiAgICBzdHJpY3RQb3J0OiB0cnVlLFxuICAgIHdhdGNoOiB7XG4gICAgICAvLyAzLiB0ZWxsIHZpdGUgdG8gaWdub3JlIHdhdGNoaW5nIGBzcmMtdGF1cmlgXG4gICAgICBpZ25vcmVkOiBbXCIqKi9zcmMtdGF1cmkvKipcIl0sXG4gICAgfSxcbiAgfSxcbn0pKTsiXSwKICAibWFwcGluZ3MiOiAiO0FBQThVLFNBQVMsb0JBQW9CO0FBQzNXLE9BQU8sU0FBUztBQUNoQixTQUFTLGVBQWUsV0FBVztBQUZnTCxJQUFNLDJDQUEyQztBQUtwUSxJQUFPLHNCQUFRLGFBQWEsYUFBYTtBQUFBLEVBQ3ZDLFNBQVMsQ0FBQyxJQUFJLENBQUM7QUFBQSxFQUVmLFNBQVM7QUFBQSxJQUNQLE9BQU87QUFBQSxNQUNMLEtBQUssY0FBYyxJQUFJLElBQUksU0FBUyx3Q0FBZSxDQUFDO0FBQUEsTUFDcEQsZUFBZSxjQUFjLElBQUksSUFBSSxvQkFBb0Isd0NBQWUsQ0FBQztBQUFBLE1BQ3pFLFdBQVcsY0FBYyxJQUFJLElBQUksZ0JBQWdCLHdDQUFlLENBQUM7QUFBQSxNQUNqRSxVQUFVLGNBQWMsSUFBSSxJQUFJLGVBQWUsd0NBQWUsQ0FBQztBQUFBLE1BQy9ELFlBQVksY0FBYyxJQUFJLElBQUksaUJBQWlCLHdDQUFlLENBQUM7QUFBQSxNQUNuRSxnQkFBZ0IsY0FBYyxJQUFJLElBQUkscUJBQXFCLHdDQUFlLENBQUM7QUFBQSxNQUMzRSxVQUFVLGNBQWMsSUFBSSxJQUFJLGVBQWUsd0NBQWUsQ0FBQztBQUFBLE1BQy9ELFdBQVcsY0FBYyxJQUFJLElBQUksdUJBQXVCLHdDQUFlLENBQUM7QUFBQSxNQUN4RSxvQkFBb0IsY0FBYyxJQUFJLElBQUksdUJBQXVCLHdDQUFlLENBQUM7QUFBQSxJQUNuRjtBQUFBLEVBQ0Y7QUFBQTtBQUFBO0FBQUE7QUFBQSxFQUtBLGFBQWE7QUFBQTtBQUFBLEVBRWIsUUFBUTtBQUFBLElBQ04sTUFBTTtBQUFBLElBQ04sWUFBWTtBQUFBLElBQ1osT0FBTztBQUFBO0FBQUEsTUFFTCxTQUFTLENBQUMsaUJBQWlCO0FBQUEsSUFDN0I7QUFBQSxFQUNGO0FBQ0YsRUFBRTsiLAogICJuYW1lcyI6IFtdCn0K
