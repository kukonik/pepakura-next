<template>
  <div class="viewer-root">
    <header class="viewer-header">
      <div class="viewer-title">
        <span class="title-main">Просмотрщик</span>
        <span class="title-sub">
          {{ viewerStatusText }}
        </span>
      </div>
      <div class="viewer-controls">
        <button
          class="icon-btn"
          :class="{ active: mode === 'auto' }"
          @click="mode = 'auto'"
        >
          Авто
        </button>
        <button
          class="icon-btn"
          :class="{ active: mode === '3d' }"
          @click="mode = '3d'"
        >
          3D
        </button>
        <button
          class="icon-btn"
          :class="{ active: mode === '2d' }"
          @click="mode = '2d'"
        >
          2D
        </button>
        <button
          class="icon-btn"
          :class="{ active: mode === 'log' }"
          @click="mode = 'log'"
        >
          Лог
        </button>

        <button
          class="icon-btn fullscreen-btn"
          @click="toggleFullscreen"
        >
          <span v-if="!viewerFullscreen">Полноэкранно</span>
          <span v-else>Выйти</span>
        </button>
      </div>
    </header>

    <section class="viewer-body">
      <div v-if="currentMode === '3d'" class="pane pane-3d">
        <template v-if="has3D && project.threeD.workingPath">
          <ThreeDViewerCanvas
            :model-path="project.threeD.workingPath"
            :mtl-path="mtlPath"
          />
        </template>
        <div v-else class="placeholder empty">
          <p class="main-line">Нет 3D модели</p>
          <p class="hint">
            Загрузите OBJ/MTL на этапе 3D, чтобы увидеть здесь модель.
          </p>
        </div>
      </div>

      <div v-else-if="currentMode === '2d'" class="pane pane-2d">
        <div v-if="active2D" class="placeholder">
          <p class="main-line">
            2D изображение: {{ active2D.label }}
          </p>
          <p class="sub-line">
            Роль: {{ active2DRoleText }}
          </p>
          <p class="hint">
            Здесь будет предпросмотр чертежа/референса для текущего проекта.
          </p>
        </div>
        <div v-else class="placeholder empty">
          <p class="main-line">Нет активного 2D изображения</p>
          <p class="hint">
            Добавьте и выберите изображение на этапе 2D.
          </p>
        </div>
      </div>

      <div v-else-if="currentMode === 'log'" class="pane pane-log">
        <div class="placeholder">
          <p class="main-line">Журнал операций</p>
          <p class="hint">
            Здесь появятся сообщения от AI, загрузки модели и раскройки (лог-панель).
          </p>
        </div>
      </div>

      <div v-else class="pane pane-auto">
        <template v-if="has3D && project.threeD.workingPath">
          <ThreeDViewerCanvas
            :model-path="project.threeD.workingPath"
            :mtl-path="mtlPath"
          />
        </template>
        <template v-else-if="active2D">
          <div class="placeholder">
            <p class="main-line">
              Авто: показ 2D‑изображения
            </p>
            <p class="sub-line">
              {{ active2D.label }} ({{ active2DRoleText }})
            </p>
            <p class="hint">
              Пока нет 3D — отображается 2D‑референс.
            </p>
          </div>
        </template>
        <div v-else class="placeholder empty">
          <p class="main-line">Просмотрщик ждёт данных</p>
          <p class="hint">
            Добавьте 2D или 3D‑данные, чтобы увидеть предпросмотр.
          </p>
        </div>
      </div>
    </section>
  </div>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue'
import { useProjectStore } from '@/stores/project'
import { useUiStore } from '@/stores/ui'
import ThreeDViewerCanvas from '@/components/viewer/ThreeDViewerCanvas.vue'

type ViewerMode = 'auto' | '3d' | '2d' | 'log'

const project = useProjectStore()
const ui = useUiStore()

const mode = ref<ViewerMode>('auto')
const viewerFullscreen = computed(() => ui.viewerFullscreen)

const has3D = computed(
  () => !!(project.threeD.workingPath || project.threeD.sourcePath)
)

// На данном этапе mtlPath фиксируем под demo-файлы из public/models.
const mtlPath = computed(() => {
  if (!project.threeD.workingPath) return null
  // Если workingPath = 'models/model.obj', то 'models/model.mtl'
  return project.threeD.workingPath.replace(/\.obj$/i, '.mtl')
})

const active2D = computed(() => {
  const id = project.twod.activeImageId
  if (!id) return null
  return project.twod.images.find(img => img.id === id) ?? null
})

const active2DRoleText = computed(() => {
  if (!active2D.value) return '—'
  switch (active2D.value.role) {
    case 'reference':
      return 'референс'
    case 'blueprint':
      return 'чертёж'
    case 'photo':
      return 'фото'
    default:
      return active2D.value.role
  }
})

const currentMode = computed<ViewerMode>(() => {
  if (mode.value !== 'auto') return mode.value
  if (has3D.value) return '3d'
  if (active2D.value) return '2d'
  return 'auto'
})

const viewerStatusText = computed(() => {
  if (currentMode.value === '3d') {
    return has3D.value ? '3D модель готова к просмотру' : '3D не загружен'
  }
  if (currentMode.value === '2d') {
    return active2D.value ? 'Активно 2D изображение' : '2D не выбрано'
  }
  if (currentMode.value === 'log') {
    return 'Лог операций проекта'
  }
  if (has3D.value) return 'Авто • приоритет 3D'
  if (active2D.value) return 'Авто • приоритет 2D'
  return 'Просмотрщик ожидает данные'
})

function toggleFullscreen() {
  ui.toggleViewerFullscreen()
}
</script>

<style scoped>
.viewer-root {
  border-radius: 0.9rem;
  border: 1px solid rgba(148, 163, 184, 0.7);
  background: radial-gradient(circle at top, #020617 0, #020617 40%, #000 100%);
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.viewer-header {
  height: 40px;
  padding: 0 0.6rem;
  border-bottom: 1px solid rgba(148, 163, 184, 0.6);
  display: flex;
  align-items: center;
  justify-content: space-between;
  background: rgba(15, 23, 42, 0.98);
}

.viewer-title {
  display: flex;
  flex-direction: column;
  gap: 0.05rem;
}

.title-main {
  font-size: 0.8rem;
  font-weight: 500;
}
.title-sub {
  font-size: 0.74rem;
  color: #9ca3af;
}

.viewer-controls {
  display: flex;
  gap: 0.25rem;
}

.icon-btn {
  border-radius: 999px;
  border: 1px solid rgba(148, 163, 184, 0.7);
  background: rgba(15, 23, 42, 0.96);
  color: #e5e7eb;
  font-size: 0.72rem;
  padding: 0.08rem 0.5rem;
  cursor: pointer;
}
.icon-btn.active {
  background: linear-gradient(135deg, #3b82f6, #6366f1);
  color: #f9fafb;
  border-color: rgba(191, 219, 254, 0.9);
}

.fullscreen-btn {
  margin-left: 0.3rem;
}

.viewer-body {
  flex: 1;
  min-height: 0;
  display: flex;
}

.pane {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
}

.placeholder {
  text-align: center;
  padding: 0.8rem;
  max-width: 360px;
  color: #e5e7eb;
}

.placeholder.empty {
  color: #9ca3af;
}

.main-line {
  font-size: 0.9rem;
  margin-bottom: 0.2rem;
}

.sub-line {
  font-size: 0.8rem;
  margin-bottom: 0.35rem;
  color: #cbd5e1;
}

.hint {
  font-size: 0.78rem;
  color: #9ca3af;
}
</style>
