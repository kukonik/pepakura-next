<template>
  <div class="ai-assistant">
    <div class="ai-header">
      <h3>🤖 AI-помощник</h3>
      <button 
        @click="showSettings = !showSettings" 
        class="settings-btn"
        title="Настройки"
      >
        ⚙️
      </button>
    </div>

    <!-- Статус подключения -->
    <div v-if="!aiStore.isAvailable" class="ai-unavailable">
      <p class="warning">Ollama не найдена</p>
      <p class="hint">
        Установите Ollama: 
        <a href="https://ollama.ai" target="_blank">ollama.ai</a>
      </p>
      <button @click="aiStore.checkStatus()" class="retry-btn">
        Проверить снова
      </button>
    </div>

    <!-- Чат -->
    <div v-else class="ai-chat">
      <div class="messages" ref="messagesContainer">
        <div
          v-for="(msg, index) in messages"
          :key="index"
          :class="['message', msg.role]"
        >
          <div class="message-avatar">
            {{ msg.role === 'user' ? '👤' : '🤖' }}
          </div>
          <div class="message-content">
            {{ msg.content }}
          </div>
        </div>
        
        <div v-if="isLoading" class="message assistant typing">
          <div class="message-avatar">🤖</div>
          <div class="message-content">
            <span class="typing-indicator">Печатает...</span>
          </div>
        </div>
      </div>

      <!-- Ввод сообщения -->
      <div class="input-area">
        <textarea
          v-model="userInput"
          placeholder="Спросите о развёртке..."
          @keydown.enter.exact.prevent="sendMessage"
          :disabled="isLoading"
          rows="3"
        />
        <button 
          @click="sendMessage" 
          :disabled="isLoading || !userInput.trim()"
          class="send-btn"
        >
          ➤
        </button>
      </div>
    </div>

    <!-- Настройки -->
    <div v-if="showSettings" class="settings-panel">
      <h4>Настройки AI</h4>
      
      <div class="setting">
        <label>Провайдер</label>
        <select v-model="localConfig.provider">
          <option value="Ollama">Ollama (локально)</option>
          <option value="OpenAI">OpenAI (облако)</option>
        </select>
      </div>

      <div v-if="localConfig.provider === 'Ollama'" class="setting">
        <label>URL</label>
        <input 
          v-model="localConfig.ollama_url" 
          type="text"
          placeholder="http://localhost:11434"
        />
      </div>

      <div class="setting">
        <label>Модель</label>
        <input 
          v-model="localConfig.model" 
          type="text"
          placeholder="llama3.2"
        />
      </div>

      <div class="setting">
        <label>Температура: {{ localConfig.temperature }}</label>
        <input 
          v-model.number="localConfig.temperature" 
          type="range"
          min="0" max="1" step="0.1"
        />
      </div>

      <div class="setting-actions">
        <button @click="saveSettings" class="primary">Сохранить</button>
        <button @click="showSettings = false" class="secondary">Закрыть</button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, nextTick } from 'vue'
import { useAiStore } from '@/stores/ai.store'
import { useAi, type ChatMessage } from '@/composables/useAi'

const aiStore = useAiStore()
const { chat } = useAi()

// State
const messages = ref<ChatMessage[]>([])
const userInput = ref('')
const isLoading = ref(false)
const showSettings = ref(false)

const localConfig = reactive({ ...aiStore.config })

// Methods
async function sendMessage() {
  if (!userInput.value.trim() || isLoading.value) return

  const userMessage = userInput.value.trim()
  userInput.value = ''
  
  // Добавляем сообщение пользователя
  messages.value.push({ role: 'user', content: userMessage })
  isLoading.value = true

  try {
    // Отправляем в AI
    const response = await chat(userMessage, messages.value)
    
    // Добавляем ответ
    messages.value.push({ role: 'assistant', content: response })
  } catch (error) {
    messages.value.push({ 
      role: 'assistant', 
      content: `Ошибка: ${error}` 
    })
  } finally {
    isLoading.value = false
    await nextTick()
    scrollToBottom()
  }
}

function scrollToBottom() {
  const container = messagesContainer.value
  if (container) {
    container.scrollTop = container.scrollHeight
  }
}

async function saveSettings() {
  try {
    await aiStore.updateConfig(localConfig)
    showSettings.value = false
    await aiStore.checkStatus()
  } catch (error) {
    alert(`Ошибка: ${error}`)
  }
}

// Lifecycle
import { onMounted } from 'vue'
onMounted(async () => {
  await aiStore.checkStatus()
  localConfig.provider = aiStore.config.provider
  localConfig.ollama_url = aiStore.config.ollama_url
  localConfig.model = aiStore.config.model
  localConfig.temperature = aiStore.config.temperature
})

const messagesContainer = ref<HTMLDivElement | null>(null)
</script>

<style scoped>
.ai-assistant {
  display: flex;
  flex-direction: column;
  height: 100%;
  border: 1px solid #ddd;
  border-radius: 8px;
  overflow: hidden;
}

.ai-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px 16px;
  background: #f5f5f5;
  border-bottom: 1px solid #ddd;
}

.ai-header h3 {
  margin: 0;
  font-size: 16px;
}

.settings-btn {
  background: none;
  border: none;
  font-size: 20px;
  cursor: pointer;
  padding: 4px;
}

.ai-unavailable {
  padding: 20px;
  text-align: center;
}

.warning {
  color: #d32f2f;
  font-weight: bold;
}

.hint {
  color: #666;
  font-size: 14px;
  margin: 8px 0;
}

.hint a {
  color: #1976d2;
}

.retry-btn {
  margin-top: 12px;
  padding: 8px 16px;
  background: #1976d2;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
}

.ai-chat {
  display: flex;
  flex-direction: column;
  flex: 1;
  overflow: hidden;
}

.messages {
  flex: 1;
  overflow-y: auto;
  padding: 16px;
}

.message {
  display: flex;
  gap: 8px;
  margin-bottom: 16px;
}

.message.user {
  flex-direction: row-reverse;
}

.message-avatar {
  font-size: 24px;
}

.message-content {
  max-width: 80%;
  padding: 8px 12px;
  border-radius: 8px;
  background: #f0f0f0;
}

.message.user .message-content {
  background: #1976d2;
  color: white;
}

.typing-indicator {
  color: #666;
  font-style: italic;
}

.input-area {
  display: flex;
  gap: 8px;
  padding: 12px;
  border-top: 1px solid #ddd;
  background: #f9f9f9;
}

.input-area textarea {
  flex: 1;
  padding: 8px;
  border: 1px solid #ddd;
  border-radius: 4px;
  resize: none;
  font-family: inherit;
}

.send-btn {
  padding: 8px 16px;
  background: #1976d2;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 18px;
}

.send-btn:disabled {
  background: #ccc;
  cursor: not-allowed;
}

.settings-panel {
  padding: 16px;
  background: #f5f5f5;
  border-top: 1px solid #ddd;
}

.setting {
  margin-bottom: 16px;
}

.setting label {
  display: block;
  margin-bottom: 4px;
  font-weight: 500;
}

.setting input[type="text"],
.setting select {
  width: 100%;
  padding: 8px;
  border: 1px solid #ddd;
  border-radius: 4px;
}

.setting input[type="range"] {
  width: 100%;
}

.setting-actions {
  display: flex;
  gap: 8px;
  margin-top: 16px;
}

.setting-actions button {
  padding: 8px 16px;
  border: none;
  border-radius: 4px;
  cursor: pointer;
}

.setting-actions .primary {
  background: #1976d2;
  color: white;
}

.setting-actions .secondary {
  background: #e0e0e0;
}
</style>
