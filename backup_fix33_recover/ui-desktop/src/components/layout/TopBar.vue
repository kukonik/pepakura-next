<template>
  <header class="top-bar">
    <div class="left-group">
      <div class="logo">
        <img :src="logoSrc" alt="Pepakura Next" class="logo-img" />
        <span class="logo-text">Pepakura Next</span>
      </div>

      <div class="search-container">
        <i class="fas fa-search search-icon"></i>
        <input
          v-model="searchQuery"
          type="text"
          class="search-input"
          placeholder="Умный поиск по проекту и AI..."
          @keyup.enter="onSearch"
        />
        <button class="search-ai-btn" @click="onAISearch">
          <i class="fas fa-robot"></i>
          AI
        </button>
        <button class="search-web-btn" @click="onWebSearch">
          <i class="fas fa-globe"></i>
          Web
        </button>
      </div>
    </div>

    <div class="spacer"></div>

    <div class="right-group">
      <button class="top-btn" @click="onNewProject">
        <i class="fas fa-file"></i>
        Новый
      </button>

      <button class="top-btn" @click="onOpenProject">
        <i class="fas fa-folder-open"></i>
        Открыть
      </button>

      <button class="top-btn primary" @click="onSaveProject">
        <i class="fas fa-save"></i>
        Сохранить
      </button>

      <button class="icon-btn" @click="onSettings">
        <i class="fas fa-cog"></i>
      </button>
    </div>
  </header>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import logoSrc from '../../assets/app-icon.png'
import { useTauriModelLoader } from '../../composables/useTauriModelLoader'

const {
  showMessage,
  updateAITip,
  openExternal,
  saveProject,
  loadProject
} = useTauriModelLoader()

const searchQuery = ref('')

function onSearch() {
  const q = searchQuery.value.trim()
  if (!q) {
    showMessage('Введите запрос для поиска', 'warning')
    return
  }
  showMessage(`Локальный поиск: "${q}" (заглушка)`, 'info')
}

function onAISearch() {
  const q = searchQuery.value.trim()
  if (!q) {
    updateAITip('Попробуйте ввести, что хотите сделать, и нажмите AI.')
    showMessage('Сначала введите запрос для AI', 'warning')
    return
  }
  showMessage(`AI поиск по запросу: "${q}" (заглушка)`, 'info')
  updateAITip(`AI анализирует запрос: "${q}"...`)
}

async function onWebSearch() {
  const q = searchQuery.value.trim()
  if (!q) {
    showMessage('Введите запрос для Web‑поиска', 'warning')
    return
  }
  const url = `https://www.google.com/search?q=${encodeURIComponent(q)}`
  await openExternal(url)
}

function onNewProject() {
  showMessage('Создание нового проекта (заглушка)', 'info')
}

async function onOpenProject() {
  await loadProject()
}

async function onSaveProject() {
  await saveProject()
}

function onSettings() {
  showMessage('Окно настроек пока не реализовано', 'info')
}
</script>

<style scoped>
.top-bar {
  display: flex;
  align-items: center;
  padding: 0.6rem 1rem;
  background: radial-gradient(circle at top left, #0f172a, #020617 55%, #020617);
  border-bottom: 1px solid rgba(148, 163, 184, 0.3);
  gap: 0.75rem;
}

.left-group {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  min-width: 0;
}

.spacer {
  flex: 1 1 auto;
}

.right-group {
  display: flex;
  align-items: center;
  gap: 0.4rem;
}

.logo {
  display: flex;
  align-items: center;
  gap: 0.45rem;
  padding-right: 0.75rem;
  border-right: 1px solid rgba(148, 163, 184, 0.25);
  white-space: nowrap;
}

.logo-img {
  width: 26px;
  height: 26px;
  border-radius: 8px;
  box-shadow: 0 0 12px rgba(56, 189, 248, 0.7);
}

.logo-text {
  font-weight: 600;
  font-size: 0.95rem;
  color: #e2e8f0;
}

.search-container {
  display: flex;
  align-items: center;
  gap: 0.35rem;
  padding: 0.25rem 0.6rem;
  border-radius: 999px;
  background: rgba(15, 23, 42, 0.96);
  border: 1px solid rgba(51, 65, 85, 0.9);
  min-width: 380px;
  max-width: 760px;
  flex: 1 1 auto;
}

.search-icon {
  color: #64748b;
  font-size: 0.85rem;
}

.search-input {
  flex: 1 1 auto;
  border: none;
  background: transparent;
  color: #e5e7eb;
  font-size: 0.85rem;
  outline: none;
  min-width: 0;
}

.search-input::placeholder {
  color: #64748b;
}

.search-ai-btn,
.search-web-btn {
  border: none;
  border-radius: 999px;
  padding: 0.25rem 0.65rem;
  font-size: 0.8rem;
  display: flex;
  align-items: center;
  gap: 0.3rem;
  cursor: pointer;
  white-space: nowrap;
}

.search-ai-btn {
  background: linear-gradient(135deg, #22c55e, #a3e635);
  color: #0f172a;
  box-shadow: 0 0 10px rgba(34, 197, 94, 0.6);
}

.search-web-btn {
  background: linear-gradient(135deg, #0ea5e9, #6366f1);
  color: #f9fafb;
  box-shadow: 0 0 12px rgba(56, 189, 248, 0.7);
}

.search-ai-btn i,
.search-web-btn i {
  font-size: 0.8rem;
}

.top-btn {
  display: flex;
  align-items: center;
  gap: 0.35rem;
  padding: 0.35rem 0.7rem;
  border-radius: 999px;
  border: 1px solid rgba(148, 163, 184, 0.5);
  background: rgba(15, 23, 42, 0.95);
  color: #e5e7eb;
  font-size: 0.8rem;
  cursor: pointer;
  white-space: nowrap;
}

.top-btn.primary {
  background: linear-gradient(135deg, #3b82f6, #6366f1);
  border-color: transparent;
  box-shadow: 0 0 10px rgba(59, 130, 246, 0.6);
}

.top-btn i {
  font-size: 0.8rem;
}

.top-btn:hover {
  filter: brightness(1.05);
}

.icon-btn {
  width: 32px;
  height: 32px;
  border-radius: 999px;
  border: 1px solid rgba(148, 163, 184, 0.6);
  background: rgba(15, 23, 42, 0.96);
  color: #e5e7eb;
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
}

.icon-btn:hover {
  background: rgba(30, 64, 175, 0.95);
  border-color: rgba(59, 130, 246, 0.9);
}
</style>
