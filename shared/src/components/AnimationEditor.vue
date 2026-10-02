<template>
  <div class="animation-editor">
    <!-- Панель инструментов анимаций -->
    <div class="animation-toolbar">
      <div class="toolbar-group">
        <button 
          class="tool-btn" 
          :class="{ active: isPlaying }" 
          @click="togglePlayback"
          title="Воспроизвести/Пауза"
        >
          <i :class="isPlaying ? 'fas fa-pause' : 'fas fa-play'"></i>
        </button>
        <button 
          class="tool-btn" 
          @click="stopAnimation"
          title="Остановить"
        >
          <i class="fas fa-stop"></i>
        </button>
        <button 
          class="tool-btn" 
          @click="addKeyframe"
          title="Добавить ключевой кадр"
        >
          <i class="fas fa-plus-circle"></i>
        </button>
      </div>
      
      <div class="toolbar-group">
        <button 
          class="tool-btn" 
          :class="{ active: isRecording }" 
          @click="toggleRecording"
          title="Запись"
        >
          <i class="fas fa-record-vinyl"></i>
        </button>
        <button 
          class="tool-btn" 
          @click="addTransition"
          title="Добавить переход"
        >
          <i class="fas fa-exchange-alt"></i>
        </button>
      </div>
      
      <div class="toolbar-group">
        <div class="speed-control">
          <label for="speedSlider">Скорость:</label>
          <input 
            id="speedSlider"
            type="range" 
            min="0.1" 
            max="3" 
            step="0.1" 
            v-model="playbackSpeed"
            @change="updatePlaybackSpeed"
          />
          <span>{{ playbackSpeed }}x</span>
        </div>
      </div>
    </div>
    
    <!-- Основная область редактора -->
    <div class="editor-main">
      <!-- Панель скелета и костей -->
      <div class="skeleton-panel">
        <h3>Скелет</h3>
        <div class="skeleton-tree">
          <div 
            v-for="bone in skeletonBones" 
            :key="bone.id"
            class="bone-item"
            :class="{ selected: selectedBone === bone.id }"
            @click="selectBone(bone.id)"
          >
            <i class="fas fa-bone"></i>
            <span>{{ bone.name }}</span>
          </div>
        </div>
        
        <!-- Панель свойств кости -->
        <div v-if="selectedBone" class="bone-properties">
          <h4>Свойства кости: {{ getBoneName(selectedBone) }}</h4>
          <div class="property-group">
            <label>Позиция X:</label>
            <input 
              type="number" 
              v-model="boneProperties.position.x" 
              @change="updateBoneProperty('position', 'x')"
              step="0.1"
            />
          </div>
          <div class="property-group">
            <label>Позиция Y:</label>
            <input 
              type="number" 
              v-model="boneProperties.position.y" 
              @change="updateBoneProperty('position', 'y')"
              step="0.1"
            />
          </div>
          <div class="property-group">
            <label>Позиция Z:</label>
            <input 
              type="number" 
              v-model="boneProperties.position.z" 
              @change="updateBoneProperty('position', 'z')"
              step="0.1"
            />
          </div>
          <div class="property-group">
            <label>Вращение X:</label>
            <input 
              type="number" 
              v-model="boneProperties.rotation.x" 
              @change="updateBoneProperty('rotation', 'x')"
              step="1"
            />
          </div>
          <div class="property-group">
            <label>Вращение Y:</label>
            <input 
              type="number" 
              v-model="boneProperties.rotation.y" 
              @change="updateBoneProperty('rotation', 'y')"
              step="1"
            />
          </div>
          <div class="property-group">
            <label>Вращение Z:</label>
            <input 
              type="number" 
              v-model="boneProperties.rotation.z" 
              @change="updateBoneProperty('rotation', 'z')"
              step="1"
            />
          </div>
        </div>
      </div>
      
      <!-- Таймлайн анимации -->
      <div class="timeline-panel">
        <div class="timeline-header">
          <div class="time-display">
            Текущее время: {{ currentTime.toFixed(2) }} сек
          </div>
          <div class="duration-display">
            Длительность: {{ animationDuration.toFixed(2) }} сек
          </div>
        </div>
        
        <!-- Визуализация таймлайна -->
        <div class="timeline-viewer" ref="timelineViewer">
          <!-- Линейка времени -->
          <div class="time-ruler">
            <div 
              v-for="time in timeMarkers" 
              :key="time" 
              class="time-marker"
              :style="{ left: `${(time / animationDuration) * 100}%` }"
            >
              {{ time }}s
            </div>
          </div>
          
          <!-- Дорожки анимации -->
          <div class="tracks-container">
            <div 
              v-for="track in animationTracks" 
              :key="track.id"
              class="track"
            >
              <div class="track-header">
                {{ track.name }}
              </div>
              <div class="track-content">
                <!-- Ключевые кадры на дорожке -->
                <div 
                  v-for="keyframe in track.keyframes"
                  :key="keyframe.id"
                  class="keyframe"
                  :class="{ selected: selectedKeyframe === keyframe.id }"
                  :style="{ left: `${(keyframe.time / animationDuration) * 100}%` }"
                  @click="selectKeyframe(keyframe.id)"
                >
                  <i class="fas fa-diamond"></i>
                </div>
              </div>
            </div>
          </div>
          
          <!-- Указатель текущего времени -->
          <div 
            class="time-indicator"
            :style="{ left: `${(currentTime / animationDuration) * 100}%` }"
          >
            <div class="time-indicator-line"></div>
            <div class="time-indicator-handle"></div>
          </div>
        </div>
        
        <!-- Контролы таймлайна -->
        <div class="timeline-controls">
          <input 
            type="range" 
            min="0" 
            :max="animationDuration" 
            step="0.01" 
            v-model="currentTime"
            @input="updateCurrentTime"
            class="time-slider"
          />
          <div class="zoom-controls">
            <button @click="zoomIn" title="Увеличить"><i class="fas fa-search-plus"></i></button>
            <button @click="zoomOut" title="Уменьшить"><i class="fas fa-search-minus"></i></button>
          </div>
        </div>
      </div>
      
      <!-- Панель предпросмотра -->
      <div class="preview-panel">
        <h3>Предпросмотр</h3>
        <div class="preview-container">
          <!-- Здесь будет отображаться 3D модель с анимацией -->
          <div class="model-placeholder">
            <i class="fas fa-cube"></i>
            <p>3D Модель с анимацией</p>
          </div>
        </div>
        
        <!-- Статистика анимации -->
        <div class="animation-stats">
          <div class="stat-item">
            <span class="stat-label">Ключевых кадров:</span>
            <span class="stat-value">{{ totalKeyframes }}</span>
          </div>
          <div class="stat-item">
            <span class="stat-label">Костей:</span>
            <span class="stat-value">{{ skeletonBones.length }}</span>
          </div>
          <div class="stat-item">
            <span class="stat-label">Дорожек:</span>
            <span class="stat-value">{{ animationTracks.length }}</span>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'

// Состояние редактора
const isPlaying = ref(false)
const isRecording = ref(false)
const playbackSpeed = ref(1.0)
const currentTime = ref(0)
const animationDuration = ref(10) // По умолчанию 10 секунд
const selectedBone = ref<string | null>(null)
const selectedKeyframe = ref<string | null>(null)

// Данные скелета
const skeletonBones = ref([
  { id: 'root', name: 'Корень', parentId: null },
  { id: 'spine', name: 'Позвоночник', parentId: 'root' },
  { id: 'neck', name: 'Шея', parentId: 'spine' },
  { id: 'head', name: 'Голова', parentId: 'neck' },
  { id: 'leftArm', name: 'Левая рука', parentId: 'spine' },
  { id: 'rightArm', name: 'Правая рука', parentId: 'spine' },
  { id: 'leftLeg', name: 'Левая нога', parentId: 'root' },
  { id: 'rightLeg', name: 'Правая нога', parentId: 'root' }
])

// Свойства выбранной кости
const boneProperties = ref({
  position: { x: 0, y: 0, z: 0 },
  rotation: { x: 0, y: 0, z: 0 }
})

// Дорожки анимации
const animationTracks = ref([
  { 
    id: 'position_x', 
    name: 'Позиция X', 
    boneId: 'root',
    keyframes: [
      { id: 'k1', time: 0, value: 0 },
      { id: 'k2', time: 2, value: 5 },
      { id: 'k3', time: 4, value: 0 }
    ]
  },
  { 
    id: 'rotation_y', 
    name: 'Вращение Y', 
    boneId: 'spine',
    keyframes: [
      { id: 'k4', time: 0, value: 0 },
      { id: 'k5', time: 3, value: 90 },
      { id: 'k6', time: 6, value: 0 }
    ]
  }
])

// Вычисляемые свойства
const timeMarkers = computed(() => {
  const markers = []
  for (let i = 0; i <= animationDuration.value; i += 1) {
    markers.push(i)
  }
  return markers
})

const totalKeyframes = computed(() => {
  return animationTracks.value.reduce((total, track) => total + track.keyframes.length, 0)
})

// Методы управления воспроизведением
function togglePlayback() {
  isPlaying.value = !isPlaying.value
  if (isPlaying.value) {
    startPlayback()
  } else {
    pausePlayback()
  }
}

function startPlayback() {
  // Здесь будет логика воспроизведения анимации
  console.log('Начало воспроизведения анимации')
}

function pausePlayback() {
  // Здесь будет логика паузы анимации
  console.log('Пауза анимации')
}

function stopAnimation() {
  isPlaying.value = false
  currentTime.value = 0
  // Здесь будет логика остановки анимации
  console.log('Остановка анимации')
}

function updatePlaybackSpeed() {
  // Здесь будет логика изменения скорости воспроизведения
  console.log('Изменение скорости воспроизведения:', playbackSpeed.value)
}

// Методы управления ключевыми кадрами
function addKeyframe() {
  // Здесь будет логика добавления ключевого кадра
  console.log('Добавление ключевого кадра')
}

function selectKeyframe(keyframeId: string) {
  selectedKeyframe.value = keyframeId
  // Здесь будет логика выбора ключевого кадра
  console.log('Выбран ключевой кадр:', keyframeId)
}

// Методы управления костями
function selectBone(boneId: string) {
  selectedBone.value = boneId
  // Здесь будет логика выбора кости
  console.log('Выбрана кость:', boneId)
}

function getBoneName(boneId: string) {
  const bone = skeletonBones.value.find(b => b.id === boneId)
  return bone ? bone.name : boneId
}

function updateBoneProperty(property: string, axis: string) {
  // Здесь будет логика обновления свойств кости
  console.log(`Обновление свойства ${property}.${axis}:`, boneProperties.value[property][axis])
}

// Методы управления переходами
function addTransition() {
  // Здесь будет логика добавления перехода
  console.log('Добавление перехода')
}

// Методы управления записью
function toggleRecording() {
  isRecording.value = !isRecording.value
  if (isRecording.value) {
    startRecording()
  } else {
    stopRecording()
  }
}

function startRecording() {
  // Здесь будет логика начала записи
  console.log('Начало записи анимации')
}

function stopRecording() {
  // Здесь будет логика остановки записи
  console.log('Остановка записи анимации')
}

// Методы управления таймлайном
function updateCurrentTime() {
  // Здесь будет логика обновления текущего времени
  console.log('Текущее время:', currentTime.value)
}

function zoomIn() {
  // Здесь будет логика увеличения масштаба таймлайна
  console.log('Увеличение масштаба таймлайна')
}

function zoomOut() {
  // Здесь будет логика уменьшения масштаба таймлайна
  console.log('Уменьшение масштаба таймлайна')
}

// Жизненный цикл компонента
onMounted(() => {
  // Инициализация редактора анимаций
  console.log('Инициализация редактора анимаций')
})

onUnmounted(() => {
  // Очистка ресурсов
  console.log('Очистка ресурсов редактора анимаций')
})
</script>

<style scoped>
.animation-editor {
  display: flex;
  flex-direction: column;
  height: 100%;
  background: #0f172a;
  color: #e2e8f0;
  font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
}

.animation-toolbar {
  display: flex;
  align-items: center;
  padding: 0.8rem 1rem;
  background: rgba(15, 23, 42, 0.95);
  border-bottom: 1px solid rgba(255, 255, 255, 0.1);
  gap: 1.5rem;
}

.toolbar-group {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.tool-btn {
  width: 36px;
  height: 36px;
  border-radius: 8px;
  border: 1px solid rgba(255, 255, 255, 0.1);
  background: rgba(30, 41, 59, 0.8);
  color: #cbd5e1;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.2s;
}

.tool-btn:hover {
  background: rgba(56, 70, 95, 0.9);
  color: #fff;
}

.tool-btn.active {
  background: linear-gradient(135deg, #3b82f6, #6366f1);
  color: white;
  border: none;
}

.speed-control {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  font-size: 0.9rem;
}

.speed-control input {
  width: 100px;
}

.editor-main {
  display: flex;
  flex: 1;
  overflow: hidden;
}

.skeleton-panel {
  width: 250px;
  background: rgba(15, 23, 42, 0.8);
  border-right: 1px solid rgba(255, 255, 255, 0.1);
  padding: 1rem;
  overflow-y: auto;
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
}

.skeleton-panel h3 {
  margin: 0 0 1rem 0;
  font-size: 1.1rem;
  color: #e2e8f0;
}

.skeleton-tree {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.bone-item {
  padding: 0.6rem 0.8rem;
  border-radius: 6px;
  background: rgba(30, 41, 59, 0.6);
  border: 1px solid rgba(255, 255, 255, 0.08);
  display: flex;
  align-items: center;
  gap: 0.7rem;
  cursor: pointer;
  transition: all 0.2s;
}

.bone-item:hover {
  background: rgba(56, 70, 95, 0.8);
}

.bone-item.selected {
  background: linear-gradient(135deg, #3b82f6, #6366f1);
  color: white;
  border: none;
}

.bone-properties {
  background: rgba(30, 41, 59, 0.6);
  border-radius: 8px;
  padding: 1rem;
  border: 1px solid rgba(255, 255, 255, 0.08);
}

.bone-properties h4 {
  margin: 0 0 1rem 0;
  font-size: 1rem;
  color: #e2e8f0;
}

.property-group {
  margin-bottom: 0.8rem;
}

.property-group label {
  display: block;
  margin-bottom: 0.3rem;
  font-size: 0.85rem;
  color: #94a3b8;
}

.property-group input {
  width: 100%;
  padding: 0.5rem;
  background: rgba(15, 23, 42, 0.8);
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 6px;
  color: #e2e8f0;
  font-size: 0.9rem;
}

.timeline-panel {
  flex: 1;
  display: flex;
  flex-direction: column;
  background: rgba(15, 23, 42, 0.9);
  border-right: 1px solid rgba(255, 255, 255, 0.1);
}

.timeline-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0.8rem 1rem;
  background: rgba(30, 41, 59, 0.8);
  border-bottom: 1px solid rgba(255, 255, 255, 0.1);
  font-size: 0.9rem;
}

.time-display, .duration-display {
  color: #94a3b8;
}

.timeline-viewer {
  flex: 1;
  position: relative;
  overflow: auto;
  background: rgba(15, 23, 42, 0.7);
}

.time-ruler {
  height: 30px;
  background: rgba(30, 41, 59, 0.8);
  border-bottom: 1px solid rgba(255, 255, 255, 0.1);
  position: relative;
}

.time-marker {
  position: absolute;
  top: 5px;
  transform: translateX(-50%);
  font-size: 0.7rem;
  color: #94a3b8;
  white-space: nowrap;
}

.tracks-container {
  padding: 1rem;
}

.track {
  margin-bottom: 1rem;
}

.track-header {
  padding: 0.5rem;
  background: rgba(30, 41, 59, 0.8);
  border-radius: 4px 4px 0 0;
  font-size: 0.9rem;
  color: #e2e8f0;
}

.track-content {
  height: 60px;
  background: rgba(30, 41, 59, 0.6);
  border-radius: 0 0 4px 4px;
  position: relative;
  border: 1px solid rgba(255, 255, 255, 0.08);
}

.keyframe {
  position: absolute;
  top: 50%;
  transform: translate(-50%, -50%);
  width: 16px;
  height: 16px;
  background: #3b82f6;
  border-radius: 50%;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  color: white;
  font-size: 0.6rem;
  border: 2px solid #60a5fa;
}

.keyframe.selected {
  background: #f59e0b;
  border-color: #fbbf24;
}

.time-indicator {
  position: absolute;
  top: 0;
  height: 100%;
  pointer-events: none;
}

.time-indicator-line {
  position: absolute;
  top: 0;
  bottom: 0;
  width: 2px;
  background: #f59e0b;
}

.time-indicator-handle {
  position: absolute;
  top: 0;
  left: -8px;
  width: 16px;
  height: 16px;
  background: #f59e0b;
  border-radius: 50%;
  border: 2px solid #fbbf24;
  cursor: pointer;
  pointer-events: auto;
}

.timeline-controls {
  display: flex;
  align-items: center;
  padding: 0.8rem 1rem;
  background: rgba(30, 41, 59, 0.8);
  border-top: 1px solid rgba(255, 255, 255, 0.1);
  gap: 1rem;
}

.time-slider {
  flex: 1;
  height: 6px;
  background: rgba(255, 255, 255, 0.1);
  border-radius: 3px;
  outline: none;
  -webkit-appearance: none;
}

.time-slider::-webkit-slider-thumb {
  -webkit-appearance: none;
  width: 16px;
  height: 16px;
  background: #3b82f6;
  border-radius: 50%;
  cursor: pointer;
}

.zoom-controls {
  display: flex;
  gap: 0.5rem;
}

.zoom-controls button {
  width: 32px;
  height: 32px;
  border-radius: 6px;
  border: 1px solid rgba(255, 255, 255, 0.1);
  background: rgba(30, 41, 59, 0.8);
  color: #cbd5e1;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.2s;
}

.zoom-controls button:hover {
  background: rgba(56, 70, 95, 0.9);
  color: #fff;
}

.preview-panel {
  width: 300px;
  background: rgba(15, 23, 42, 0.8);
  padding: 1rem;
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
}

.preview-panel h3 {
  margin: 0 0 1rem 0;
  font-size: 1.1rem;
  color: #e2e8f0;
}

.preview-container {
  flex: 1;
  background: rgba(30, 41, 59, 0.6);
  border-radius: 8px;
  border: 1px solid rgba(255, 255, 255, 0.08);
  display: flex;
  align-items: center;
  justify-content: center;
}

.model-placeholder {
  text-align: center;
  color: #94a3b8;
}

.model-placeholder i {
  font-size: 3rem;
  margin-bottom: 1rem;
  opacity: 0.5;
}

.animation-stats {
  background: rgba(30, 41, 59, 0.6);
  border-radius: 8px;
  padding: 1rem;
  border: 1px solid rgba(255, 255, 255, 0.08);
}

.stat-item {
  display: flex;
  justify-content: space-between;
  margin-bottom: 0.5rem;
  font-size: 0.9rem;
}

.stat-label {
  color: #94a3b8;
}

.stat-value {
  color: #e2e8f0;
  font-weight: 500;
}
</style>