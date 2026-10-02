param()

Write-Host "=== RESTORE UI-DESKTOP: START ===" -ForegroundColor Cyan

$root = "D:\Dev\pepakura-next"
$uiDesktopPath = Join-Path $root "packages\ui-desktop"

if (-not (Test-Path $uiDesktopPath)) {
  Write-Host "Пакет ui-desktop не найден: $uiDesktopPath" -ForegroundColor Red
  exit 1
}

# 1. Создаем aiStore.ts
Write-Host "`n[1/3] Создание/обновление aiStore.ts..." -ForegroundColor Cyan

$aiStoreContent = @"
import { defineStore } from 'pinia'
import type { AiBackendConfig } from '../../shared/src/ai/AiBackendConfig'

export interface AiProvider {
  id: 'ollama' | 'openai' | 'custom'
  name: string
  description: string
}

export interface AiState {
  activeProvider: 'ollama' | 'openai' | 'custom'
  providers: Record<'ollama' | 'openai' | 'custom', AiBackendConfig>
  isBusy: boolean
  lastError: string | null
  requestHistory: Array<{
    timestamp: Date
    provider: string
    status: 'success' | 'error'
    message: string
  }>
}

export const useAiStore = defineStore('ai', {
  state: (): AiState => ({
    activeProvider: 'ollama',
    providers: {
      ollama: {
        endpoint: 'http://localhost:11434',
        model: 'llama3',
        temperature: 0.7,
        maxTokens: 2048,
        apiKey: ''
      },
      openai: {
        endpoint: 'https://api.openai.com/v1',
        model: 'gpt-4',
        temperature: 0.7,
        maxTokens: 2048,
        apiKey: ''
      },
      custom: {
        endpoint: '',
        model: '',
        temperature: 0.7,
        maxTokens: 2048,
        apiKey: ''
      }
    },
    isBusy: false,
    lastError: null,
    requestHistory: []
  }),

  getters: {
    currentProviderConfig: (state) => state.providers[state.activeProvider],
    isProviderValid: (state) => {
      const config = state.providers[state.activeProvider]
      if (state.activeProvider === 'ollama' || state.activeProvider === 'openai') {
        return config.endpoint && config.model
      }
      return config.endpoint && config.model && config.apiKey
    }
  },

  actions: {
    setActiveProvider(provider: 'ollama' | 'openai' | 'custom') {
      this.activeProvider = provider
    },

    updateProviderConfig(provider: 'ollama' | 'openai' | 'custom', config: Partial<AiBackendConfig>) {
      this.providers[provider] = { ...this.providers[provider], ...config }
    },

    setBusy(busy: boolean) {
      this.isBusy = busy
    },

    setError(error: string) {
      this.lastError = error
    },

    addRequestToHistory(provider: string, status: 'success' | 'error', message: string) {
      this.requestHistory.unshift({
        timestamp: new Date(),
        provider,
        status,
        message
      })
      // Ограничиваем историю 20 записями
      if (this.requestHistory.length > 20) {
        this.requestHistory.pop()
      }
    },

    resetError() {
      this.lastError = null
    }
  }
})
"@

$aiStorePath = Join-Path $uiDesktopPath "src\stores\aiStore.ts"
Set-Content -Path $aiStorePath -Value $aiStoreContent -Encoding UTF8
Write-Host "Создан/обновлён: $aiStorePath" -ForegroundColor Green

# 2. Создаем SettingsView.vue
Write-Host "`n[2/3] Создание/обновление SettingsView.vue..." -ForegroundColor Cyan

$settingsViewContent = @"
<script setup lang='ts'>
import { ref, computed } from 'vue'
import { useAiStore } from '../stores/aiStore'
import type { AiProvider } from '../stores/aiStore'

const aiStore = useAiStore()

const providers: AiProvider[] = [
  { id: 'ollama', name: 'Ollama', description: 'Локальный AI через Ollama' },
  { id: 'openai', name: 'OpenAI', description: 'Облачный AI через OpenAI API' },
  { id: 'custom', name: 'Пользовательский', description: 'Пользовательская конфигурация' }
]

const activeProvider = computed({
  get: () => aiStore.activeProvider,
  set: (value) => aiStore.setActiveProvider(value)
})

const currentConfig = computed(() => aiStore.currentProviderConfig)

// Методы для обновления конфига
const updateConfig = (field: string, value: string | number) => {
  const update: any = {}
  update[field] = value
  aiStore.updateProviderConfig(activeProvider.value, update)
}

// Метод для сброса ошибок
const clearError = () => {
  aiStore.resetError()
}
</script>

<template>
  <div class="settings-view">
    <h2>Настройки AI</h2>
    
    <!-- Выбор провайдера -->
    <div class="provider-selection">
      <h3>Выбор провайдера</h3>
      <div class="provider-buttons">
        <button 
          v-for="provider in providers" 
          :key="provider.id"
          :class="{ active: activeProvider === provider.id }"
          @click="activeProvider = provider.id"
        >
          {{ provider.name }}
        </button>
      </div>
      <p class="provider-description">{{ providers.find(p => p.id === activeProvider)?.description }}</p>
    </div>

    <!-- Настройки провайдера -->
    <div class="provider-config">
      <h3>Настройки {{ providers.find(p => p.id === activeProvider)?.name }}</h3>
      
      <div class="form-group">
        <label for="endpoint">Endpoint:</label>
        <input 
          id="endpoint"
          v-model="currentConfig.endpoint" 
          type="text" 
          placeholder="URL API провайдера"
        />
      </div>

      <div class="form-group">
        <label for="model">Модель:</label>
        <input 
          id="model"
          v-model="currentConfig.model" 
          type="text" 
          placeholder="Название модели"
        />
      </div>

      <div class="form-group">
        <label for="temperature">Температура:</label>
        <input 
          id="temperature"
          v-model.number="currentConfig.temperature" 
          type="number" 
          min="0" 
          max="1" 
          step="0.1"
        />
        <span class="help-text">0.0 - детерминированные ответы, 1.0 - креативные</span>
      </div>

      <div class="form-group">
        <label for="maxTokens">Максимальное количество токенов:</label>
        <input 
          id="maxTokens"
          v-model.number="currentConfig.maxTokens" 
          type="number" 
          min="1"
        />
      </div>

      <div v-if="activeProvider !== 'ollama'" class="form-group">
        <label for="apiKey">API Key:</label>
        <input 
          id="apiKey"
          v-model="currentConfig.apiKey" 
          type="password" 
          placeholder="Введите API ключ"
        />
      </div>
    </div>

    <!-- Состояние -->
    <div class="status-section" v-if="aiStore.lastError">
      <div class="error-message">
        <strong>Ошибка:</strong> {{ aiStore.lastError }}
        <button @click="clearError" class="close-btn">×</button>
      </div>
    </div>

    <div class="actions">
      <button @click="() => {}" class="btn-primary">Сохранить настройки</button>
    </div>
  </div>
</template>

<style scoped>
.settings-view {
  padding: 20px;
  max-width: 800px;
  margin: 0 auto;
}

h2 {
  color: #333;
  margin-bottom: 20px;
}

.provider-selection {
  margin-bottom: 30px;
  padding: 20px;
  border-radius: 8px;
  background-color: #f5f5f5;
}

.provider-buttons {
  display: flex;
  gap: 10px;
  margin-bottom: 15px;
}

.provider-buttons button {
  padding: 10px 15px;
  border: 1px solid #ddd;
  background: white;
  cursor: pointer;
  border-radius: 4px;
  transition: all 0.2s;
}

.provider-buttons button:hover {
  background-color: #e9ecef;
}

.provider-buttons button.active {
  background-color: #007bff;
  color: white;
  border-color: #007bff;
}

.provider-description {
  color: #666;
  font-style: italic;
}

.provider-config {
  margin-bottom: 30px;
  padding: 20px;
  border-radius: 8px;
  background-color: #f8f9fa;
}

.form-group {
  margin-bottom: 20px;
}

.form-group label {
  display: block;
  margin-bottom: 5px;
  font-weight: bold;
  color: #333;
}

.form-group input {
  width: 100%;
  padding: 10px;
  border: 1px solid #ddd;
  border-radius: 4px;
  box-sizing: border-box;
}

.help-text {
  display: block;
  margin-top: 5px;
  font-size: 0.9em;
  color: #666;
}

.status-section {
  margin-bottom: 20px;
}

.error-message {
  padding: 10px;
  background-color: #f8d7da;
  color: #721c24;
  border-radius: 4px;
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.close-btn {
  background: none;
  border: none;
  font-size: 1.5em;
  cursor: pointer;
  color: #721c24;
}

.actions {
  text-align: right;
}

.btn-primary {
  padding: 10px 20px;
  background-color: #007bff;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
}

.btn-primary:hover {
  background-color: #0056b3;
}
</style>
"@

$settingsViewPath = Join-Path $uiDesktopPath "src\views\SettingsView.vue"
Set-Content -Path $settingsViewPath -Value $settingsViewContent -Encoding UTF8
Write-Host "Создан/обновлён: $settingsViewPath" -ForegroundColor Green

# 3. Создаем UnfoldEditorView.vue
Write-Host "`n[3/3] Создание/обновление UnfoldEditorView.vue..." -ForegroundColor Cyan

$unfoldEditorContent = @"
<script setup lang='ts'>
import { ref, onMounted, onUnmounted } from 'vue'
import { useAiStore } from '../stores/aiStore'
import { useProjectStore } from '../stores/project.store'
import type { Project } from '../stores/project.store'

const aiStore = useAiStore()
const projectStore = useProjectStore()

const canvasRef = ref<HTMLCanvasElement | null>(null)
const is3DView = ref(true)
const isLoading = ref(false)
const statusMessage = ref('')
const errorMessage = ref('')

// Имитация загрузки модели
const loadModel = () => {
  if (isLoading.value) return
  
  isLoading.value = true
  statusMessage.value = 'Загрузка модели...'
  
  setTimeout(() => {
    isLoading.value = false
    statusMessage.value = 'Модель загружена'
  }, 1500)
}

// Имитация генерации швов
const generateSeams = () => {
  if (aiStore.isBusy) return
  
  aiStore.setBusy(true)
  statusMessage.value = 'Генерация швов...'
  
  setTimeout(() => {
    aiStore.setBusy(false)
    statusMessage.value = 'Швы сгенерированы успешно'
    aiStore.addRequestToHistory(aiStore.activeProvider, 'success', 'Генерация швов завершена')
  }, 2000)
}

// Имитация экспорта
const exportModel = () => {
  if (aiStore.isBusy) return
  
  aiStore.setBusy(true)
  statusMessage.value = 'Экспорт модели...'
  
  setTimeout(() => {
    aiStore.setBusy(false)
    statusMessage.value = 'Модель экспортирована'
    aiStore.addRequestToHistory(aiStore.activeProvider, 'success', 'Экспорт модели завершен')
  }, 1500)
}

// Имитация очистки
const clearScene = () => {
  statusMessage.value = 'Сцена очищена'
}

// Имитация переключения вида
const toggleView = () => {
  is3DView.value = !is3DView.value
  statusMessage.value = is3DView.value ? 'Переключено на 3D вид' : 'Переключено на 2D вид'
}

// Инициализация
onMounted(() => {
  statusMessage.value = 'Готов к работе'
})

onUnmounted(() => {
  // Очистка при размонтировании
})
</script>

<template>
  <div class="unfold-editor-view">
    <!-- Верхняя панель -->
    <div class="top-bar">
      <h1>Редактор развёртки 3D моделей</h1>
      <div class="status-indicator">
        <span :class="{ 'busy': aiStore.isBusy }">{{ aiStore.isBusy ? 'Обработка...' : statusMessage }}</span>
      </div>
    </div>

    <div class="main-content">
      <!-- Левая панель управления -->
      <div class="left-panel">
        <div class="panel-section">
          <h3>Управление сценой</h3>
          
          <div class="control-group">
            <button @click="loadModel" :disabled="isLoading" class="btn-secondary">
              {{ isLoading ? 'Загрузка...' : 'Загрузить модель' }}
            </button>
            <button @click="clearScene" class="btn-secondary">Очистить</button>
          </div>
          
          <div class="control-group">
            <button @click="toggleView" class="btn-secondary">
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
            <button @click="generateSeams" :disabled="aiStore.isBusy" class="btn-primary">
              {{ aiStore.isBusy ? 'Обработка...' : 'Сгенерировать швы' }}
            </button>
          </div>
          
          <div class="control-group">
            <button @click="exportModel" :disabled="aiStore.isBusy" class="btn-success">
              {{ aiStore.isBusy ? 'Экспорт...' : 'Экспорт' }}
            </button>
          </div>
        </div>

        <div class="panel-section">
          <h3>История запросов</h3>
          <div class="history-list">
            <div 
              v-for="(item, index) in aiStore.requestHistory.slice(0, 5)" 
              :key="index"
              class="history-item"
            >
              <span>{{ item.timestamp.toLocaleTimeString() }}</span>
              <span :class="item.status">{{ item.provider }}: {{ item.message }}</span>
            </div>
          </div>
        </div>
      </div>

      <!-- Центральная область с 3D/2D видом -->
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
              <button @click="loadModel" class="btn-primary">Загрузить модель</button>
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

.history-list {
  font-size: 0.85em;
}

.history-item {
  padding: 5px 0;
  border-bottom: 1px solid #eee;
}

.history-item:last-child {
  border-bottom: none;
}

.history-item span:first-child {
  color: #666;
  margin-right: 10px;
}

.history-item span:last-child {
  color: #28a745;
}

.history-item span:last-child.error {
  color: #dc3545;
}
</style>
"@

$unfoldEditorPath = Join-Path $uiDesktopPath "src\views\UnfoldEditorView.vue"
Set-Content -Path $unfoldEditorPath -Value $unfoldEditorContent -Encoding UTF8
Write-Host "Создан/обновлён: $unfoldEditorPath" -ForegroundColor Green

Write-Host "`n=== RESTORE UI-DESKTOP: DONE ===" -ForegroundColor Cyan
Write-Host "Созданы/обновлены файлы:" -ForegroundColor Green
Write-Host "  ✓ $aiStorePath" -ForegroundColor Green
Write-Host "  ✓ $settingsViewPath" -ForegroundColor Green
Write-Host "  ✓ $unfoldEditorPath" -ForegroundColor Green
Write-Host "" -ForegroundColor Green
Write-Host "Теперь можно запустить:" -ForegroundColor Cyan
Write-Host "  cd D:\Dev\pepakura-next\packages\ui-desktop" -ForegroundColor Yellow
Write-Host "  pnpm tauri dev" -ForegroundColor Yellow
