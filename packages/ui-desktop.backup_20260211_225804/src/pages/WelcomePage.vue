<template>
  <div class="welcome-root">
    <section class="hero">
      <h1 class="title">Проекты</h1>
      <p class="subtitle">
        Создайте новый проект или откройте существующий, чтобы перейти к редактору и развёртке.
      </p>

      <div class="actions">
        <button class="primary-btn" @click="createProject">
          Создать новый проект
        </button>
        <button class="secondary-btn" @click="openProject">
          Открыть существующий
        </button>
      </div>

      <p class="hint">
        Новый проект откроется сразу в редакторе текста описания модели.
      </p>
    </section>

    <section class="recent">
      <h2 class="recent-title">Последние проекты</h2>
      <p class="recent-empty">
        Список недавних проектов будет появляться здесь.
      </p>
    </section>
  </div>
</template>

<script setup lang="ts">
import { useUiStore } from '@/stores/ui'
import { useProjectStore } from '@/stores/project'
import { useTauriModelLoader } from '@/composables/useTauriModelLoader'

const ui = useUiStore()
const project = useProjectStore()
const { showMessage } = useTauriModelLoader()

function createProject() {
  project.id = null
  project.name = 'Новый проект'
  project.createdAt = new Date().toISOString()
  project.updatedAt = project.createdAt
  project.resetThreeD()
  project.resetUnfold()
  project.setText({ rawText: '' })
  ui.setPage('editor')
  project.setStage('text')
}

function openProject() {
  showMessage(
    'Открытие существующего проекта пока не реализовано. Будет добавлен диалог выбора файла проекта.',
    'info'
  )
}
</script>

<style scoped>
.welcome-root {
  height: 100%;
  display: flex;
  flex-direction: column;
  padding: 1.2rem 1.6rem;
  gap: 1.2rem;
  color: #e5e7eb;
}

.hero {
  max-width: 640px;
}

.title {
  font-size: 1.4rem;
  margin: 0 0 0.3rem;
  font-weight: 600;
}

.subtitle {
  font-size: 0.9rem;
  color: #cbd5e1;
  margin: 0 0 0.7rem;
}

.actions {
  display: flex;
  flex-wrap: wrap;
  gap: 0.5rem;
  margin-bottom: 0.4rem;
}

.primary-btn {
  border-radius: 999px;
  border: 1px solid transparent;
  padding: 0.4rem 1.1rem;
  font-size: 0.9rem;
  cursor: pointer;
  background: linear-gradient(135deg, #22c55e, #16a34a);
  color: #f9fafb;
}

.secondary-btn {
  border-radius: 999px;
  border: 1px solid rgba(148, 163, 184, 0.8);
  padding: 0.4rem 1rem;
  font-size: 0.9rem;
  cursor: pointer;
  background: rgba(15, 23, 42, 0.96);
  color: #e5e7eb;
}

.hint {
  font-size: 0.8rem;
  color: #9ca3af;
}

.recent {
  margin-top: 0.4rem;
}

.recent-title {
  font-size: 0.95rem;
  margin: 0 0 0.2rem;
}

.recent-empty {
  font-size: 0.82rem;
  color: #9ca3af;
  margin: 0;
}
</style>
