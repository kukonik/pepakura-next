<template>
  <div class="ai-assistant-panel">
    <div class="panel-header">
      <h3>AI Ассистент</h3>
      <div class="status-indicator" :class="aiStatus">
        {{ statusIcon }} {{ statusText }}
      </div>
    </div>

    <div class="panel-content">
      <div class="ai-actions">
        <button @click="analyzeModel" :disabled="!aiAvailable || loading || !hasModel" class="action-button analyze-button" title="Анализ текущей 3D модели">
          🧠 AI Анализ модели
        </button>
        <button @click="generateInstructions" :disabled="!aiAvailable || loading || !hasModel" class="action-button instructions-button" title="Генерация инструкций по сборке">
          📋 Сгенерировать инструкции
        </button>
        <button @click="checkStatus" :disabled="loading" class="action-button status-button" title="Проверить статус Ollama">
          🔄 Проверить статус
        </button>
      </div>

      <div v-if="!hasModel" class="warning-section">
        <div class="warning-message">
          ⚠️ Для анализа загрузите 3D модель
        </div>
      </div>

      <div class="input-section">
        <textarea
          v-model="userPrompt"
          placeholder="Задайте вопрос или опишите задачу..."
          rows="3"
          @keydown.enter.exact.prevent="sendPrompt"
        ></textarea>
        <button @click="sendPrompt" :disabled="!aiAvailable || loading">
          {{ loading ? 'Обработка...' : 'Отправить' }}
        </button>
      </div>

      <div class="suggestions">
        <h4>Подсказки:</h4>
        <div class="suggestion-list">
          <button
            v-for="(suggestion, index) in suggestions"
            :key="index"
            @click="applySuggestion(suggestion)"
            class="suggestion-item"
          >
            {{ suggestion }}
          </button>
        </div>
      </div>

      <div class="response-section" v-if="response">
        <h4>Ответ AI:</h4>
        <div class="response-content" v-html="formattedResponse"></div>
      </div>

      <div class="analysis-result" v-if="analysisResult">
        <h4>Результат анализа:</h4>
        <div class="result-content">
          <div v-if="analysisResult.type === 'advice'">
            <p><strong>Алгоритм:</strong> {{ analysisResult.data.algorithm }}</p>
            <p><strong>Максимум итераций:</strong> {{ analysisResult.data.max_iterations }}</p>
            <p><strong>Допуск:</strong> {{ analysisResult.data.tolerance }}</p>
            <div v-if="analysisResult.data.tips.length">
              <strong>Советы:</strong>
              <ul>
                <li v-for="(tip, idx) in analysisResult.data.tips" :key="idx">{{ tip }}</li>
              </ul>
            </div>
            <div v-if="analysisResult.data.potential_issues.length">
              <strong>Возможные проблемы:</strong>
              <ul>
                <li v-for="(issue, idx) in analysisResult.data.potential_issues" :key="idx">{{ issue }}</li>
              </ul>
            </div>
          </div>
          <div v-else-if="analysisResult.type === 'instructions'">
            <p><strong>Модель:</strong> {{ analysisResult.data.model_name }}</p>
            <p><strong>Сложность:</strong> {{ analysisResult.data.difficulty }}</p>
            <p><strong>Общее время сборки:</strong> {{ analysisResult.data.total_time_minutes }} минут</p>
            <div v-if="analysisResult.data.tips.length">
              <strong>Советы:</strong>
              <ul>
                <li v-for="(tip, idx) in analysisResult.data.tips" :key="idx">{{ tip }}</li>
              </ul>
            </div>
            <div v-if="analysisResult.data.steps.length">
              <strong>Шаги сборки:</strong>
              <ol>
                <li v-for="step in analysisResult.data.steps" :key="step.step_number">
                  <strong>Шаг {{ step.step_number }}:</strong> {{ step.description }} (детали: {{ step.part_ids.join(', ') }}) – {{ step.estimated_time_minutes }} мин
                </li>
              </ol>
            </div>
          </div>
          <div v-else>
            <pre>{{ analysisResult.raw }}</pre>
          </div>
        </div>
      </div>

      <div class="error-section" v-if="error">
        <div class="error-message">
          <strong>Ошибка:</strong> {{ error }}
          <button @click="clearError" class="error-close">×</button>
        </div>
      </div>

      <div class="info-section" v-if="!aiAvailable && aiStatus === 'offline'">
        <div class="info-message">
          <strong>AI недоступен:</strong> Убедитесь, что Ollama запущен на localhost:11434
          <button @click="checkStatus" class="retry-button">Повторить проверку</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted, onUnmounted, computed } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'
import { useProjectStore } from '../stores/projectStore'

const projectStore = useProjectStore()

const userPrompt = ref('')
const loading = ref(false)
const response = ref('')
const error = ref('')
const aiStatus = ref('checking') // 'checking', 'online', 'offline'
const aiAvailable = ref(false)
const analysisResult = ref(null)

const suggestions = ref([
  '🔍 Анализировать искажения развёртки',
  '📊 Оптимизировать раскладку на листе',
  '📋 Сгенерировать инструкции по сборке',
  '⚠️ Найти проблемные грани',
  '💡 Предложить улучшения модели',
  '📄 Рекомендовать формат бумаги',
  '🎯 Оценить качество развёртки',
  '🔧 Оптимизировать сложность сборки'
])

const hasModel = computed(() => projectStore.hasModel)

const statusIcon = computed(() => {
  switch (aiStatus.value) {
    case 'online': return '🟢'
    case 'offline': return '🔴'
    default: return '🟡'
  }
})

const statusText = computed(() => {
  switch (aiStatus.value) {
    case 'online': return `Ollama онлайн (${aiAvailable.value ? 'доступен' : 'недоступен'})`
    case 'offline': return 'Ollama офлайн'
    default: return 'Проверка статуса...'
  }
})

const formattedResponse = computed(() => {
  return response.value.replace(/\n/g, '<br>').replace(/\*\*(.*?)\*\*/g, '<strong>$1</strong>')
})

// Проверка статуса AI через Tauri команду
async function checkStatus() {
  try {
    aiStatus.value = 'checking'
    const status = await invoke('ai_check_status')
    aiAvailable.value = status.available
    aiStatus.value = status.available ? 'online' : 'offline'
    error.value = ''
  } catch (err) {
    aiStatus.value = 'offline'
    aiAvailable.value = false
    error.value = `Ошибка проверки статуса: ${err.message || err}`
  }
}

// Отправка промпта через AI чат
async function sendPrompt() {
  if (!userPrompt.value.trim()) return

  loading.value = true
  response.value = ''
  error.value = ''
  analysisResult.value = null

  try {
    const result = await invoke('ai_chat', {
      message: userPrompt.value,
      history: []
    })
    response.value = result
  } catch (err) {
    error.value = `Ошибка AI: ${err.message || err}`
  } finally {
    loading.value = false
  }
}

// Анализ модели
async function analyzeModel() {
  if (!hasModel.value) {
    error.value = 'Нет загруженной модели для анализа'
    return
  }

  loading.value = true
  error.value = ''
  response.value = ''
  analysisResult.value = null

  try {
    // Получаем текущую модель из store
    const mesh = projectStore.currentMesh
    if (!mesh) {
      throw new Error('Модель не найдена')
    }
    // Преобразуем MeshData в формат, ожидаемый бэкендом (пока заглушка)
    const meshForBackend = {
      vertices: mesh.vertices.map(v => [v.x, v.y, v.z]),
      faces: mesh.faces.map(f => [f.a, f.b, f.c])
    }
    const advice = await invoke('ai_get_unfold_advice', { mesh: meshForBackend })
    analysisResult.value = {
      type: 'advice',
      data: advice,
      raw: JSON.stringify(advice, null, 2)
    }
  } catch (err) {
    error.value = `Ошибка анализа: ${err.message || err}`
  } finally {
    loading.value = false
  }
}

// Генерация инструкций по сборке
async function generateInstructions() {
  if (!hasModel.value) {
    error.value = 'Нет загруженной модели для генерации инструкций'
    return
  }

  loading.value = true
  error.value = ''
  response.value = ''
  analysisResult.value = null

  try {
    const mesh = projectStore.currentMesh
    if (!mesh) {
      throw new Error('Модель не найдена')
    }
    const meshForBackend = {
      vertices: mesh.vertices.map(v => [v.x, v.y, v.z]),
      faces: mesh.faces.map(f => [f.a, f.b, f.c])
    }
    const instructions = await invoke('ai_generate_instructions', { mesh: meshForBackend })
    analysisResult.value = {
      type: 'instructions',
      data: instructions,
      raw: JSON.stringify(instructions, null, 2)
    }
  } catch (err) {
    error.value = `Ошибка генерации инструкций: ${err.message || err}`
  } finally {
    loading.value = false
  }
}

// Применение подсказки
function applySuggestion(suggestion) {
  userPrompt.value = suggestion
}

// Очистка ошибки
function clearError() {
  error.value = ''
}

// Периодическая проверка статуса
let statusInterval
onMounted(() => {
  checkStatus()
  statusInterval = setInterval(checkStatus, 30000) // каждые 30 секунд
})

onUnmounted(() => {
  if (statusInterval) clearInterval(statusInterval)
})
</script>

<style scoped>
.ai-assistant-panel {
  border: 1px solid #e0e0e0;
  border-radius: 8px;
  padding: 16px;
  background: #f9f9f9;
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
}

.panel-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 16px;
}

.panel-header h3 {
  margin: 0;
  font-size: 1.2rem;
  color: #333;
}

.status-indicator {
  font-size: 0.85rem;
  padding: 4px 8px;
  border-radius: 12px;
  background: #eee;
}

.status-indicator.online {
  background: #d4edda;
  color: #155724;
}

.status-indicator.offline {
  background: #f8d7da;
  color: #721c24;
}

.status-indicator.checking {
  background: #fff3cd;
  color: #856404;
}

.ai-actions {
  display: flex;
  gap: 8px;
  margin-bottom: 20px;
  flex-wrap: wrap;
}

.action-button {
  padding: 8px 12px;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 0.9rem;
  flex: 1;
  min-width: 120px;
}

.analyze-button {
  background: #6f42c1;
  color: white;
}

.instructions-button {
  background: #20c997;
  color: white;
}

.status-button {
  background: #6c757d;
  color: white;
}

.action-button:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.warning-section {
  padding: 12px;
  background: #fff3cd;
  border: 1px solid #ffeaa7;
  border-radius: 4px;
  color: #856404;
  margin-bottom: 16px;
}

.input-section {
  margin-bottom: 20px;
}

textarea {
  width: 100%;
  padding: 10px;
  border: 1px solid #ccc;
  border-radius: 4px;
  font-family: inherit;
  font-size: 0.95rem;
  resize: vertical;
  margin-bottom: 10px;
}

button {
  padding: 8px 16px;
  background: #007bff;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 0.95rem;
}

button:disabled {
  background: #6c757d;
  cursor: not-allowed;
}

.suggestions {
  margin-bottom: 20px;
}

.suggestions h4 {
  margin-top: 0;
  margin-bottom: 10px;
  font-size: 1rem;
  color: #555;
}

.suggestion-list {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
}

.suggestion-item {
  padding: 6px 12px;
  background: #e9ecef;
  border: 1px solid #dee2e6;
  border-radius: 20px;
  font-size: 0.85rem;
  cursor: pointer;
  transition: background 0.2s;
}

.suggestion-item:hover {
  background: #d0d7e0;
}

.response-section {
  padding: 12px;
  background: #fff;
  border: 1px solid #d1ecf1;
  border-radius: 4px;
  margin-bottom: 16px;
}

.response-section h4 {
  margin-top: 0;
  margin-bottom: 8px;
  color: #0c5460;
}

.response-content {
  white-space: pre-wrap;
  font-size: 0.95rem;
  line-height: 1.4;
}

.analysis-result {
  padding: 12px;
  background: #f8f9fa;
  border: 1px solid #dee2e6;
  border-radius: 4px;
  margin-bottom: 16px;
}

.analysis-result h4 {
  margin-top: 0;
  margin-bottom: 8px;
  color: #495057;
}

.result-content {
  font-size: 0.9rem;
  line-height: 1.5;
}

.result-content ul,
.result-content ol {
  margin: 8px 0;
  padding-left: 20px;
}

.result-content li {
  margin-bottom: 4px;
}

.error-section {
  padding: 12px;
  background: #f8d7da;
  border: 1px solid #f5c6cb;
  border-radius: 4px;
  color: #721c24;
  margin-bottom: 16px;
  position: relative;
}

.error-close {
  position: absolute;
  top: 8px;
  right: 8px;
  background: transparent;
  color: #721c24;
  border: none;
  font-size: 1.2rem;
  padding: 0;
  width: 24px;
  height: 24px;
  display: flex;
  align-items: center;
  justify-content: center;
}

.retry-button {
  margin-left: 8px;
  padding: 4px 8px;
  background: #0c5460;
  color: white;
  border: none;
  border-radius: 4px;
  font-size: 0.8rem;
}

.info-section {
  padding: 12px;
  background: #d1ecf1;
  border: 1px solid #bee5eb;
  border-radius: 4px;
  color: #0c5460;
}
</style>