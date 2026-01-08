<template>
  <aside class="left-toolbar">
    <!-- Режим TXT -->
    <button
      class="tool-btn"
      :class="{ active: stage === 'txt' }"
      @click="setStage('txt')"
      title="Текст → модель"
    >
      <i class="fas fa-font"></i>
      <span class="tool-label">TXT</span>
    </button>

    <!-- Режим 2D -->
    <button
      class="tool-btn"
      :class="{ active: stage === '2d' }"
      @click="setStage('2d')"
      title="2D редактор"
    >
      <i class="fas fa-image"></i>
      <span class="tool-label">2D</span>
    </button>

    <!-- Режим 3D -->
    <button
      class="tool-btn"
      :class="{ active: stage === '3d' }"
      @click="setStage('3d')"
      title="3D редактор и просмотрщик"
    >
      <i class="fas fa-cube"></i>
      <span class="tool-label">3D</span>
    </button>

    <!-- Режим развёртки -->
    <button
      class="tool-btn"
      :class="{ active: stage === 'unfold' }"
      @click="setStage('unfold')"
      title="Бумажная развёртка"
    >
      <i class="fas fa-cut"></i>
      <span class="tool-label">Paper</span>
    </button>

    <!-- Нижние утилиты -->
    <div class="bottom-group">
      <button class="tool-btn" @click="toggleTheme" title="Тема">
        <i :class="themeIcon"></i>
        <span class="tool-label">Тема</span>
      </button>

      <button class="tool-btn" @click="showHelp" title="Помощь">
        <i class="fas fa-question-circle"></i>
        <span class="tool-label">Помощь</span>
      </button>
    </div>
  </aside>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useProjectStore } from '@/stores/project'
import { useTauriModelLoader } from '@/composables/useTauriModelLoader'

const project = useProjectStore()
const { showMessage } = useTauriModelLoader()

const stage = computed(() => project.stage)

const darkTheme = computed({
  get: () => !document.body.classList.contains('light-theme'),
  set: v => {
    if (v) document.body.classList.remove('light-theme')
    else document.body.classList.add('light-theme')
  }
})

const themeIcon = computed(() =>
  darkTheme.value ? 'fas fa-moon' : 'fas fa-sun'
)

function setStage(s: 'txt' | '2d' | '3d' | 'unfold') {
  project.setStage(s)
  showMessage(`Режим: ${s.toUpperCase()}`, 'info')
}

function toggleTheme() {
  darkTheme.value = !darkTheme.value
  showMessage(
    darkTheme.value ? 'Переключено на тёмную тему' : 'Переключено на светлую тему',
    'info'
  )
}

function showHelp() {
  showMessage('Открыта справка (заглушка)', 'info')
}
</script>

<style scoped>
.left-toolbar {
  width: 70px;
  background: rgba(15, 23, 42, 0.7);
  border-right: 1px solid rgba(255, 255, 255, 0.08);
  padding: 1.5rem 0.4rem;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 1rem;
  flex-shrink: 0;
}

.tool-btn {
  width: 50px;
  height: 50px;
  background: rgba(30, 41, 59, 0.8);
  border-radius: 12px;
  border: 1px solid rgba(255, 255, 255, 0.1);
  color: #cbd5e1;
  cursor: pointer;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 4px;
  font-size: 0.72rem;
  transition: all 0.2s;
  flex-shrink: 0;
}

.tool-btn i {
  font-size: 1rem;
}

.tool-btn:hover {
  background: rgba(56, 70, 95, 0.9);
  border-color: #60a5fa;
  color: #ffffff;
}

.tool-btn.active {
  background: linear-gradient(135deg, #3b82f6, #6366f1);
  color: #ffffff;
  border: none;
}

.tool-label {
  display: block;
  max-width: 100%;
  text-align: center;
  font-size: 0.7rem;
  line-height: 1.1;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.bottom-group {
  margin-top: auto;
  display: flex;
  flex-direction: column;
  gap: 0.8rem;
}
</style>
