<script setup lang='ts'>
import { ref, onMounted, onUnmounted } from 'vue'
import { useAiStore } from '../stores/aiStore'
import { useProjectStore } from '../stores/project.store'

const aiStore = useAiStore()
const projectStore = useProjectStore()

const canvasRef = ref<HTMLCanvasElement | null>(null)
const is3DView = ref(true)
const isLoading = ref(false)
const statusMessage = ref('')

// Рабочие методы
const loadModel = () => {
  isLoading.value = true
  statusMessage.value = 'Загрузка модели...'
  
  // Имитация загрузки
  setTimeout(() => {
    isLoading.value = false
    statusMessage.value = 'Модель загружена'
    // Здесь будет реальная логика загрузки модели
  }, 1500)
}

const generateSeams = () => {
  if (aiStore.isBusy) return
  
  aiStore.setBusy(true)
  statusMessage.value = 'Генерация швов...'
  
  // Имитация генерации
  setTimeout(() => {
    aiStore.setBusy(false)
    statusMessage.value = 'Швы сгенерированы успешно'
    // Здесь будет реальная логика генерации швов
  }, 2000)
}

const exportModel = () => {
  if (aiStore.isBusy) return
  
  aiStore.setBusy(true)
  statusMessage.value = 'Экспорт модели...'
  
  // Имитация экспорта
  setTimeout(() => {
    aiStore.setBusy(false)
    statusMessage.value = 'Модель экспортирована'
    // Здесь будет реальная логика экспорта
  }, 1500)
}

const clearScene = () => {
  statusMessage.value = 'Сцена очищена'
  // Здесь будет реальная логика очистки
}

const toggleView = () => {
  is3DView.value = !is3DView.value
  statusMessage.value = is3DView.value ? 'Переключено на 3D вид' : 'Переключено на 2D вид'
  // Здесь будет реальная логика переключения вида
}

onMounted(() => {
  statusMessage.value = 'Готов к работе'
})

onUnmounted(() => {
  // Очистка при размонтировании
})
</script>

<template>
  <div class="unfold-editor-view">
    <div class="top-bar">
      <h1>Редактор развёртки 3D моделей</h1>
      <div class="status-indicator">
        <span :class="{ 'busy': aiStore.isBusy }">{{ aiStore.isBusy ? 'Обработка...' : statusMessage }}</span>
      </div>
    </div>

    <div class="main-content">
      <div class="left-panel">
        <div class="panel-section">
          <h3>Управление сценой</h3>
          
          <div class="control-group">
            <button @click="loadModel" :disabled="isLoading" class="btn-secondary" type="button">
              {{ isLoading ? 'Загрузка...' : 'Загрузить модель' }}
            </button>
            <button @click="clearScene" class="btn-secondary" type="button">Очистить</button>
          </div>
          
          <div class="control-group">
            <button @click="toggleView" class="btn-secondary" type="button">
              {{ is3DView ? 'Переключить на 2D' : 'Переключить на 3D' }}
            </button>
          </div>
        </div>

        <div class="panel-section">
          <h3>Параметры развёртки</h3>
          
          <div class="control-group">
            <label>Качество развёртки:</label>
            <select>
              <option>Низкое</option>
              <option>Среднее</option>
              <option>Высокое</option>
            </select>
          </div>
          
          <div class="control-group">
            <label>Размер бумаги:</label>
            <select>
              <option>A4</option>
              <option>A3</option>
              <option>Custom</option>
            </select>
          </div>
        </div>

        <div class="panel-section">
          <h3>AI настройки</h3>
          
          <div class="control-group">
            <label>Провайдер:</label>
            <select v-model="aiStore.activeProvider">
              <option value="ollama">Ollama</option>
              <option value="openai">OpenAI</option>
              <option value="custom">Пользовательский</option>
            </select>
          </div>
          
          <div class="control-group">
            <label>Модель:</label>
            <input 
              v-model="aiStore.currentProviderConfig.model" 
              type="text" 
              placeholder="Название модели"
            />
          </div>
        </div>

        <div class="panel-section">
          <h3>Действия</h3>
          
          <div class="control-group">
            <button @click="generateSeams" :disabled="aiStore.isBusy" class="btn-primary" type="button">
              {{ aiStore.isBusy ? 'Обработка...' : 'Сгенерировать швы' }}
            </button>
          </div>
          
          <div class="control-group">
            <button @click="exportModel" :disabled="aiStore.isBusy" class="btn-success" type="button">
              {{ aiStore.isBusy ? 'Экспорт...' : 'Экспорт' }}
            </button>
          </div>
        </div>
      </div>

      <div class="center-area">
        <div class="canvas-container">
          <canvas 
            ref="canvasRef" 
            class="render-canvas"
            :class="{ 'loading': isLoading }"
          ></canvas>
          
          <div v-if="isLoading" class="loading-overlay">
            <div class="spinner"></div>
            <p>Загрузка модели...</p>
          </div>
          
          <div v-else-if="!projectStore.currentProject" class="placeholder">
            <div class="placeholder-content">
              <h3>Загрузите 3D модель</h3>
              <p>Выберите файл OBJ, STL или другой формат</p>
              <button @click="loadModel" class="btn-primary" type="button">Загрузить модель</button>
            </div>
          </div>
          
          <div v-else class="model-info">
            <h3>Модель: {{ projectStore.currentProject.name }}</h3>
            <p>Размер: {{ projectStore.currentProject.fileSize }} байт</p>
            <p>Дата: {{ projectStore.currentProject.lastModified.toLocaleDateString() }}</p>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.unfold-editor-view {
  height: 100vh;
  display: flex;
  flex-direction: column;
}

.top-bar {
  padding: 15px 20px;
  background-color: #f8f9fa;
  border-bottom: 1px solid #dee2e6;
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.top-bar h1 {
  margin: 0;
  font-size: 1.2em;
  color: #333;
}

.status-indicator span {
  padding: 5px 10px;
  border-radius: 4px;
  background-color: #e9ecef;
  font-size: 0.9em;
}

.status-indicator span.busy {
  background-color: #fff3cd;
  color: #856404;
}

.main-content {
  display: flex;
  flex: 1;
  overflow: hidden;
}

.left-panel {
  width: 280px;
  background-color: #f8f9fa;
  border-right: 1px solid #dee2e6;
  padding: 20px;
  overflow-y: auto;
}

.panel-section {
  margin-bottom: 25px;
}

.panel-section h3 {
  margin-top: 0;
  margin-bottom: 15px;
  font-size: 1em;
  color: #333;
}

.control-group {
  margin-bottom: 15px;
}

.control-group label {
  display: block;
  margin-bottom: 5px;
  font-size: 0.9em;
  color: #555;
}

.control-group select,
.control-group input {
  width: 100%;
  padding: 8px 10px;
  border: 1px solid #ddd;
  border-radius: 4px;
  box-sizing: border-box;
}

.control-group button {
  width: 100%;
  padding: 8px;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 0.9em;
}

.btn-primary {
  background-color: #007bff;
  color: white;
}

.btn-primary:hover {
  background-color: #0056b3;
}

.btn-secondary {
  background-color: #6c757d;
  color: white;
}

.btn-secondary:hover {
  background-color: #545b62;
}

.btn-success {
  background-color: #28a745;
  color: white;
}

.btn-success:hover {
  background-color: #1e7e34;
}

.center-area {
  flex: 1;
  display: flex;
  justify-content: center;
  align-items: center;
  overflow: hidden;
}

.canvas-container {
  position: relative;
  width: 100%;
  height: 100%;
  background-color: #333;
  display: flex;
  justify-content: center;
  align-items: center;
}

.render-canvas {
  width: 100%;
  height: 100%;
  display: block;
}

.loading-overlay {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background-color: rgba(0, 0, 0, 0.7);
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  color: white;
}

.spinner {
  border: 4px solid #f3f3f3;
  border-top: 4px solid #007bff;
  border-radius: 50%;
  width: 40px;
  height: 40px;
  animation: spin 1s linear infinite;
  margin-bottom: 15px;
}

@keyframes spin {
  0% { transform: rotate(0deg); }
  100% { transform: rotate(360deg); }
}

.placeholder {
  text-align: center;
  color: #ccc;
}

.placeholder-content {
  padding: 20px;
}

.placeholder-content h3 {
  margin-top: 0;
  margin-bottom: 10px;
}

.placeholder-content p {
  margin: 5px 0;
  color: #999;
}

.model-info {
  position: absolute;
  bottom: 20px;
  left: 20px;
  background-color: rgba(0, 0, 0, 0.7);
  color: white;
  padding: 10px;
  border-radius: 4px;
  max-width: 300px;
}
</style>
