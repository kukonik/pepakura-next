<template>
  <div class="ai-chat-stream">
    <div class="chat-header">
      <h3>🤖 AI Чат</h3>
      <div class="chat-status">
        <span v-if="isStreaming" class="status streaming">
          <i class="fas fa-circle-notch fa-spin"></i>
          Генерация...
        </span>
        <span v-else class="status idle">
          <i class="fas fa-check-circle"></i>
          Готов
        </span>
      </div>
    </div>

    <!-- Область сообщений -->
    <div ref="messagesContainer" class="messages-container">
      <div v-for="(msg, idx) in messages" :key="idx" :class="['message', msg.role]">
        <div class="message-avatar">
          <i :class="msg.role === 'user' ? 'fas fa-user' : 'fas fa-robot'"></i>
        </div>
        <div class="message-content">
          <div class="message-text">{{ msg.content }}</div>
          <div class="message-time">{{ msg.time }}</div>
        </div>
      </div>

      <!-- Текущий стриминг ответ -->
      <div v-if="isStreaming && currentResponse" class="message assistant streaming">
        <div class="message-avatar">
          <i class="fas fa-robot"></i>
        </div>
        <div class="message-content">
          <div class="message-text">
            {{ currentResponse }}
            <span class="cursor-blink">|</span>
          </div>
          <div class="message-progress">
            <div class="progress-bar" :style="{ width: progress + '%' }"></div>
            <span class="progress-text">{{ totalTokens }} токенов</span>
          </div>
        </div>
      </div>

      <!-- Ошибка -->
      <div v-if="error" class="message error">
        <div class="message-avatar">
          <i class="fas fa-exclamation-triangle"></i>
        </div>
        <div class="message-content">
          <div class="message-text error-text">{{ error }}</div>
        </div>
      </div>

      <!-- Пустое состояние -->
      <div v-if="messages.length === 0 && !isStreaming" class="empty-state">
        <i class="fas fa-comments"></i>
        <p>Задайте вопрос AI-помощнику</p>
        <span class="hint">Например: "Как выбрать бумагу для модели?"</span>
      </div>
    </div>

    <!-- Область ввода -->
    <div class="input-area">
      <div class="input-wrapper">
        <textarea
          v-model="inputMessage"
          :disabled="isStreaming"
          placeholder="Введите вопрос..."
          @keydown.enter.exact.prevent="sendMessage"
          rows="1"
          ref="inputRef"
        ></textarea>
        <button
          class="send-btn"
          :disabled="!inputMessage.trim() || isStreaming"
          @click="sendMessage"
        >
          <i v-if="!isStreaming" class="fas fa-paper-plane"></i>
          <i v-else class="fas fa-circle-notch fa-spin"></i>
        </button>
      </div>

      <div class="input-actions">
        <button class="action-btn" @click="clearChat" title="Очистить чат">
          <i class="fas fa-trash"></i>
        </button>
        <button
          class="action-btn"
          @click="stopStreaming"
          v-if="isStreaming"
          title="Остановить"
        >
          <i class="fas fa-stop"></i>
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, nextTick, onMounted, onUnmounted } from 'vue'
import { useAiStream } from '@/composables/useAiStream'

interface Message {
  role: 'user' | 'assistant'
  content: string
  time: string
}

const inputMessage = ref('')
const messages = reactive<Message[]>([])
const messagesContainer = ref<HTMLElement>()
const inputRef = ref<HTMLTextAreaElement>()

const {
  isStreaming,
  currentResponse,
  totalTokens,
  error,
  progress,
  streamChat,
  stopStreaming,
  reset,
} = useAiStream({
  onToken: (token, total) => {
    // Авто-скролл к последнему сообщению
    scrollToBottom()
  },
  onComplete: (fullResponse, totalTokensCount) => {
    // Добавляем завершённое сообщение в историю
    messages.push({
      role: 'assistant',
      content: fullResponse,
      time: getCurrentTime(),
    })
    reset()
    scrollToBottom()
  },
  onError: (err) => {
    console.error('Stream error:', err)
  },
})

const getCurrentTime = () => {
  return new Date().toLocaleTimeString('ru-RU', {
    hour: '2-digit',
    minute: '2-digit',
  })
}

const scrollToBottom = () => {
  nextTick(() => {
    if (messagesContainer.value) {
      messagesContainer.value.scrollTop = messagesContainer.value.scrollHeight
    }
  })
}

const sendMessage = async () => {
  const text = inputMessage.value.trim()
  if (!text || isStreaming.value) return

  // Добавляем сообщение пользователя
  messages.push({
    role: 'user',
    content: text,
    time: getCurrentTime(),
  })

  const history = messages
    .filter(m => m.role !== 'assistant' || m.content !== currentResponse.value)
    .slice(-10) // Последние 10 сообщений

  inputMessage.value = ''
  await streamChat(text, history.map(m => ({ role: m.role, content: m.content })))
}

const clearChat = () => {
  if (confirm('Очистить историю чата?')) {
    messages.length = 0
    reset()
  }
}

// Авто-ресайз textarea
const adjustTextareaHeight = () => {
  if (inputRef.value) {
    inputRef.value.style.height = 'auto'
    inputRef.value.style.height = Math.min(inputRef.value.scrollHeight, 120) + 'px'
  }
}

onMounted(() => {
  adjustTextareaHeight()
})

onUnmounted(() => {
  stopStreaming()
})
</script>

<style scoped>
.ai-chat-stream {
  display: flex;
  flex-direction: column;
  height: 100%;
  background: var(--bg-secondary, #f5f5f5);
  border-radius: 8px;
  overflow: hidden;
}

.chat-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px 16px;
  background: var(--bg-primary, #fff);
  border-bottom: 1px solid var(--border-color, #eee);
}

.chat-header h3 {
  margin: 0;
  font-size: 16px;
  color: var(--text-primary, #333);
}

.chat-status {
  font-size: 13px;
}

.status {
  display: flex;
  align-items: center;
  gap: 6px;
}

.status.streaming {
  color: var(--accent-color, #4a9eff);
}

.status.idle {
  color: var(--text-secondary, #999);
}

.messages-container {
  flex: 1;
  overflow-y: auto;
  padding: 16px;
}

.message {
  display: flex;
  gap: 12px;
  margin-bottom: 16px;
  animation: fadeIn 0.3s ease;
}

@keyframes fadeIn {
  from {
    opacity: 0;
    transform: translateY(10px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.message.user {
  flex-direction: row-reverse;
}

.message.assistant.streaming .message-text {
  position: relative;
}

.cursor-blink {
  animation: blink 1s infinite;
  color: var(--accent-color, #4a9eff);
}

@keyframes blink {
  0%, 50% { opacity: 1; }
  51%, 100% { opacity: 0; }
}

.message-avatar {
  width: 36px;
  height: 36px;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
}

.message.user .message-avatar {
  background: var(--accent-light, #e8f4ff);
  color: var(--accent-color, #4a9eff);
}

.message.assistant .message-avatar {
  background: #e8f5e9;
  color: #4caf50;
}

.message.error .message-avatar {
  background: #ffebee;
  color: #f44336;
}

.message-content {
  max-width: 70%;
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.message.user .message-content {
  align-items: flex-end;
}

.message-text {
  padding: 12px 16px;
  border-radius: 12px;
  background: var(--bg-primary, #fff);
  color: var(--text-primary, #333);
  line-height: 1.5;
  word-wrap: break-word;
}

.message.user .message-text {
  background: var(--accent-color, #4a9eff);
  color: white;
}

.message-time {
  font-size: 11px;
  color: var(--text-secondary, #999);
  padding: 0 4px;
}

.message-progress {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 12px;
  color: var(--text-secondary, #999);
}

.progress-bar {
  width: 100px;
  height: 4px;
  background: var(--border-color, #eee);
  border-radius: 2px;
  overflow: hidden;
}

.progress-bar > div {
  height: 100%;
  background: var(--accent-color, #4a9eff);
  transition: width 0.2s;
}

.error-text {
  background: #ffebee;
  color: #c62828;
}

.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 100%;
  color: var(--text-secondary, #999);
  text-align: center;
}

.empty-state i {
  font-size: 48px;
  margin-bottom: 16px;
  opacity: 0.5;
}

.empty-state p {
  margin: 0 0 8px;
  font-size: 16px;
}

.empty-state .hint {
  font-size: 13px;
  opacity: 0.7;
}

.input-area {
  padding: 16px;
  background: var(--bg-primary, #fff);
  border-top: 1px solid var(--border-color, #eee);
}

.input-wrapper {
  display: flex;
  gap: 8px;
  align-items: flex-end;
}

textarea {
  flex: 1;
  padding: 12px 16px;
  border: 1px solid var(--border-color, #ddd);
  border-radius: 24px;
  resize: none;
  font-family: inherit;
  font-size: 14px;
  line-height: 1.5;
  max-height: 120px;
  background: var(--bg-secondary, #f9f9f9);
  color: var(--text-primary, #333);
}

textarea:focus {
  outline: none;
  border-color: var(--accent-color, #4a9eff);
}

textarea:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.send-btn {
  width: 44px;
  height: 44px;
  border-radius: 50%;
  border: none;
  background: var(--accent-color, #4a9eff);
  color: white;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.2s;
}

.send-btn:hover:not(:disabled) {
  background: var(--accent-hover, #3a8eef);
  transform: scale(1.05);
}

.send-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.input-actions {
  display: flex;
  gap: 8px;
  margin-top: 8px;
  justify-content: flex-end;
}

.action-btn {
  padding: 8px 12px;
  border: 1px solid var(--border-color, #ddd);
  border-radius: 6px;
  background: var(--bg-secondary, #f9f9f9);
  color: var(--text-primary, #333);
  cursor: pointer;
  font-size: 13px;
  transition: all 0.2s;
}

.action-btn:hover {
  background: var(--bg-tertiary, #e5e5e5);
}

.action-btn.danger {
  color: #c62828;
  border-color: #ffcdd2;
  background: #ffebee;
}

.action-btn.danger:hover {
  background: #ffcdd2;
}
</style>
