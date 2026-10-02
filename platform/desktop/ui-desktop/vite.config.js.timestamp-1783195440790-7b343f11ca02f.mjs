// vite.config.js
import { defineConfig } from "file:///D:/Dev/pepakura-next/node_modules/.pnpm/vite@5.4.21_@types+node@20.19.41/node_modules/vite/dist/node/index.js";
import vue from "file:///D:/Dev/pepakura-next/node_modules/.pnpm/@vitejs+plugin-vue@5.2.4_vi_33550c44c85bfd9de625a03d9c6d8284/node_modules/@vitejs/plugin-vue/dist/index.mjs";
import { fileURLToPath, URL } from "node:url";
var __vite_injected_original_import_meta_url = "file:///D:/Dev/pepakura-next/platform/desktop/ui-desktop/vite.config.js";
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
//# sourceMappingURL=data:application/json;base64,ewogICJ2ZXJzaW9uIjogMywKICAic291cmNlcyI6IFsidml0ZS5jb25maWcuanMiXSwKICAic291cmNlc0NvbnRlbnQiOiBbImNvbnN0IF9fdml0ZV9pbmplY3RlZF9vcmlnaW5hbF9kaXJuYW1lID0gXCJEOlxcXFxEZXZcXFxccGVwYWt1cmEtbmV4dFxcXFxwbGF0Zm9ybVxcXFxkZXNrdG9wXFxcXHVpLWRlc2t0b3BcIjtjb25zdCBfX3ZpdGVfaW5qZWN0ZWRfb3JpZ2luYWxfZmlsZW5hbWUgPSBcIkQ6XFxcXERldlxcXFxwZXBha3VyYS1uZXh0XFxcXHBsYXRmb3JtXFxcXGRlc2t0b3BcXFxcdWktZGVza3RvcFxcXFx2aXRlLmNvbmZpZy5qc1wiO2NvbnN0IF9fdml0ZV9pbmplY3RlZF9vcmlnaW5hbF9pbXBvcnRfbWV0YV91cmwgPSBcImZpbGU6Ly8vRDovRGV2L3BlcGFrdXJhLW5leHQvcGxhdGZvcm0vZGVza3RvcC91aS1kZXNrdG9wL3ZpdGUuY29uZmlnLmpzXCI7aW1wb3J0IHsgZGVmaW5lQ29uZmlnIH0gZnJvbSBcInZpdGVcIjtcclxuaW1wb3J0IHZ1ZSBmcm9tIFwiQHZpdGVqcy9wbHVnaW4tdnVlXCI7XHJcbmltcG9ydCB7IGZpbGVVUkxUb1BhdGgsIFVSTCB9IGZyb20gXCJub2RlOnVybFwiO1xyXG5cclxuLy8gaHR0cHM6Ly92aXRlanMuZGV2L2NvbmZpZy9cclxuZXhwb3J0IGRlZmF1bHQgZGVmaW5lQ29uZmlnKGFzeW5jICgpID0+ICh7XHJcbiAgcGx1Z2luczogW3Z1ZSgpXSxcclxuXHJcbiAgcmVzb2x2ZToge1xyXG4gICAgYWxpYXM6IHtcclxuICAgICAgXCJAXCI6IGZpbGVVUkxUb1BhdGgobmV3IFVSTChcIi4vc3JjXCIsIGltcG9ydC5tZXRhLnVybCkpLFxyXG4gICAgICBcIkBjb21wb25lbnRzXCI6IGZpbGVVUkxUb1BhdGgobmV3IFVSTChcIi4vc3JjL2NvbXBvbmVudHNcIiwgaW1wb3J0Lm1ldGEudXJsKSksXHJcbiAgICAgIFwiQHN0b3Jlc1wiOiBmaWxlVVJMVG9QYXRoKG5ldyBVUkwoXCIuL3NyYy9zdG9yZXNcIiwgaW1wb3J0Lm1ldGEudXJsKSksXHJcbiAgICAgIFwiQHZpZXdzXCI6IGZpbGVVUkxUb1BhdGgobmV3IFVSTChcIi4vc3JjL3ZpZXdzXCIsIGltcG9ydC5tZXRhLnVybCkpLFxyXG4gICAgICBcIkBtb2R1bGVzXCI6IGZpbGVVUkxUb1BhdGgobmV3IFVSTChcIi4vc3JjL21vZHVsZXNcIiwgaW1wb3J0Lm1ldGEudXJsKSksXHJcbiAgICAgIFwiQGNvbXBvc2FibGVzXCI6IGZpbGVVUkxUb1BhdGgobmV3IFVSTChcIi4vc3JjL2NvbXBvc2FibGVzXCIsIGltcG9ydC5tZXRhLnVybCkpLFxyXG4gICAgICBcIkB0eXBlc1wiOiBmaWxlVVJMVG9QYXRoKG5ldyBVUkwoXCIuL3NyYy90eXBlc1wiLCBpbXBvcnQubWV0YS51cmwpKSxcclxuICAgICAgXCJAc2hhcmVkXCI6IGZpbGVVUkxUb1BhdGgobmV3IFVSTChcIi4uLy4uLy4uL3NoYXJlZC9zcmNcIiwgaW1wb3J0Lm1ldGEudXJsKSksXHJcbiAgICAgIFwiQHBlcGFrdXJhL3NoYXJlZFwiOiBmaWxlVVJMVG9QYXRoKG5ldyBVUkwoXCIuLi8uLi8uLi9zaGFyZWQvc3JjXCIsIGltcG9ydC5tZXRhLnVybCkpLFxyXG4gICAgfSxcclxuICB9LFxyXG5cclxuICAvLyBWaXRlIG9wdGlvbnMgdGFpbG9yZWQgZm9yIFRhdXJpIGRldmVsb3BtZW50IGFuZCBvbmx5IGFwcGxpZWQgaW4gYHRhdXJpIGRldmAgb3IgYHRhdXJpIGJ1aWxkYFxyXG4gIC8vXHJcbiAgLy8gMS4gcHJldmVudCB2aXRlIGZyb20gb2JzY3VyaW5nIHJ1c3QgZXJyb3JzXHJcbiAgY2xlYXJTY3JlZW46IGZhbHNlLFxyXG4gIC8vIDIuIHRhdXJpIGV4cGVjdHMgYSBmaXhlZCBwb3J0LCBmYWlsIGlmIHRoYXQgcG9ydCBpcyBub3QgYXZhaWxhYmxlXHJcbiAgc2VydmVyOiB7XHJcbiAgICBwb3J0OiA1MTczLFxyXG4gICAgc3RyaWN0UG9ydDogdHJ1ZSxcclxuICAgIHdhdGNoOiB7XHJcbiAgICAgIC8vIDMuIHRlbGwgdml0ZSB0byBpZ25vcmUgd2F0Y2hpbmcgYHNyYy10YXVyaWBcclxuICAgICAgaWdub3JlZDogW1wiKiovc3JjLXRhdXJpLyoqXCJdLFxyXG4gICAgfSxcclxuICB9LFxyXG59KSk7Il0sCiAgIm1hcHBpbmdzIjogIjtBQUE4VSxTQUFTLG9CQUFvQjtBQUMzVyxPQUFPLFNBQVM7QUFDaEIsU0FBUyxlQUFlLFdBQVc7QUFGZ0wsSUFBTSwyQ0FBMkM7QUFLcFEsSUFBTyxzQkFBUSxhQUFhLGFBQWE7QUFBQSxFQUN2QyxTQUFTLENBQUMsSUFBSSxDQUFDO0FBQUEsRUFFZixTQUFTO0FBQUEsSUFDUCxPQUFPO0FBQUEsTUFDTCxLQUFLLGNBQWMsSUFBSSxJQUFJLFNBQVMsd0NBQWUsQ0FBQztBQUFBLE1BQ3BELGVBQWUsY0FBYyxJQUFJLElBQUksb0JBQW9CLHdDQUFlLENBQUM7QUFBQSxNQUN6RSxXQUFXLGNBQWMsSUFBSSxJQUFJLGdCQUFnQix3Q0FBZSxDQUFDO0FBQUEsTUFDakUsVUFBVSxjQUFjLElBQUksSUFBSSxlQUFlLHdDQUFlLENBQUM7QUFBQSxNQUMvRCxZQUFZLGNBQWMsSUFBSSxJQUFJLGlCQUFpQix3Q0FBZSxDQUFDO0FBQUEsTUFDbkUsZ0JBQWdCLGNBQWMsSUFBSSxJQUFJLHFCQUFxQix3Q0FBZSxDQUFDO0FBQUEsTUFDM0UsVUFBVSxjQUFjLElBQUksSUFBSSxlQUFlLHdDQUFlLENBQUM7QUFBQSxNQUMvRCxXQUFXLGNBQWMsSUFBSSxJQUFJLHVCQUF1Qix3Q0FBZSxDQUFDO0FBQUEsTUFDeEUsb0JBQW9CLGNBQWMsSUFBSSxJQUFJLHVCQUF1Qix3Q0FBZSxDQUFDO0FBQUEsSUFDbkY7QUFBQSxFQUNGO0FBQUE7QUFBQTtBQUFBO0FBQUEsRUFLQSxhQUFhO0FBQUE7QUFBQSxFQUViLFFBQVE7QUFBQSxJQUNOLE1BQU07QUFBQSxJQUNOLFlBQVk7QUFBQSxJQUNaLE9BQU87QUFBQTtBQUFBLE1BRUwsU0FBUyxDQUFDLGlCQUFpQjtBQUFBLElBQzdCO0FBQUEsRUFDRjtBQUNGLEVBQUU7IiwKICAibmFtZXMiOiBbXQp9Cg==
