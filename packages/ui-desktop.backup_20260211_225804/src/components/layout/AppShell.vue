<template>
  <div class="shell-root">
    <header class="shell-top">
      <div class="brand" @click="goWelcome">
        <img
          :src="appIconUrl"
          alt="Pepakura Next"
          class="brand-logo"
        />
        <div class="brand-text">
          <span class="brand-main">Pepakura Next</span>
          <span class="brand-sub">TXT2D3DPaper</span>
        </div>
      </div>

      <nav class="top-nav">
        <button
          class="top-btn"
          :class="{ active: page === 'welcome' }"
          @click="setPage('welcome')"
        >
          Проекты
        </button>
        <button
          class="top-btn"
          :class="{ active: page === 'editor' }"
          @click="setPage('editor')"
        >
          Редактор
        </button>
        <button
          class="top-btn"
          :class="{ active: page === 'settings' }"
          @click="setPage('settings')"
        >
          Настройки
        </button>
      </nav>

      <div class="top-actions">
        <span class="top-hint">
          Экспериментальный AI‑режим. Интерфейс и поведение могут меняться.
        </span>
      </div>
    </header>

    <main class="shell-main">
      <WelcomePage v-if="page === 'welcome'" />
      <EditorPage v-else-if="page === 'editor'" />
      <SettingsPage v-else />
    </main>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted, onBeforeUnmount } from 'vue'
import { useUiStore } from '@/stores/ui'
import WelcomePage from '@/pages/WelcomePage.vue'
import EditorPage from '@/pages/EditorPage.vue'
import SettingsPage from '@/pages/SettingsPage.vue'
import appIconUrl from '@/assets/app-icon.png'

const ui = useUiStore()
const page = computed(() => ui.currentPage)

function setPage(p: 'welcome' | 'editor' | 'settings') {
  ui.setPage(p)
}

function goWelcome() {
  ui.setPage('welcome')
}

function preventGlobalFileDrop(e: DragEvent) {
  if (e.dataTransfer && e.dataTransfer.types.includes('Files')) {
    e.preventDefault()
  }
}

onMounted(() => {
  window.addEventListener('dragover', preventGlobalFileDrop)
  window.addEventListener('drop', preventGlobalFileDrop)
})

onBeforeUnmount(() => {
  window.removeEventListener('dragover', preventGlobalFileDrop)
  window.removeEventListener('drop', preventGlobalFileDrop)
})
</script>

<style scoped>
.shell-root {
  display: flex;
  flex-direction: column;
  height: 100vh;
  background: radial-gradient(circle at top left, #0f172a 0, #020617 55%, #000 100%);
  color: #e5e7eb;
  font-family: system-ui, -apple-system, BlinkMacSystemFont, 'Segoe UI',
    sans-serif;
}

.shell-top {
  height: 48px;
  padding: 0 0.9rem;
  border-bottom: 1px solid rgba(148, 163, 184, 0.6);
  display: flex;
  align-items: center;
  justify-content: space-between;
  background: rgba(15, 23, 42, 0.98);
}

.brand {
  display: flex;
  align-items: center;
  gap: 0.4rem;
  cursor: pointer;
}

.brand-logo {
  width: 24px;
  height: 24px;
  border-radius: 0.4rem;
}

.brand-text {
  display: flex;
  flex-direction: column;
  gap: 0.05rem;
}

.brand-main {
  font-size: 0.9rem;
  font-weight: 600;
}

.brand-sub {
  font-size: 0.75rem;
  color: #9ca3af;
}

.top-nav {
  display: flex;
  gap: 0.35rem;
}

.top-btn {
  border-radius: 999px;
  border: 1px solid transparent;
  background: rgba(15, 23, 42, 0.96);
  color: #e5e7eb;
  padding: 0.2rem 0.8rem;
  font-size: 0.8rem;
  cursor: pointer;
}

.top-btn.active {
  background: linear-gradient(135deg, #3b82f6, #6366f1);
  border-color: rgba(191, 219, 254, 0.9);
  color: #f9fafb;
}

.top-actions {
  display: flex;
  align-items: center;
}

.top-hint {
  font-size: 0.78rem;
  color: #cbd5e1;
}

.shell-main {
  flex: 1;
  min-height: 0;
}
</style>
