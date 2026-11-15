<template>
  <div class="ai-assistant-panel">
    <!-- Header со статусом -->
    <div class="panel-header">
      <h3>{{ t('ai.assistant.title') }}</h3>
      <div class="header-actions">
        <!-- Индикатор статуса Ollama -->
        <div class="status-indicator" :class="statusClass" :title="statusTooltip">
          <span class="status-dot"></span>
          <span class="status-text">{{ statusText }}</span>
        </div>
        <button @click="refreshStatus" :disabled="aiStore.ollamaStatus.checking" class="icon-btn" :title="$t('common.loading')">
          <svg v-if="aiStore.ollamaStatus.checking" class="spinner" viewBox="0 0 24 24" width="16" height="16">
            <circle cx="12" cy="12" r="10" stroke="currentColor" stroke-width="3" fill="none" stroke-dasharray="31.4 31.4"/>
          </svg>
          <svg v-else viewBox="0 0 24 24" width="16" height="16">
            <path d="M17.65 6.35A7.958 7.958 0 0 0 12 4c-4.42 0-7.99 3.58-7.99 8s3.57 8 7.99 8c3.73 0 6.84-2.55 7.73-6h-2.08A5.99 5.99 0 0 1 12 18c-3.31 0-6-2.69-6-6s2.69-6 6-6c1.66 0 3.14.69 4.22 1.78L13 11h7V4l-2.35 2.35z"/>
          </svg>
        </button>
      </div>
    </div>

    <!-- Tabs -->
    <div class="tabs">
      <button 
        :class="['tab', { active: activeTab === 'chat' }]"
        @click="activeTab = 'chat'"
      >
        💬 {{ $t('ai.assistant.chat.send') }}
      </button>
      <button 
        :class="['tab', { active: activeTab === 'analysis' }]"
        @click="activeTab = 'analysis'"
      >
        📊 {{ $t('ai.assistant.analyze.title') }}
      </button>
    </div>

    <!-- Content -->
    <div class="panel-content">
      <!-- Chat Tab -->
      <div v-show="activeTab === 'chat'" class="tab-content chat-content">
        <AiMessageList :messages="aiStore.chatMessages" />
        
        <div v-if="aiStore.lastError" class="error-banner">
          {{ aiStore.lastError }}
        </div>

        <div class="input-container">
          <textarea
            v-model="inputMessage"
            :placeholder="$t('ai.assistant.chat.placeholder')"
            :disabled="aiStore.isBusy || !aiStore.ollamaStatus.available"
            @keydown.enter.exact.prevent="sendMessage"
            @keydown.enter.shift.exact.prevent="inputMessage += '\n'"
            rows="3"
          ></textarea>
          <div class="input-actions">
            <button
              @click="clearChat"
              :disabled="aiStore.chatMessages.length === 0"
              class="secondary-btn"
            >
              {{ $t('ai.assistant.chat.clear') }}
            </button>
            <button
              @click="sendMessage"
              :disabled="aiStore.isBusy || !inputMessage.trim() || !aiStore.ollamaStatus.available"
              class="send-btn"
            >
              {{ aiStore.isBusy ? $t('ai.assistant.chat.sending') : $t('ai.assistant.chat.send') }}
            </button>
          </div>
        </div>
      </div>

      <!-- Analysis Tab -->
      <div v-show="activeTab === 'analysis'" class="tab-content analysis-content">
        <div class="analysis-actions">
          <button
            @click="runAnalysis"
            :disabled="aiStore.isBusy || !aiStore.ollamaStatus.available"
            class="analyze-btn"
          >
            <span v-if="aiStore.isBusy" class="spinner-inline"></span>
            {{ $t('ai.assistant.analyze.button') }}
          </button>
        </div>

        <!-- Результаты анализа -->
        <div v-if="aiStore.analysisResults.length === 0" class="no-results">
          <p>{{ $t('ai.assistant.analysis.noResults') }}</p>
        </div>

        <div v-for="(result, idx) in aiStore.analysisResults" :key="idx" class="analysis-result">
          <div class="result-header">
            <h4>{{ getResultTitle(result.type) }}</h4>
            <span class="result-time">{{ formatTime(result.timestamp) }}</span>
          </div>
          
          <div class="result-content" v-if="result.type === 'unfold'">
            <div class="result-item">
              <strong>{{ $t('ai.assistant.analysis.algorithm') }}:</strong>
              <span>{{ result.data.algorithm || 'N/A' }}</span>
            </div>
            <div class="result-item">
              <strong>{{ $t('ai.assistant.analysis.maxIterations') }}:</strong>
              <span>{{ result.data.max_iterations || 'N/A' }}</span>
            </div>
            <div class="result-item">
              <strong>{{ $t('ai.assistant.analysis.tolerance') }}:</strong>
              <span>{{ result.data.tolerance || 'N/A' }}</span>
            </div>
            
            <div v-if="result.data.tips && result.data.tips.length" class="result-section">
              <strong>💡 {{ $t('ai.assistant.analysis.tips') }}:</strong>
              <ul>
                <li v-for="(tip, i) in result.data.tips" :key="i">{{ tip }}</li>
              </ul>
            </div>
            
            <div v-if="result.data.potential_issues && result.data.potential_issues.length" class="result-section">
              <strong>⚠️ {{ $t('ai.assistant.analysis.potentialIssues') }}:</strong>
              <ul>
                <li v-for="(issue, i) in result.data.potential_issues" :key="i" class="issue-item">{{ issue }}</li>
              </ul>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'
import { useI18n } from 'vue-i18n'
import { useAiStore } from '../../stores/aiStore'
import AiMessageList from './AiMessageList.vue'
import { useProjectStore } from '../../stores/projectStore'

const { t } = useI18n()
const aiStore = useAiStore()
const projectStore = useProjectStore()

const activeTab = ref<'chat' | 'analysis'>('chat')
const inputMessage = ref('')

// ===========================================================================
// Status
// ===========================================================================

const statusClass = computed(() => {
  if (aiStore.ollamaStatus.checking) return 'checking'
  return aiStore.ollamaStatus.available ? 'online' : 'offline'
})

const statusText = computed(() => {
  if (aiStore.ollamaStatus.checking) return t('ai.assistant.status.checking')
  return aiStore.ollamaStatus.available 
    ? t('ai.assistant.status.online') 
    : t('ai.assistant.status.offline')
})

const statusTooltip = computed(() => {
  if (aiStore.ollamaStatus.checking) return t('ai.assistant.status.checking')
  if (aiStore.ollamaStatus.available) {
    const models = aiStore.ollamaStatus.models.join(', ')
    return `${t('ai.assistant.status.online')}${models ? ': ' + models : ''}`
  }
  return t('ai.assistant.errors.ollamaUnavailable')
})

async function refreshStatus() {
  try {
    await aiStore.checkOllamaStatus()
  } catch (err) {
    console.warn('Status refresh failed:', err)
  }
}

// ===========================================================================
// Chat
// ===========================================================================

async function sendMessage() {
  if (!inputMessage.value.trim() || aiStore.isBusy) return

  const message = inputMessage.value.trim()
  inputMessage.value = ''

  try {
    // Store сам добавит сообщения пользователя и AI, а также запустит стриминг
    await aiStore.sendMessageToAI(message)
  } catch (error) {
    console.error('Failed to send message:', error)
    // Ошибка уже обработана в store
  }
}

function clearChat() {
  aiStore.clearChat()
}

// ===========================================================================
// Analysis
// ===========================================================================

async function runAnalysis() {
  // Проверяем, есть ли загруженная модель
  if (!projectStore.currentModel) {
    aiStore.lastError = t('ai.assistant.errors.noModel')
    return
  }

  try {
    await aiStore.analyzeModel(projectStore.currentModel)
  } catch (error) {
    console.error('Analysis failed:', error)
  }
}

function getResultTitle(type: string): string {
  const titles: Record<string, string> = {
    'unfold': t('ai.assistant.analyze.title'),
    'distortion': 'Анализ искажений',
    'nesting': 'Анализ раскладки'
  }
  return titles[type] || type
}

function formatTime(date: Date): string {
  return date.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' })
}

// ===========================================================================
// Lifecycle
// ===========================================================================

onMounted(async () => {
  // Проверяем статус при загрузке
  await refreshStatus()

  // Запускаем периодическую проверку (каждые 30 секунд)
  aiStore.startStatusPolling(30000)
})

onUnmounted(() => {
  // Очищаем слушатели стриминга при размонтировании
  aiStore.cleanupStreamingListeners()
})
</script>

<style scoped>
.ai-assistant-panel {
  display: flex;
  flex-direction: column;
  height: 100%;
  background: var(--bg-secondary);
  border-radius: 8px;
  overflow: hidden;
}

/* Header */
.panel-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px 16px;
  background: var(--bg-primary);
  border-bottom: 1px solid var(--border-color);
}

.panel-header h3 {
  margin: 0;
  font-size: 16px;
  font-weight: 600;
  color: var(--text-primary);
}

.header-actions {
  display: flex;
  align-items: center;
  gap: 12px;
}

/* Status Indicator */
.status-indicator {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 4px 10px;
  border-radius: 12px;
  font-size: 12px;
  font-weight: 500;
}

.status-indicator.online {
  background: rgba(16, 185, 129, 0.15);
  color: var(--success);
}

.status-indicator.offline {
  background: rgba(239, 68, 68, 0.15);
  color: var(--error);
}

.status-indicator.checking {
  background: rgba(245, 158, 11, 0.15);
  color: var(--warning);
}

.status-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background: currentColor;
}

.status-indicator.online .status-dot {
  animation: pulse 2s infinite;
}

@keyframes pulse {
  0%, 100% { opacity: 1; }
  50% { opacity: 0.5; }
}

/* Icon Button */
.icon-btn {
  background: transparent;
  border: none;
  color: var(--text-secondary);
  cursor: pointer;
  padding: 4px;
  border-radius: 4px;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.2s;
}

.icon-btn:hover:not(:disabled) {
  background: var(--bg-secondary);
  color: var(--text-primary);
}

.icon-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.spinner {
  animation: rotate 1s linear infinite;
}

@keyframes rotate {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}

/* Tabs */
.tabs {
  display: flex;
  background: var(--bg-primary);
  border-bottom: 1px solid var(--border-color);
}

.tab {
  flex: 1;
  padding: 10px 16px;
  background: transparent;
  border: none;
  color: var(--text-secondary);
  cursor: pointer;
  font-size: 13px;
  font-weight: 500;
  transition: all 0.2s;
  border-bottom: 2px solid transparent;
}

.tab:hover {
  background: var(--bg-secondary);
  color: var(--text-primary);
}

.tab.active {
  color: var(--primary);
  border-bottom-color: var(--primary);
  background: rgba(99, 102, 241, 0.1);
}

/* Content */
.panel-content {
  flex: 1;
  overflow: hidden;
  display: flex;
  flex-direction: column;
}

.tab-content {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

/* Chat */
.chat-content {
  height: 100%;
}

.input-container {
  padding: 16px;
  background: var(--bg-primary);
  border-top: 1px solid var(--border-color);
}

.input-container textarea {
  width: 100%;
  padding: 12px;
  border: 1px solid var(--border-color);
  border-radius: 8px;
  background: var(--bg-secondary);
  color: var(--text-primary);
  font-family: inherit;
  font-size: 14px;
  resize: vertical;
  margin-bottom: 10px;
  transition: border-color 0.2s;
}

.input-container textarea:focus {
  outline: none;
  border-color: var(--primary);
  box-shadow: 0 0 0 2px rgba(99, 102, 241, 0.2);
}

.input-container textarea:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.input-actions {
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 10px;
}

.secondary-btn {
  padding: 8px 16px;
  background: transparent;
  border: 1px solid var(--border-color);
  border-radius: 6px;
  color: var(--text-secondary);
  cursor: pointer;
  font-size: 13px;
  transition: all 0.2s;
}

.secondary-btn:hover:not(:disabled) {
  background: var(--bg-secondary);
  color: var(--text-primary);
  border-color: var(--text-secondary);
}

.secondary-btn:disabled {
  opacity: 0.4;
  cursor: not-allowed;
}

.send-btn {
  padding: 8px 20px;
  background: var(--primary);
  color: white;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  font-size: 14px;
  font-weight: 500;
  transition: all 0.2s;
}

.send-btn:hover:not(:disabled) {
  background: var(--primary-dark);
}

.send-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

/* Error Banner */
.error-banner {
  padding: 10px 16px;
  background: rgba(239, 68, 68, 0.1);
  border-top: 1px solid var(--error);
  color: var(--error);
  font-size: 13px;
}

/* Analysis */
.analysis-content {
  padding: 16px;
  overflow-y: auto;
}

.analysis-actions {
  margin-bottom: 16px;
}

.analyze-btn {
  width: 100%;
  padding: 12px 20px;
  background: linear-gradient(135deg, var(--primary), var(--primary-dark));
  color: white;
  border: none;
  border-radius: 8px;
  cursor: pointer;
  font-size: 14px;
  font-weight: 600;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  transition: all 0.2s;
}

.analyze-btn:hover:not(:disabled) {
  transform: translateY(-1px);
  box-shadow: 0 4px 12px rgba(99, 102, 241, 0.3);
}

.analyze-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
  transform: none;
  box-shadow: none;
}

.spinner-inline {
  display: inline-block;
  width: 14px;
  height: 14px;
  border: 2px solid rgba(255, 255, 255, 0.3);
  border-top-color: white;
  border-radius: 50%;
  animation: rotate 0.8s linear infinite;
}

.no-results {
  text-align: center;
  padding: 40px 20px;
  color: var(--text-secondary);
  font-size: 14px;
}

.analysis-result {
  background: var(--bg-primary);
  border: 1px solid var(--border-color);
  border-radius: 8px;
  padding: 16px;
  margin-bottom: 12px;
}

.result-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 12px;
  padding-bottom: 8px;
  border-bottom: 1px solid var(--border-color);
}

.result-header h4 {
  margin: 0;
  font-size: 15px;
  font-weight: 600;
  color: var(--text-primary);
}

.result-time {
  font-size: 12px;
  color: var(--text-secondary);
}

.result-content {
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.result-item {
  display: flex;
  justify-content: space-between;
  padding: 8px 0;
  font-size: 13px;
}

.result-item strong {
  color: var(--text-secondary);
  margin-right: 8px;
}

.result-section {
  margin-top: 12px;
  padding: 12px;
  background: var(--bg-secondary);
  border-radius: 6px;
}

.result-section strong {
  display: block;
  margin-bottom: 8px;
  color: var(--text-primary);
  font-size: 13px;
}

.result-section ul {
  margin: 0;
  padding-left: 20px;
  list-style-type: disc;
}

.result-section li {
  margin-bottom: 6px;
  font-size: 13px;
  line-height: 1.5;
  color: var(--text-primary);
}

.issue-item {
  color: var(--warning) !important;
}
</style>
