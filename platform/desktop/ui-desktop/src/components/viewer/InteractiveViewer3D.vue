<template>
  <div class="model-viewer-container">
    <!-- Пустое состояние -->
    <div class="viewer-placeholder" v-if="!modelLoaded">
      <i class="fas fa-cube"></i>
      <h3>3D Модель не загружена</h3>
      <p>Используйте инструменты импорта для загрузки модели</p>
      <button class="import-btn" @click="emitImport">
        <i class="fas fa-upload"></i>
        Импорт модели
      </button>
    </div>

    <!-- 3D Viewer -->
    <div class="viewer-content" v-else>
      <!-- Информация о модели -->
      <div class="model-info" v-if="showInfo">
        <div class="info-header">
          <h4>{{ modelName }}</h4>
          <button class="close-btn" @click="showInfo = false">
            <i class="fas fa-times"></i>
          </button>
        </div>
        <div class="stats-grid" v-if="modelStats">
          <div class="stat-item">
            <i class="fas fa-vector-square"></i>
            <span class="stat-label">Вершин:</span>
            <span class="stat-value">{{ modelStats.vertices }}</span>
          </div>
          <div class="stat-item">
            <i class="fas fa-shapes"></i>
            <span class="stat-label">Граней:</span>
            <span class="stat-value">{{ modelStats.faces }}</span>
          </div>
          <div class="stat-item">
            <i class="fas fa-random"></i>
            <span class="stat-label">Рёбер:</span>
            <span class="stat-value">{{ modelStats.edges }}</span>
          </div>
        </div>
      </div>

      <!-- Toolbar -->
      <div class="viewer-toolbar">
        <div class="toolbar-group">
          <button
            :class="{ active: viewMode === 'orbit' }"
            @click="setViewMode('orbit')"
            title="Вращение"
          >
            <i class="fas fa-orbit"></i>
          </button>
          <button
            :class="{ active: viewMode === 'pan' }"
            @click="setViewMode('pan')"
            title="Перемещение"
          >
            <i class="fas fa-hand-paper"></i>
          </button>
          <button
            :class="{ active: viewMode === 'zoom' }"
            @click="setViewMode('zoom')"
            title="Приближение"
          >
            <i class="fas fa-search-plus"></i>
          </button>
        </div>

        <div class="toolbar-group">
          <button @click="fitToMesh" title="Показать всё">
            <i class="fas fa-expand"></i>
          </button>
          <button @click="resetView" title="Сбросить вид">
            <i class="fas fa-undo"></i>
          </button>
          <button @click="toggleAutoRotate" :class="{ active: autoRotate }" title="Авто-вращение">
            <i class="fas fa-sync"></i>
          </button>
        </div>

        <div class="toolbar-group">
          <button
            :class="{ active: link3d2d }"
            @click="toggleLink3d2d"
            title="Привязка 3D ↔ 2D"
          >
            <i class="fas fa-link"></i>
          </button>
          <button @click="toggleInfo" title="Информация">
            <i class="fas fa-info-circle"></i>
          </button>
        </div>
      </div>

      <!-- Three.js контейнер -->
      <div
        ref="viewerContainer"
        class="threejs-viewer"
        :class="{ 'link-active': link3d2d, 'face-selected': selectedFaceIndex !== null }"
      ></div>

      <!-- Индикатор выделенной грани -->
      <div class="face-indicator" v-if="selectedFaceIndex !== null">
        <i class="fas fa-cube"></i>
        <span>Грань #{{ selectedFaceIndex + 1 }}</span>
        <button class="clear-btn" @click="deselectFace">
          <i class="fas fa-times"></i>
        </button>
      </div>

      <!-- Подсказка -->
      <div class="viewer-hint">
        <span v-if="!link3d2d">🖱️ ЛКМ: вращение • ПКМ: перемещение • Колесо: зум</span>
        <span v-else>🔗 Привязка 3D ↔ 2D активна</span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted, watch } from 'vue'
import { useInteractiveViewer3D, type Mesh3DData } from '@/composables/useInteractiveViewer3D'
import { use3d2dLink } from '@/composables/use3d2dLink'

interface ModelStats {
  vertices: number
  faces: number
  edges: number
}

interface Props {
  modelPath?: string
  modelData?: Mesh3DData
  showInfo?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  modelPath: undefined,
  modelData: undefined,
  showInfo: true,
})

const emit = defineEmits<{
  import: []
  'face-select': [faceIndex: number]
}>()

// Состояние
const modelLoaded = ref(false)
const modelName = ref('')
const modelStats = ref<ModelStats | null>(null)
const viewMode = ref<'orbit' | 'pan' | 'zoom'>('orbit')
const autoRotate = ref(false)
const link3d2d = ref(true)
const selectedFaceIndex = ref<number | null>(null)

// Контейнер
const viewerContainer = ref<HTMLElement | null>(null)

// 3D Viewer composable
const {
  containerRef,
  isReady,
  initScene,
  loadMesh,
  selectFace,
  deselectFace,
  fitCameraToMesh,
  getFaceInfo,
  setAutoRotate,
  setBackgroundColor,
  getCameraState,
  setCameraState,
  cleanup,
} = useInteractiveViewer3D({
  backgroundColor: 0x0b1120,
  highlightColor: 0x4a9eff,
  enableGrid: true,
  enableAxes: false,
  autoRotate: false,
})

// 3D ↔ 2D Link composable
const {
  highlightFaceIn2D,
  highlightFaceIn3D,
  clearHighlight,
  syncCamera3dTo2d,
} = use3d2dLink()

// Вычисляемые свойства
const internalShowInfo = ref(props.showInfo)

// Методы
const emitImport = () => emit('import')

const setViewMode = (mode: 'orbit' | 'pan' | 'zoom') => {
  viewMode.value = mode
  // TODO: Применить режим к controls
}

const toggleInfo = () => {
  internalShowInfo.value = !internalShowInfo.value
}

const resetView = () => {
  if (isReady.value) {
    setCameraState({
      position: [3, 3, 3],
      target: [0, 0, 0],
    })
  }
}

const toggleAutoRotate = () => {
  autoRotate.value = !autoRotate.value
  setAutoRotate(autoRotate.value)
}

const toggleLink3d2d = () => {
  link3d2d.value = !link3d2d.value
}

const handleFaceSelected = (faceIndex: number) => {
  selectedFaceIndex.value = faceIndex
  emit('face-select', faceIndex)

  if (link3d2d.value) {
    highlightFaceIn2D(faceIndex)
  }
}

const deselectFace = () => {
  selectedFaceIndex.value = null
  deselectFace()
  if (link3d2d.value) {
    clearHighlight()
  }
}

// Обработчик событий от 3D↔2D link
const handle2dHighlight = (event: Event) => {
  const customEvent = event as CustomEvent<{ faceIndex: number | null, highlighted: boolean }>
  if (customEvent.detail.faceIndex !== null && customEvent.detail.highlighted) {
    selectFace(customEvent.detail.faceIndex)
  }
}

// Загрузка модели
const loadModel = (data: Mesh3DData, name?: string) => {
  if (!isReady.value) return

  loadMesh(data)
  modelLoaded.value = true
  modelName.value = name || '3D Модель'

  // Вычисляем статистику
  modelStats.value = {
    vertices: data.vertices.length,
    faces: data.faces.length,
    edges: Math.floor((data.faces.length * 3) / 2),
  }
}

// Инициализация
onMounted(() => {
  if (viewerContainer.value) {
    containerRef.value = viewerContainer.value
    initScene()

    // Подписка на события
    window.addEventListener('highlight-face-3d', handle2dHighlight)

    // Загрузка модели из props
    if (props.modelData) {
      loadModel(props.modelData, props.modelPath)
    }
  }
})

// Очистка
onUnmounted(() => {
  cleanup()
  window.removeEventListener('highlight-face-3d', handle2dHighlight)
})

// Watch для modelData
watch(() => props.modelData, (newData) => {
  if (newData && isReady.value) {
    loadModel(newData, props.modelPath)
  }
})

// Публичные методы
defineExpose({
  loadModel,
  selectFace,
  deselectFace,
  fitCameraToMesh,
  getFaceInfo,
  getCameraState,
  setCameraState,
})
</script>

<style scoped>
.model-viewer-container {
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
  background: var(--bg-secondary, #1a1a2e);
  border-radius: 8px;
  overflow: hidden;
}

.viewer-placeholder {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 100%;
  color: var(--text-secondary, #888);
  text-align: center;
  padding: 40px;
}

.viewer-placeholder i {
  font-size: 64px;
  margin-bottom: 20px;
  opacity: 0.5;
}

.viewer-placeholder h3 {
  margin: 0 0 10px;
  font-size: 18px;
  color: var(--text-primary, #fff);
}

.viewer-placeholder p {
  margin: 0 0 20px;
  font-size: 14px;
}

.import-btn {
  display: inline-flex;
  align-items: center;
  gap: 8px;
  padding: 12px 24px;
  background: var(--accent-color, #4a9eff);
  color: white;
  border: none;
  border-radius: 6px;
  font-size: 14px;
  cursor: pointer;
  transition: all 0.2s;
}

.import-btn:hover {
  background: var(--accent-hover, #3a8eef);
  transform: translateY(-2px);
}

.viewer-content {
  position: relative;
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
}

.model-info {
  position: absolute;
  top: 10px;
  right: 10px;
  background: rgba(0, 0, 0, 0.8);
  border-radius: 8px;
  padding: 16px;
  min-width: 200px;
  z-index: 10;
  backdrop-filter: blur(10px);
}

.info-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 12px;
}

.info-header h4 {
  margin: 0;
  font-size: 14px;
  color: white;
}

.close-btn {
  background: transparent;
  border: none;
  color: white;
  cursor: pointer;
  padding: 4px;
  opacity: 0.7;
}

.close-btn:hover {
  opacity: 1;
}

.stats-grid {
  display: grid;
  gap: 8px;
}

.stat-item {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 13px;
  color: #ccc;
}

.stat-item i {
  width: 16px;
  text-align: center;
}

.stat-value {
  font-weight: 600;
  color: white;
  margin-left: auto;
}

.viewer-toolbar {
  position: absolute;
  top: 10px;
  left: 10px;
  display: flex;
  gap: 8px;
  z-index: 10;
}

.toolbar-group {
  display: flex;
  gap: 4px;
  background: rgba(0, 0, 0, 0.6);
  padding: 4px;
  border-radius: 6px;
  backdrop-filter: blur(10px);
}

.toolbar-group button {
  width: 36px;
  height: 36px;
  border: none;
  background: rgba(255, 255, 255, 0.1);
  color: white;
  border-radius: 4px;
  cursor: pointer;
  transition: all 0.2s;
}

.toolbar-group button:hover {
  background: rgba(255, 255, 255, 0.2);
}

.toolbar-group button.active {
  background: var(--accent-color, #4a9eff);
}

.threejs-viewer {
  flex: 1;
  width: 100%;
  height: 100%;
}

.threejs-viewer.link-active {
  cursor: crosshair;
}

.threejs-viewer.face-selected {
  cursor: pointer;
}

.face-indicator {
  position: absolute;
  bottom: 50px;
  left: 50%;
  transform: translateX(-50%);
  display: flex;
  align-items: center;
  gap: 8px;
  background: rgba(0, 0, 0, 0.8);
  color: white;
  padding: 8px 16px;
  border-radius: 20px;
  font-size: 14px;
  backdrop-filter: blur(10px);
  z-index: 10;
}

.face-indicator i {
  color: var(--accent-color, #4a9eff);
}

.clear-btn {
  background: transparent;
  border: none;
  color: white;
  cursor: pointer;
  padding: 4px 8px;
  opacity: 0.7;
  margin-left: 8px;
}

.clear-btn:hover {
  opacity: 1;
}

.viewer-hint {
  position: absolute;
  bottom: 10px;
  left: 50%;
  transform: translateX(-50%);
  font-size: 12px;
  color: rgba(255, 255, 255, 0.6);
  background: rgba(0, 0, 0, 0.6);
  padding: 6px 12px;
  border-radius: 12px;
  backdrop-filter: blur(10px);
}
</style>
