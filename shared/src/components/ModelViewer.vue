<template>
  <div 
    class="viewer-container" 
    ref="containerRef"
    :class="{ 'drag-over': isDragOver }"
    @dragover.prevent="handleDragOver"
    @dragleave.prevent="handleDragLeave"
    @drop.prevent="handleDrop"
  >
    <div class="viewer-placeholder" v-if="!meshData || !meshData.vertices || meshData.vertices.length === 0">
      <i class="fas fa-cube"></i>
      <h3 style="margin-bottom: 0.5rem; color: #cbd5e1;">Загрузите модель для начала работы</h3>
      <p style="font-size: 0.95rem; max-width: 500px; margin: 0 auto 1.5rem;">
        Перетащите файл сюда или используйте кнопки импорта.<br>
        Поддерживаемые форматы: OBJ, STL, GLTF, SVG, PNG, JPG
      </p>
      <div style="display: flex; gap: 1rem; justify-content: center;">
        <button class="search-btn primary" @click="handleImport3D()">
          <i class="fas fa-cube"></i> Импорт 3D
        </button>
        <button class="search-btn secondary" @click="handleImport2D()">
          <i class="fas fa-image"></i> Импорт 2D
        </button>
      </div>
    </div>
    <div class="viewer-stats" id="viewerStats">
      <span id="modelStats">
        <div v-if="meshData && meshData.vertices && meshData.vertices.length > 0">
          Деталей: <strong>{{ calculateParts() }}</strong>,
          Граней: <strong>{{ calculateFaces() }}</strong>
        </div>
        <div v-else>Нет модели</div>
      </span>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch, onMounted, nextTick } from 'vue';
import { useModelViewer } from '../composables/useModelViewer';
import type { MeshData } from '../types/model';

const props = defineProps<{
  meshData?: MeshData | null;
}>();

// Определяем события
const emit = defineEmits<{
  'request-import3-d': [];
  'request-import2-d': [];
  'file-dropped': [file: File];
}>();

// Используем composable только для рендеринга и инициализации
const { renderMesh, setupViewer, isSceneReady } = useModelViewer();
const containerRef = ref<HTMLElement | null>(null);
const isInitialized = ref(false);
const isDragOver = ref(false);

// Инициализируем Three.js после монтирования компонента
onMounted(async () => {
  if (containerRef.value) {
    setupViewer(containerRef);
    // Ждем инициализации сцены
    await nextTick();
    // Даем время на инициализацию Three.js
    setTimeout(() => {
      isInitialized.value = true;
      // Если данные уже есть, рендерим их
      if (props.meshData && isSceneReady()) {
        renderMesh(props.meshData);
      }
    }, 200);
  } else {
    console.warn('ModelViewer: containerRef.value отсутствует при монтировании');
  }
});

// Обновляем рендеринг при изменении props.meshData (только после инициализации)
watch(() => props.meshData, (newMeshData) => {
  if (newMeshData && isInitialized.value) {
    // Проверяем, что сцена готова
    if (isSceneReady()) {
      renderMesh(newMeshData);
    } else {
      // Если сцена еще не готова, ждем немного и пробуем снова
      setTimeout(() => {
        if (isSceneReady() && newMeshData) {
          renderMesh(newMeshData);
        }
      }, 300);
    }
  }
});

// Обработчики drag&drop
function handleDragOver(event: DragEvent) {
  event.preventDefault();
  event.stopPropagation();
  isDragOver.value = true;
}

function handleDragLeave(event: DragEvent) {
  event.preventDefault();
  event.stopPropagation();
  isDragOver.value = false;
}

async function handleDrop(event: DragEvent) {
  event.preventDefault();
  event.stopPropagation();
  isDragOver.value = false;

  const files = event.dataTransfer?.files;
  if (!files || files.length === 0) {
    console.warn('ModelViewer: Нет файлов в событии drop');
    return;
  }

  const file = files[0];
  console.log('ModelViewer: Файл перетащен:', file.name, file.type);

  // Отправляем событие родителю для обработки
  emit('file-dropped', file);
}

// Вспомогательные функции для вычисления статистики
function calculateParts(): number {
  return props.meshData && props.meshData.vertices && props.meshData.vertices.length > 0 ? 1 : 0;
}

function calculateFaces(): number {
  return props.meshData && props.meshData.triangles ? props.meshData.triangles.length : 0;
}

// Функции импорта - отправляем события родителю
function handleImport3D() {
  console.log('ModelViewer: Запрошена загрузка 3D модели.');
  emit('request-import3-d');
}

function handleImport2D() {
  console.log('ModelViewer: Запрошена загрузка 2D изображения.');
  emit('request-import2-d');
}
</script>

<style scoped>
.viewer-container {
  flex: 1;
  background: rgba(15, 23, 42, 0.9);
  border-radius: 16px;
  border: 2px solid rgba(255, 255, 255, 0.1);
  overflow: hidden;
  display: flex;
  align-items: center;
  justify-content: center;
  position: relative;
  box-shadow: inset 0 0 30px rgba(0, 0, 0, 0.3);
  min-height: 400px;
  width: 100%;
  transition: all 0.3s ease;
}

.viewer-container.drag-over {
  border-color: #60a5fa;
  box-shadow: inset 0 0 30px rgba(96, 165, 250, 0.2), 0 0 20px rgba(96, 165, 250, 0.3);
  background: rgba(15, 23, 42, 0.95);
}

.viewer-placeholder {
  text-align: center;
  color: #94a3b8;
  padding: 2rem;
}

.viewer-placeholder i {
  font-size: 4rem;
  margin-bottom: 1.5rem;
  opacity: 0.3;
}

.viewer-stats {
  position: absolute;
  bottom: 1rem;
  left: 1rem;
  background: rgba(0, 0, 0, 0.5);
  padding: 0.5rem 1rem;
  border-radius: 8px;
  font-size: 0.85rem;
}

.search-btn {
  padding: 0.7rem 1.2rem;
  border-radius: 10px;
  border: none;
  font-weight: 500;
  cursor: pointer;
  display: flex;
  align-items: center;
  gap: 0.5rem;
  transition: all 0.2s;
}

.search-btn.primary {
  background: linear-gradient(135deg, #3b82f6, #6366f1);
  color: white;
}

.search-btn.secondary {
  background: rgba(71, 85, 105, 0.6);
  color: #e2e8f0;
}

.search-btn:hover {
  transform: translateY(-2px);
}
</style>



