<template>
  <div class="model-viewer-container">
    <div class="viewer-placeholder" v-if="!modelPath">
      <i class="fas fa-cube"></i>
      <h3>3D Модель не загружена</h3>
      <p>Используйте инструменты импорта для загрузки модели</p>
    </div>
    <div class="viewer-content" v-else>
      <div class="model-info">
        <p>Загружена модель: {{ modelPath }}</p>
        <div v-if="modelStats" class="stats-grid">
          <div class="stat-item">
            <span class="stat-label">Вершин:</span>
            <span class="stat-value">{{ modelStats.vertices }}</span>
          </div>
          <div class="stat-item">
            <span class="stat-label">Граней:</span>
            <span class="stat-value">{{ modelStats.faces }}</span>
          </div>
          <div class="stat-item">
            <span class="stat-label">Ребер:</span>
            <span class="stat-value">{{ modelStats.edges }}</span>
          </div>
          <div class="stat-item">
            <span class="stat-label">Площадь:</span>
            <span class="stat-value">{{ modelStats.surfaceArea }} кв.ед.</span>
          </div>
          <div class="stat-item">
            <span class="stat-label">Объем:</span>
            <span class="stat-value">{{ modelStats.volume }} куб.ед.</span>
          </div>
        </div>
        <p>{{ loadStatus }}</p>
      </div>
      <!-- Three.js вьювер -->
      <div ref="viewerContainer" class="threejs-viewer"></div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted, watch } from 'vue'
import { useModelViewer } from '../composables/useModelViewer'
import { useModelFileLoader } from '../composables/useModelFileLoader'
import { useModelStats } from '../composables/useModelStats'

const props = defineProps<{
  modelPath?: string | null
}>()

const emit = defineEmits<{
  'update:modelInfo': [info: {
    vertices: number
    faces: number
    edges: number
    surfaceArea: number
    volume: number
    error?: string
  }]
}>()

// Ссылка на контейнер для Three.js
const viewerContainer = ref<HTMLElement | null>(null)

// Используем композабл для Three.js вьювера
const { renderMesh, setupViewer, isSceneReady } = useModelViewer()

// Используем композабл для загрузки моделей
const { isLoading, error, loadModelFromArrayBuffer } = useModelFileLoader()

// Временное хранилище для meshData
const meshData = ref<any>(null)

// Используем композабл для статистики модели
const modelStats = useModelStats(meshData.value)

// Статус загрузки
const loadStatus = computed(() => {
  if (isLoading.value) return 'Загрузка...'
  if (error.value) return `Ошибка: ${error.value}`
  if (!meshData.value) return 'Нет данных'
  return 'Загружено'
})

// Инициализация вьювера при монтировании
onMounted(() => {
  if (viewerContainer.value) {
    setupViewer(viewerContainer)
  }
})

// Загрузка и отображение модели при изменении пути
watch(() => props.modelPath, async (newPath) => {
  if (newPath && isSceneReady()) {
    try {
      // Загружаем файл как ArrayBuffer
      const response = await fetch(newPath)
      const arrayBuffer = await response.arrayBuffer()

      // Определяем расширение файла
      const extension = newPath.split('.').pop() || ''

      // Загружаем модель через новый композабл
      meshData.value = await loadModelFromArrayBuffer(arrayBuffer, extension)

      // Отображаем модель
      renderMesh(meshData.value)

      // Отправляем информацию о модели через emit
      if (modelStats.value) {
        emit('update:modelInfo', {
          vertices: modelStats.value.vertices,
          faces: modelStats.value.faces,
          edges: modelStats.value.edges,
          surfaceArea: modelStats.value.surfaceArea,
          volume: modelStats.value.volume
        })
      }
    } catch (err) {
      console.error('Ошибка загрузки модели:', err)
      // Отправляем информацию об ошибке через emit
      emit('update:modelInfo', {
        vertices: 0,
        faces: 0,
        edges: 0,
        surfaceArea: 0,
        volume: 0,
        error: err instanceof Error ? err.message : 'Неизвестная ошибка'
      })
    }
  }
}, { immediate: true })

// Очистка при размонтировании
onUnmounted(() => {
  // Очистка ресурсов Three.js будет выполнена в useModelViewer
})
</script>

<style scoped>
.model-viewer-container {
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
  background: rgba(15, 23, 42, 0.9);
  border-radius: 16px;
  border: 2px solid rgba(255, 255, 255, 0.1);
  overflow: hidden;
  position: relative;
  box-shadow: inset 0 0 30px rgba(0, 0, 0, 0.3);
}

.viewer-placeholder {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 100%;
  color: #94a3b8;
  text-align: center;
  padding: 2rem;
}

.viewer-placeholder i {
  font-size: 4rem;
  margin-bottom: 1.5rem;
  opacity: 0.3;
}

.viewer-placeholder h3 {
  color: #cbd5e1;
  margin-bottom: 0.5rem;
}

.viewer-content {
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
}

.model-info {
  padding: 1rem;
  background: rgba(30, 41, 59, 0.8);
  border-bottom: 1px solid rgba(255, 255, 255, 0.1);
}

.threejs-viewer {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 2rem;
}

.threejs-viewer p {
  color: #cbd5e1;
  text-align: center;
}

.stats-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(150px, 1fr));
  gap: 1rem;
  margin: 1rem 0;
}

.stat-item {
  display: flex;
  flex-direction: column;
  padding: 0.5rem;
  background: rgba(30, 41, 59, 0.8);
  border-radius: 8px;
  border: 1px solid rgba(255, 255, 255, 0.1);
}

.stat-label {
  font-size: 0.8rem;
  color: #94a3b8;
  margin-bottom: 0.25rem;
}

.stat-value {
  font-size: 1rem;
  font-weight: 600;
  color: #e2e8f0;
}
</style>
