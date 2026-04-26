<template>
  <div class="ai-message-list" ref="listRef">
    <div
      v-for="(message, index) in messages"
      :key="index"
      :class="['message-item', message.role]"
    >
      <div class="message-header">
        <span class="message-role">
          <span class="role-icon">{{ getRoleIcon(message.role) }}</span>
          {{ getRoleName(message.role) }}
        </span>
        <span class="message-time">{{ formatTime(message.timestamp) }}</span>
      </div>
      <div class="message-content" v-html="formatContent(message.content)"></div>
      <!-- Индикатор стриминга для активного сообщения -->
      <div v-if="message.isStreaming" class="streaming-indicator">
        <span class="typing-dot"></span>
        <span class="typing-dot"></span>
        <span class="typing-dot"></span>
      </div>
    </div>

    <!-- Индикатор загрузки -->
    <div v-if="isLoading" class="message-item assistant loading">
      <div class="message-header">
        <span class="message-role">
          <span class="role-icon">🤖</span>
          {{ $t('ai.assistant.roles.assistant') }}
        </span>
        <span class="message-time">...</span>
      </div>
      <div class="message-content">
        <div class="typing-indicator">
          <span></span>
          <span></span>
          <span></span>
        </div>
      </div>
    </div>

    <div v-if="messages.length === 0 && !isLoading" class="no-messages">
      <div class="no-messages-content">
        <div class="icon">💭</div>
        <p>{{ $t('ai.assistant.chat.noMessages') }}</p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, nextTick, watch, ref } from 'vue'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()

interface Message {
  role: 'user' | 'assistant' | 'system'
  content: string
  timestamp: Date
  isStreaming?: boolean
}

const props = defineProps<{
  messages: Message[]
  isLoading?: boolean
}>()

const listRef = ref<HTMLElement | null>(null)

// Автоматическая прокрутка вниз при новых сообщениях
watch(
  () => props.messages.length,
  async () => {
    await nextTick()
    if (listRef.value) {
      listRef.value.scrollTop = listRef.value.scrollHeight
    }
  }
)

function getRoleName(role: string): string {
  return t(`ai.assistant.roles.${role}`, role)
}

function getRoleIcon(role: string): string {
  const icons: Record<string, string> = {
    'user': '👤',
    'assistant': '🤖',
    'system': '⚙️'
  }
  return icons[role] || '💬'
}

function formatTime(date: Date): string {
  return date.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' })
}

// Простое форматирование контента с поддержкой Markdown
function formatContent(text: string): string {
  // Экранирование HTML
  let html = text
    .replace(/&/g, '&amp;')
    .replace(/</g, '&lt;')
    .replace(/>/g, '&gt;')

  // Жирный текст **text** или __text__
  html = html.replace(/\*\*(.+?)\*\*/g, '<strong>$1</strong>')
  html = html.replace(/__(.+?)__/g, '<strong>$1</strong>')

  // Курсив *text* или _text_
  html = html.replace(/\*(.+?)\*/g, '<em>$1</em>')
  html = html.replace(/_(.+?)_/g, '<em>$1</em>')

  // Код `code`
  html = html.replace(/`([^`]+)`/g, '<code>$1</code>')

  // Блоки кода ```code```
  html = html.replace(/```([\s\S]*?)```/g, '<pre><code>$1</code></pre>')

  // Заголовки # ## ###
  html = html.replace(/^### (.+)$/gm, '<h4>$1</h4>')
  html = html.replace(/^## (.+)$/gm, '<h3>$1</h3>')
  html = html.replace(/^# (.+)$/gm, '<h2>$1</h2>')

  // Списки - item или * item
  html = html.replace(/^[\-\*] (.+)$/gm, '<li>$1</li>')
  html = html.replace(/(<li>.*<\/li>\n?)+/g, '<ul>$&</ul>')

  // Нумерованные списки 1. item
  html = html.replace(/^\d+\. (.+)$/gm, '<li>$1</li>')

  // Переносы строк
  html = html.replace(/\n/g, '<br>')

  // Ссылки [text](url)
  html = html.replace(/\[([^\]]+)\]\(([^)]+)\)/g, '<a href="$2" target="_blank" rel="noopener">$1</a>')

  return html
}
</script>

<style scoped>
.ai-message-list {
  display: flex;
  flex-direction: column;
  gap: 16px;
  padding: 16px;
  height: 100%;
  overflow-y: auto;
  scroll-behavior: smooth;
}

.message-item {
  padding: 12px 16px;
  border-radius: 12px;
  border: 1px solid var(--border-color);
  animation: fadeIn 0.3s ease-out;
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

.message-item.user {
  background: linear-gradient(135deg, rgba(99, 102, 241, 0.1), rgba(99, 102, 241, 0.05));
  border-color: var(--primary);
  margin-left: 20px;
}

.message-item.assistant {
  background: var(--bg-primary);
  margin-right: 20px;
}

.message-item.system {
  background: rgba(245, 158, 11, 0.1);
  border-color: var(--warning);
}

.message-item.loading {
  opacity: 0.7;
}

.message-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 8px;
  font-size: 12px;
  color: var(--text-secondary);
}

.message-role {
  font-weight: 600;
  display: flex;
  align-items: center;
  gap: 6px;
}

.role-icon {
  font-size: 14px;
}

.message-time {
  font-size: 11px;
  opacity: 0.7;
}

.message-content {
  white-space: pre-wrap;
  word-break: break-word;
  line-height: 1.6;
  font-size: 14px;
}

.message-content :deep(h2),
.message-content :deep(h3),
.message-content :deep(h4) {
  margin: 12px 0 8px 0;
  color: var(--text-primary);
}

.message-content :deep(h2) {
  font-size: 18px;
}

.message-content :deep(h3) {
  font-size: 16px;
}

.message-content :deep(h4) {
  font-size: 14px;
}

.message-content :deep(ul) {
  margin: 8px 0;
  padding-left: 20px;
}

.message-content :deep(li) {
  margin-bottom: 4px;
}

.message-content :deep(code) {
  background: var(--bg-secondary);
  padding: 2px 6px;
  border-radius: 4px;
  font-family: 'Courier New', monospace;
  font-size: 13px;
  color: var(--primary);
}

.message-content :deep(pre) {
  background: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: 8px;
  padding: 12px;
  overflow-x: auto;
  margin: 8px 0;
}

.message-content :deep(pre code) {
  background: transparent;
  padding: 0;
  color: var(--text-primary);
}

.message-content :deep(a) {
  color: var(--primary);
  text-decoration: none;
}

.message-content :deep(a:hover) {
  text-decoration: underline;
}

.message-content :deep(strong) {
  color: var(--text-primary);
  font-weight: 600;
}

.message-content :deep(em) {
  font-style: italic;
}

/* Typing Indicator */
.typing-indicator {
  display: flex;
  gap: 4px;
  align-items: center;
  padding: 8px 0;
}

.typing-indicator span {
  width: 8px;
  height: 8px;
  background: var(--text-secondary);
  border-radius: 50%;
  animation: bounce 1.4s infinite ease-in-out;
}

.typing-indicator span:nth-child(1) {
  animation-delay: -0.32s;
}

.typing-indicator span:nth-child(2) {
  animation-delay: -0.16s;
}

@keyframes bounce {
  0%, 80%, 100% {
    transform: scale(0);
  }
  40% {
    transform: scale(1);
  }
}

/* No Messages */
.no-messages {
  display: flex;
  align-items: center;
  justify-content: center;
  height: 100%;
  color: var(--text-secondary);
}

.no-messages-content {
  text-align: center;
  padding: 40px;
}

.no-messages-content .icon {
  font-size: 48px;
  margin-bottom: 16px;
  opacity: 0.5;
}

.no-messages-content p {
  margin: 0;
  font-size: 14px;
  line-height: 1.6;
}

/* Scrollbar Styling */
.ai-message-list::-webkit-scrollbar {
  width: 8px;
}

.ai-message-list::-webkit-scrollbar-track {
  background: var(--bg-secondary);
}

.ai-message-list::-webkit-scrollbar-thumb {
  background: var(--border-color);
  border-radius: 4px;
}

.ai-message-list::-webkit-scrollbar-thumb:hover {
  background: var(--text-secondary);
}

/* Streaming Indicator */
.streaming-indicator {
  display: flex;
  gap: 4px;
  margin-top: 8px;
  align-items: center;
}

.typing-dot {
  width: 6px;
  height: 6px;
  background: var(--primary);
  border-radius: 50%;
  animation: streaming-bounce 1.4s infinite ease-in-out;
}

.typing-dot:nth-child(1) {
  animation-delay: -0.32s;
}

.typing-dot:nth-child(2) {
  animation-delay: -0.16s;
}

@keyframes streaming-bounce {
  0%, 80%, 100% {
    transform: scale(0.6);
    opacity: 0.5;
  }
  40% {
    transform: scale(1);
    opacity: 1;
  }
}
</style>
