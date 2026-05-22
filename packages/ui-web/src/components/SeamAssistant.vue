<template>
  <div class="seam-assistant">
    <div class="assistant-header">
      <h3>AI Assistant</h3>
      <button @click="clearChat" class="clear-btn">Очистить</button>
    </div>
    
    <div class="messages-container">
      <div 
        v-for="(message, index) in messages" 
        :key="index"
        :class="['message', message.role]"
      >
        <div class="message-content">
          {{ message.content }}
        </div>
        <div class="message-timestamp">
          {{ formatTime(message.timestamp) }}
        </div>
      </div>
    </div>
    
    <div class="input-container">
      <textarea
        v-model="inputMessage"
        placeholder="Введите сообщение для AI ассистента..."
        :disabled="isSending"
        @keydown.enter.exact.prevent="sendMessage"
        @keydown.enter.shift.exact.prevent="inputMessage += '\n'"
      ></textarea>
      <button 
        @click="sendMessage" 
        :disabled="isSending || !inputMessage.trim()"
        class="send-btn"
      >
        {{ isSending ? 'Отправка...' : 'Отправить' }}
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import type { ChatMessage } from '@pepakura-next/shared';
import { useSeamsStore } from '../stores/seams.store';
import { AiServiceClient } from '../modules/ai-service/client';

// Хранилище швов
const seamsStore = useSeamsStore();

// Состояние компонента
const messages = ref<ChatMessage[]>([]);
const inputMessage = ref('');
const isSending = ref(false);

// Клиент AI сервиса
const aiClient = new AiServiceClient();

/**
 * Отправка сообщения в AI
 */
async function sendMessage() {
  // Проверяем, что есть сообщение для отправки
  if (!inputMessage.value.trim() || isSending.value) {
    return;
  }

  // Устанавливаем флаг отправки
  isSending.value = true;

  try {
    // Добавляем сообщение пользователя в чат
    const userMessage: ChatMessage = {
      role: 'user',
      content: inputMessage.value.trim(),
      timestamp: new Date(),
    };
    
    messages.value.push(userMessage);
    
    // Очищаем поле ввода
    inputMessage.value = '';
    
    // Получаем ответ от AI
    const aiResponse = await aiClient.sendMessage(
      messages.value, 
      seamsStore.aiConfig
    );
    
    // Добавляем ответ AI в чат
    const aiMessage: ChatMessage = {
      role: 'assistant',
      content: aiResponse.content,
      timestamp: new Date(),
    };
    
    messages.value.push(aiMessage);
  } catch (error) {
    // Обрабатываем ошибки
    console.error('Failed to send message to AI:', error);
    
    // Добавляем сообщение об ошибке в чат
    const errorMessage: ChatMessage = {
      role: 'system',
      content: 'Ошибка при отправке сообщения: ' + (error instanceof Error ? error.message : 'Неизвестная ошибка'),
      timestamp: new Date(),
    };
    
    messages.value.push(errorMessage);
  } finally {
    // Сбрасываем флаг отправки
    isSending.value = false;
  }
}

/**
 * Очистка чата
 */
function clearChat() {
  messages.value = [];
}

/**
 * Форматирование времени
 */
function formatTime(date: Date): string {
  return date.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
}
</script>

<style scoped>
.seam-assistant {
  display: flex;
  flex-direction: column;
  height: 100%;
  background-color: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: 8px;
  overflow: hidden;
}

.assistant-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px 16px;
  background-color: var(--bg-primary);
  border-bottom: 1px solid var(--border-color);
}

.assistant-header h3 {
  margin: 0;
  font-size: 16px;
  font-weight: 600;
}

.clear-btn {
  padding: 6px 12px;
  background-color: var(--btn-secondary-bg);
  color: var(--btn-secondary-text);
  border: 1px solid var(--btn-secondary-border);
  border-radius: 4px;
  cursor: pointer;
  font-size: 14px;
}

.clear-btn:hover {
  background-color: var(--btn-secondary-hover);
}

.messages-container {
  flex: 1;
  overflow-y: auto;
  padding: 16px;
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.message {
  max-width: 80%;
  padding: 12px 16px;
  border-radius: 12px;
  font-size: 14px;
  line-height: 1.4;
}

.message.user {
  align-self: flex-end;
  background-color: var(--primary-color);
  color: white;
}

.message.assistant {
  align-self: flex-start;
  background-color: var(--bg-primary);
  border: 1px solid var(--border-color);
}

.message.system {
  align-self: center;
  background-color: var(--warning-bg);
  color: var(--warning-text);
  border: 1px solid var(--warning-border);
  font-size: 12px;
}

.message-content {
  white-space: pre-wrap;
  word-break: break-word;
}

.message-timestamp {
  font-size: 10px;
  opacity: 0.7;
  margin-top: 4px;
  text-align: right;
}

.input-container {
  display: flex;
  flex-direction: column;
  padding: 16px;
  border-top: 1px solid var(--border-color);
  background-color: var(--bg-primary);
}

.input-container textarea {
  width: 100%;
  min-height: 80px;
  padding: 12px;
  border: 1px solid var(--border-color);
  border-radius: 8px;
  resize: vertical;
  font-family: inherit;
  font-size: 14px;
  margin-bottom: 12px;
}

.input-container textarea:focus {
  outline: none;
  border-color: var(--primary-color);
  box-shadow: 0 0 0 2px rgba(0, 123, 255, 0.25);
}

.send-btn {
  align-self: flex-end;
  padding: 8px 16px;
  background-color: var(--primary-color);
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 14px;
  font-weight: 500;
}

.send-btn:hover:not(:disabled) {
  background-color: var(--primary-hover);
}

.send-btn:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}
</style>