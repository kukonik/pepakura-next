<!-- pepakura-next/ui-desktop/src/components/stages/ThreeDStage.vue -->
<template>
  <div class="stage-root" :class="{ 'stage-fullscreen': fullscreen }">
    <div class="viewer-shell">
      <header class="viewer-top">
        <div class="viewer-titles">
          <h2 class="viewer-title">3D модель</h2>
          <p class="viewer-subtitle">
            Полноэкранный просмотр с базовыми действиями. Редактирование пока в разработке.
          </p>
        </div>

        <div class="top-actions">
          <button class="icon-btn" @click="toggleLeftPanel">
            ⚙
          </button>
          <button class="icon-btn" @click="toggleRightPanel">
            ⓘ
          </button>
          <button class="icon-btn" @click="onToggleFullscreen">
            {{ fullscreen ? '⤢' : '⛶' }}
          </button>
        </div>
      </header>

      <div class="viewer-main">
        <ThreeDViewerCanvas
          class="three-viewer"
          :model-path="project.threeD.workingPath"
          :mtl-path="project.threeD.mtlPath"
        />

        <button class="fab-import" @click="importModel">
          Импорт 3D
        </button>

        <transition name="slide-left">
          <aside
            v-if="showLeft"
            class="side-panel left"
          >
            <div class="panel-inner">
              <h3 class="side-title">Действия с моделью</h3>

              <button class="primary-btn" @click="importModel">
                Импортировать 3D‑модель (демо/Tauri)
              </button>

              <div class="actions-group">
                <p class="group-title">Геометрия (в разработке)</p>
                <div class="group-buttons">
                  <button class="ghost-btn" :disabled="!hasModel" @click="stub('simplify')">
                    Упростить модель
                  </button>
                  <button class="ghost-btn" :disabled="!hasModel" @click="stub('smooth')">
                    Сгладить
                  </button>
                  <button class="ghost-btn" :disabled="!hasModel" @click="stub('recalc-normals')">
                    Пересчитать нормали
                  </button>
                </div>
              </div>

              <div class="actions-group">
                <p class="group-title">Поза и детали (в разработке)</p>
                <div class="group-buttons">
                  <button class="ghost-btn" :disabled="!hasModel" @click="stub('pose')">
                    Позинг
                  </button>
                  <button class="ghost-btn" :disabled="!hasModel" @click="stub('details')">
                    Упростить/обогатить детали
                  </button>
                </div>
              </div>

              <div class="actions-group">
                <p class="group-title">Текстуры (в разработке)</p>
                <div class="group-buttons">
                  <button class="ghost-btn" :disabled="!hasModel" @click="stub('textures')">
                    Работа с текстурами
                  </button>
                </div>
              </div>
            </div>
          </aside>
        </transition>

        <transition name="slide-right">
          <aside
            v-if="showRight"
            class="side-panel right"
          >
            <div class="panel-inner">
              <h3 class="side-title">Состояние 3D‑этапа</h3>

              <div class="status-block">
                <p class="status-line">
                  Статус:
                  <span
                    class="status-tag"
                    :class="{
                      ok: threeStatus === 'ready',
                      bad: threeStatus === 'error',
                      idle: threeStatus !== 'ready' && threeStatus !== 'error'
                    }"
                  >
                    {{ threeStatusLabel }}
                  </span>
                </p>
                <p class="status-line">
                  Геометрия:
                  <span class="status-value">
                    {{ facesText }}, {{ partsText }}
                  </span>
                </p>
                <p class="status-line">
                  Источник:
                  <span class="status-value">
                    {{ sourceLabel }}
                  </span>
                </p>
              </div>

              <div class="side-section">
                <p class="side-label">Связь с этапами</p>
                <ul class="links-list">
                  <li>
                    TXT:
                    <span class="link-tag">
                      {{ hasTxt ? 'Есть текстовое ТЗ' : 'Без текста' }}
                    </span>
                  </li>
                  <li>
                    2D:
                    <span class="link-tag">
                      {{ hasTwoD ? 'Есть 2D‑референсы' : '2D не использовалось' }}
                    </span>
                  </li>
                  <li>
                    Paper:
                    <span class="link-tag">
                      {{ hasUnfold ? 'Есть развёртка' : 'Развёртка ещё не построена' }}
                    </span>
                  </li>
                </ul>
              </div>

              <div class="side-section">
                <p class="side-label">Дальше → Paper</p>
                <button
                  class="primary-btn"
                  :disabled="!hasModel"
                  @click="goToPaperStub"
                >
                  Перейти к развёртке (заглушка)
                </button>
                <p class="side-note">
                  Позже здесь будет прямой переход к Paper с проверкой актуальности развёртки.
                </p>
              </div>
            </div>
          </aside>
        </transition>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, defineEmits, defineProps, ref } from 'vue'
import { useProjectStore } from '@/stores/project'
import { useTauriModelLoader } from '@/composables/useTauriModelLoader'
import ThreeDViewerCanvas from '@/components/viewer/ThreeDViewerCanvas.vue'

const props = defineProps<{
  fullscreen?: boolean
}>()

const emit = defineEmits<{
  (e: 'toggle-fullscreen'): void
}>()

const project = useProjectStore()
const { showMessage, import3dModel } = useTauriModelLoader()

const threeStatus = computed(() => project.threeD.status)

const threeStatusLabel = computed(() => {
  switch (threeStatus.value) {
    case 'ready':
      return 'модель готова'
    case 'loading':
      return 'загружается'
    case 'error':
      return 'ошибка загрузки'
    default:
      return 'нет модели'
  }
})

const hasModel = computed(() => threeStatus.value === 'ready')

const facesText = computed(() =>
  project.threeD.faces != null ? `${project.threeD.faces} граней` : 'граней: —'
)

const partsText = computed(() =>
  project.threeD.parts != null ? `${project.threeD.parts} частей` : 'частей: —'
)

const hasTxt = computed(() => {
  const txt = project.text?.rawText
  return typeof txt === 'string' && txt.trim().length > 0
})

const hasTwoD = computed(() => project.twod.images.length > 0)

const hasUnfold = computed(
  () => !!project.unfold.layoutPath || !!project.unfold.estimatedSheets
)

const sourceLabel = computed(() => {
  if (hasTxt.value && hasTwoD.value) return 'TXT + 2D'
  if (hasTwoD.value) return '2D'
  if (hasTxt.value) return 'TXT'
  return 'импорт / не указан'
})

const showLeft = ref(false)
const showRight = ref(false)

function toggleLeftPanel() {
  showLeft.value = !showLeft.value
}

function toggleRightPanel() {
  showRight.value = !showRight.value
}

async function importModel() {
  try {
    await import3dModel()
  } catch (e) {
    console.error(e)
    showMessage('Не удалось импортировать 3D‑модель.', 'error')
  }
}

function stub(kind: 'simplify' | 'smooth' | 'recalc-normals' | 'pose' | 'details' | 'textures') {
  switch (kind) {
    case 'simplify':
      showMessage('Упрощение (decimation) 3D‑модели пока в разработке.', 'info')
      break
    case 'smooth':
      showMessage('Сглаживание 3D‑модели пока в разработке.', 'info')
      break
    case 'recalc-normals':
      showMessage('Пересчёт нормалей пока в разработке.', 'info')
      break
    case 'pose':
      showMessage('Позинг 3D‑модели пока в разработке.', 'info')
      break
    case 'details':
      showMessage('Изменение уровня детализации модели пока в разработке.', 'info')
      break
    case 'textures':
      showMessage('Редактор текстур пока в разработке.', 'info')
      break
  }
}

function goToPaperStub() {
  if (!hasModel.value) {
    showMessage('Сначала нужно получить или импортировать 3D‑модель.', 'warning')
    return
  }
  showMessage('Переход к Paper пока в разработке. Откройте Paper‑стейдж вручную.', 'info')
}

function onToggleFullscreen() {
  emit('toggle-fullscreen')
}
</script>

<style scoped>
/* твои же стили, только файл повешен на полный путь в репе */
.stage-root {
  flex: 1;
  min-height: 0;
  display: flex;
  padding: 0.75rem;
  border-radius: 0.9rem;
  border: 1px solid rgba(148, 163, 184, 0.7);
  background: radial-gradient(circle at top, #020617 0, #020617 40%, #000 100%);
}

.stage-fullscreen {
  padding: 0.3rem;
}

.viewer-shell {
  flex: 1;
  display: flex;
  flex-direction: column;
  min-height: 0;
}

.viewer-top {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding-bottom: 0.4rem;
  border-bottom: 1px solid rgba(148, 163, 184, 0.5);
}

.viewer-titles {
  display: flex;
  flex-direction: column;
  gap: 0.1rem;
}

.viewer-title {
  font-size: 0.95rem;
  margin: 0;
}

.viewer-subtitle {
  font-size: 0.78rem;
  color: #9ca3af;
  margin: 0;
}

.top-actions {
  display: flex;
  gap: 0.3rem;
}

.viewer-main {
  position: relative;
  flex: 1;
  min-height: 0;
  margin-top: 0.5rem;
  border-radius: 0.8rem;
  overflow: hidden;
  border: 1px solid rgba(148, 163, 184, 0.7);
}

.three-viewer {
  width: 100%;
  height: 100%;
}

.side-panel {
  position: absolute;
  top: 0;
  bottom: 0;
  width: 260px;
  max-width: 70%;
  background: rgba(15, 23, 42, 0.98);
  border-radius: 0.8rem;
  box-shadow: 0 0 0 1px rgba(148, 163, 184, 0.7);
  z-index: 10;
  display: flex;
  flex-direction: column;
}

.side-panel.left {
  left: 0.4rem;
}

.side-panel.right {
  right: 0.4rem;
}

.panel-inner {
  padding: 0.55rem 0.6rem 0.6rem;
  display: flex;
  flex-direction: column;
  gap: 0.6rem;
  font-size: 0.78rem;
}

.side-title {
  font-size: 0.85rem;
  font-weight: 500;
  margin: 0;
}

.status-block {
  border-radius: 0.6rem;
  border: 1px solid rgba(148, 163, 184, 0.6);
  padding: 0.4rem 0.5rem;
  background: rgba(15, 23, 42, 0.96);
}

.status-line {
  margin: 0.1rem 0;
  display: flex;
  gap: 0.25rem;
  align-items: baseline;
}

.status-tag {
  padding: 0.05rem 0.4rem;
  border-radius: 999px;
  font-size: 0.72rem;
  border: 1px solid rgba(148, 163, 184, 0.6);
}
.status-tag.ok {
  color: #bbf7d0;
  border-color: rgba(22, 163, 74, 0.9);
}
.status-tag.bad {
  color: #fecaca;
  border-color: rgba(239, 68, 68, 0.9);
}
.status-tag.idle {
  color: #9ca3af;
}

.status-value {
  color: #e5e7eb;
}

.side-section {
  font-size: 0.78rem;
}

.side-label {
  margin: 0 0 0.15rem;
  color: #cbd5e1;
}

.side-note {
  font-size: 0.76rem;
  color: #9ca3af;
  margin: 0.25rem 0 0;
}

.links-list {
  margin: 0;
  padding-left: 1.1rem;
  color: #9ca3af;
  font-size: 0.78rem;
}

.link-tag {
  color: #e5e7eb;
}

.primary-btn {
  border-radius: 999px;
  border: 1px solid transparent;
  padding: 0.35rem 0.9rem;
  font-size: 0.8rem;
  cursor: pointer;
  background: linear-gradient(135deg, #3b82f6, #6366f1);
  color: #f9fafb;
}
.primary-btn:disabled {
  opacity: 0.5;
  cursor: default;
}

.ghost-btn {
  border-radius: 999px;
  border: 1px solid rgba(148, 163, 184, 0.8);
  background: transparent;
  color: #e5e7eb;
  font-size: 0.75rem;
  padding: 0.18rem 0.7rem;
  cursor: pointer;
}
.ghost-btn:disabled {
  opacity: 0.4;
  cursor: default;
}

.icon-btn {
  border-radius: 999px;
  border: 1px solid rgba(148, 163, 184, 0.8);
  background: rgba(15, 23, 42, 0.96);
  color: #e5e7eb;
  font-size: 0.8rem;
  padding: 0.12rem 0.5rem;
  cursor: pointer;
}

.fab-import {
  position: absolute;
  bottom: 0.6rem;
  left: 0.8rem;
  border-radius: 999px;
  border: 1px solid transparent;
  padding: 0.3rem 0.8rem;
  font-size: 0.78rem;
  cursor: pointer;
  background: linear-gradient(135deg, #3b82f6, #6366f1);
  color: #f9fafb;
}

.slide-left-enter-active,
.slide-left-leave-active,
.slide-right-enter-active,
.slide-right-leave-active {
  transition: transform 0.18s ease-out, opacity 0.18s ease-out;
}

.slide-left-enter-from,
.slide-left-leave-to {
  transform: translateX(-8px);
  opacity: 0;
}

.slide-right-enter-from,
.slide-right-leave-to {
  transform: translateX(8px);
  opacity: 0;
}
</style>
