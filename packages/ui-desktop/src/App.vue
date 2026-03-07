<template>
  <div id="app">
    <!-- Глобальный layout -->
    <div class="app-layout">
      <PepakuraEditor />
      
      <!-- Глобальный toast контейнер -->
      <Teleport to="body">
        <div class="global-toast-container">
          <NotificationToast />
        </div>
      </Teleport>
    </div>
  </div>
</template>

<script setup lang="ts">
import { onMounted } from 'vue'
import NotificationToast from './components/NotificationToast.vue'
import PepakuraEditor from './components/PepakuraEditor.vue'
import { useProjectStore } from './stores/projectStore'
// глобальные стили

onMounted(() => {
  // Глобальный drag&drop prevent (дублируем из index.html)
  document.addEventListener('dragover', (e) => {
    e.preventDefault()
  }, true)
  
  document.addEventListener('drop', (e) => {
    e.preventDefault()
  }, true)
  
  // Инициализируем автосохранение
  const projectStore = useProjectStore()
  projectStore.initAutoSave()
})
</script>

<style>
:root {
  --primary: #6366f1;
  --primary-dark: #4f46e5;
  --bg-primary: #0f172a;
  --bg-secondary: #1e293b;
  --text-primary: #f8fafc;
  --text-secondary: #cbd5e1;
  --success: #10b981;
  --error: #ef4444;
  --warning: #f59e0b;
  --warning-color: #f59e0b;
}

* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

html, body {
  height: 100%;
  overflow: hidden;
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif;
  background: var(--bg-primary);
  color: var(--text-primary);
}

#app {
  height: 100vh;
  width: 100vw;
  position: relative;
}

.app-layout {
  height: 100%;
  width: 100%;
}

.global-toast-container {
  position: fixed;
  top: 20px;
  right: 20px;
  z-index: 9999;
  pointer-events: none;
}

.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.3s;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}

/* Скроллбар */
::-webkit-scrollbar {
  width: 8px;
}

::-webkit-scrollbar-track {
  background: rgba(30, 41, 59, 0.5);
  border-radius: 4px;
}

::-webkit-scrollbar-thumb {
  background: rgba(99, 102, 241, 0.6);
  border-radius: 4px;
}

::-webkit-scrollbar-thumb:hover {
  background: rgba(99, 102, 241, 0.8);
}
</style>

