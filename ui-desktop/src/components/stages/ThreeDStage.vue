<template>
  <div class="threed-root">
    <div class="threed-inner">
      <header class="threed-header">
        <div class="header-text">
          <h2 class="header-title">3D Редактор и Просмотрщик</h2>
          <p class="header-subtitle">
            Загрузите модель, затем настройте развёртку на этапе Paper.
          </p>
        </div>
      </header>

      <section class="viewer-section">
        <div
          class="viewer-wrapper"
          ref="viewerWrapper"
          @dragover.prevent="onDragOver"
          @dragenter.prevent="onDragOver"
          @drop.prevent="onDrop"
        >
          <header class="viewer-header">
            <div class="viewer-text">
              <h3 class="viewer-title">Просмотрщик 3D модели</h3>
              <p class="viewer-subtitle">
                {{ hasModel ? 'Используйте мышь для вращения, колёсико для зума.' : 'Загрузите модель, чтобы увидеть её здесь.' }}
              </p>
            </div>
            <div class="viewer-actions">
              <button
                type="button"
                class="viewer-btn"
                @click="resetView"
              >
                Сбросить вид
              </button>
              <button
                type="button"
                class="viewer-btn secondary"
                @click="toggleFullscreen"
              >
                <span v-if="!viewerFullscreen">Полноэкранно</span>
                <span v-else>Выйти</span>
              </button>
            </div>
          </header>

          <div class="viewer-shell">
            <div class="viewer-canvas-container">
              <ThreeDViewerCanvas
                v-if="hasModel"
                :model-path="project.threeD.workingPath"
                :mtl-path="mtlPath"
                :reset-token="resetToken"
              />
            </div>

            <!-- Оверлей импорта поверх viewer -->
            <div class="import-overlay" :class="{ hidden: hasModel }">
              <div class="import-card">
                <div class="import-icon">▢</div>
                <h3 class="import-title">Загрузите модель для начала работы</h3>
                <p class="import-text">
                  Перетащите файлы OBJ + MTL в эту область или воспользуйтесь кнопкой ниже.
                </p>
                <p class="import-text small">
                  Поддерживаемые форматы: OBJ, STL, GLTF, SVG, PNG, JPG.
                </p>
                <button
                  type="button"
                  class="import-btn"
                  @click="triggerFileDialog"
                >
                  Импорт 3D
                </button>
                <input
                  ref="fileInput"
                  type="file"
                  class="file-input"
                  multiple
                  accept=".obj,.mtl,.stl,.gltf,.glb,.svg,.png,.jpg,.jpeg"
                  @change="onFileInputChange"
                />
              </div>
            </div>
          </div>
        </div>
      </section>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue'
import { useProjectStore } from '@/stores/project'
import { useImport3DModel } from '@/composables/useImport3DModel'
import ThreeDViewerCanvas from '@/components/viewer/ThreeDViewerCanvas.vue'

const project = useProjectStore()
const { importFromFiles, reset } = useImport3DModel()

const fileInput = ref<HTMLInputElement | null>(null)
const viewerWrapper = ref<HTMLDivElement | null>(null)

const hasModel = computed(() => !!project.threeD.workingPath)

const viewerFullscreen = ref(false)
const resetToken = ref(0)

const mtlPath = computed(() => {
  if (!project.threeD.workingPath) return null
  return project.threeD.workingPath.replace(/\.obj$/i, '.mtl')
})

function triggerFileDialog() {
  if (fileInput.value) {
    fileInput.value.value = ''
    fileInput.value.click()
  }
}

async function onFileInputChange(e: Event) {
  const input = e.target as HTMLInputElement
  const files = input.files
  if (!files || files.length === 0) return
  await importFromFiles(files)
}

function onDragOver(e: DragEvent) {
  e.preventDefault()
}

async function onDrop(e: DragEvent) {
  e.preventDefault()
  const items = e.dataTransfer?.files
  if (!items || items.length === 0) return
  await importFromFiles(items)
}

function toggleFullscreen() {
  viewerFullscreen.value = !viewerFullscreen.value
  const el = viewerWrapper.value
  if (!el) return
  if (viewerFullscreen.value) {
    if (el.requestFullscreen) el.requestFullscreen()
  } else if (document.fullscreenElement) {
    document.exitFullscreen()
  }
}

function resetView() {
  resetToken.value++
}

function clearModel() {
  reset()
}
</script>

<style scoped>
.threed-root {
  flex: 1;
  display: flex;
  min-height: 0;
}

.threed-inner {
  flex: 1;
  border-radius: 0.9rem;
  border: 1px solid rgba(148, 163, 184, 0.7);
  background: radial-gradient(circle at top, #020617 0, #020617 40%, #000 100%);
  padding: 0.75rem;
  display: flex;
  flex-direction: column;
  gap: 0.6rem;
}

.threed-header {
  padding-bottom: 0.35rem;
  border-bottom: 1px solid rgba(148, 163, 184, 0.5);
}

.header-text {
  display: flex;
  flex-direction: column;
  gap: 0.1rem;
}

.header-title {
  margin: 0;
  font-size: 0.95rem;
  font-weight: 500;
}

.header-subtitle {
  margin: 0;
  font-size: 0.8rem;
  color: #9ca3af;
}

.viewer-section {
  flex: 1;
  min-height: 0;
  margin-top: 0.5rem;
}

.viewer-wrapper {
  height: 100%;
  border-radius: 0.8rem;
  border: 1px solid rgba(148, 163, 184, 0.7);
  background: radial-gradient(circle at top, #020617 0, #020617 40%, #000 100%);
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.viewer-header {
  padding: 0.45rem 0.6rem;
  border-bottom: 1px solid rgba(148, 163, 184, 0.6);
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 0.6rem;
  background: rgba(15, 23, 42, 0.98);
}

.viewer-text {
  display: flex;
  flex-direction: column;
  gap: 0.08rem;
}

.viewer-title {
  margin: 0;
  font-size: 0.88rem;
  font-weight: 500;
}

.viewer-subtitle {
  margin: 0;
  font-size: 0.8rem;
  color: #9ca3af;
}

.viewer-actions {
  display: flex;
  gap: 0.25rem;
}

.viewer-btn {
  border-radius: 999px;
  border: 1px solid rgba(59, 130, 246, 0.9);
  background: linear-gradient(135deg, #3b82f6, #6366f1);
  color: #f9fafb;
  font-size: 0.78rem;
  padding: 0.18rem 0.7rem;
  cursor: pointer;
}

.viewer-btn.secondary {
  border-color: rgba(148, 163, 184, 0.8);
  background: rgba(15, 23, 42, 0.98);
  color: #e5e7eb;
}

.viewer-shell {
  flex: 1;
  min-height: 0;
  position: relative;
  padding: 0.4rem;
  display: flex;
  align-items: stretch;
  justify-content: center;
}

.viewer-canvas-container {
  flex: 1;
  min-height: 0;
}

/* Оверлей импорта */

.import-overlay {
  position: absolute;
  inset: 0.4rem;
  display: flex;
  align-items: center;
  justify-content: center;
  pointer-events: auto;
}

.import-overlay.hidden {
  display: none;
}

.import-card {
  max-width: 420px;
  width: 100%;
  padding: 1rem 1.2rem;
  border-radius: 0.9rem;
  border: 1px solid rgba(148, 163, 184, 0.8);
  background: radial-gradient(circle at top, rgba(15, 23, 42, 0.98) 0, #020617 60%);
  text-align: center;
  box-shadow: 0 20px 40px rgba(15, 23, 42, 0.9);
}

.import-icon {
  font-size: 2.2rem;
  margin-bottom: 0.4rem;
  color: #64748b;
}

.import-title {
  margin: 0 0 0.25rem;
  font-size: 0.9rem;
}

.import-text {
  margin: 0;
  font-size: 0.8rem;
  color: #9ca3af;
}

.import-text.small {
  margin-top: 0.15rem;
}

.import-btn {
  margin-top: 0.6rem;
  border-radius: 0.7rem;
  border: 1px solid rgba(59, 130, 246, 0.9);
  background: linear-gradient(135deg, #3b82f6, #6366f1);
  color: #f9fafb;
  font-size: 0.82rem;
  padding: 0.3rem 0.9rem;
  cursor: pointer;
}

.file-input {
  display: none;
}

.viewer-placeholder {
  margin: auto;
  text-align: center;
  max-width: 320px;
  color: #e5e7eb;
}

.placeholder-title {
  font-size: 0.9rem;
  margin-bottom: 0.2rem;
}

.placeholder-text {
  font-size: 0.8rem;
  color: #9ca3af;
}
</style>
