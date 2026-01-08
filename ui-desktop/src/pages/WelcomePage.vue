<template>
  <div class="welcome-root">
    <section class="hero">
      <h1 class="title">Pepakura Next</h1>
      <p class="subtitle">
        Простое и современное приложение для создания бумажных 3D‑моделей:
        от текста и референсов до развёртки на листы.
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
        Начните с пустого проекта или загрузите уже существующую 3D‑модель, чтобы сразу перейти к развёртке.
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
  padding: 1.5rem 1.8rem;
  gap: 1.4rem;
  color: #e5e7eb;
}

.hero {
  max-width: 640px;
}

.title {
  font-size: 2rem;
  margin: 0 0 0.4rem;
  font-weight: 700;
}

.subtitle {
  font-size: 0.95rem;
  color: #cbd5e1;
  margin: 0 0 0.9rem;
}

.actions {
  display: flex;
  gap: 0.6rem;
  margin-bottom: 0.5rem;
}

.primary-btn {
  border-radius: 999px;
  border: 1px solid transparent;
  padding: 0.45rem 1.2rem;
  font-size: 0.9rem;
  cursor: pointer;
  background: linear-gradient(135deg, #22c55e, #16a34a);
  color: #f9fafb;
}

.secondary-btn {
  border-radius: 999px;
  border: 1px solid rgba(148, 163, 184, 0.8);
  padding: 0.45rem 1.1rem;
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
  margin-top: 0.6rem;
}

.recent-title {
  font-size: 0.95rem;
  margin: 0 0 0.25rem;
}

.recent-empty {
  font-size: 0.82rem;
  color: #9ca3af;
  margin: 0;
}
</style>
