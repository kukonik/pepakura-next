<template>
  <div class="ai-assistant-panel">
    <div class="panel-header">
      <h3>AI Ассистент</h3>
      <button @click="togglePanel" class="toggle-btn">
        <i :class="isPanelOpen ? 'icon-collapse' : 'icon-expand'"></i>
      </button>
    </div>
    
    <div v-show="isPanelOpen" class="panel-content">
      <AiMessageList :messages="messages" />
      
      <div class="input-container">
        <textarea
          v-model="inputMessage"
          placeholder="Введите сообщение для AI ассистента..."
          :disabled="aiStore.isLoading"
          @keydown.enter.exact.prevent="sendMessage"
          @keydown.enter.shift.exact.prevent="inputMessage += '\n'"
        ></textarea>
        <button 
          @click="sendMessage" 
          :disabled="aiStore.isLoading || !inputMessage.trim()"
          class="send-btn"
        >
          {{ aiStore.isLoading ? 'Отправка...' : 'Отправить' }}
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { useAiStore } from '../../stores/aiStore';
import AiMessageList from './AiMessageList.vue';

// Хранилище AI
const aiStore = useAiStore();

// Состояние компонента
const isPanelOpen = ref(true);
const messages = ref<{ role: string; content: string; timestamp: Date }[]>([]);
const inputMessage = ref('');

/**
 * Переключение видимости панели
 */
function togglePanel() {
  isPanelOpen.value = !isPanelOpen.value;
}

/**
 * Отправка сообщения в AI
 */
async function sendMessage() {
  // Проверяем, что есть сообщение для отправки
  if (!inputMessage.value.trim() || aiStore.isLoading) {
    return;
  }

  try {
    // Добавляем сообщение пользователя в чат
    const userMessage = {
      role: 'user',
      content: inputMessage.value.trim(),
      timestamp: new Date(),
    };
    
    messages.value.push(userMessage);
    
    // Очищаем поле ввода
    inputMessage.value = '';
    
    // Получаем ответ от AI
    const aiResponse = await aiStore.sendMessage(userMessage.content);
    
    // Добавляем ответ AI в чат
    const aiMessage = {
      role: 'assistant',
      content: aiResponse,
      timestamp: new Date(),
    };
    
    messages.value.push(aiMessage);
  } catch (error) {
    // Обрабатываем ошибки
    console.error('Failed to send message to AI:', error);
    
    // Добавляем сообщение об ошибке в чат
    const errorMessage = {
      role: 'system',
      content: 'Ошибка при отправке сообщения: ' + (error instanceof Error ? error.message : 'Неизвестная ошибка'),
      timestamp: new Date(),
    };
    
    messages.value.push(errorMessage);
  }
}
</script>

<style scoped>
.ai-assistant-panel {
  display: flex;
  flex-direction: column;
  height: 100%;
  background-color: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: 8px;
  overflow: hidden;
}

.panel-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px 16px;
  background-color: var(--bg-primary);
  border-bottom: 1px solid var(--border-color);
}

.panel-header h3 {
  margin: 0;
  font-size: 16px;
  font-weight: 600;
}

.toggle-btn {
  padding: 4px;
  background-color: transparent;
  border: none;
  cursor: pointer;
  color: var(--text-secondary);
}

.toggle-btn:hover {
  color: var(--text-primary);
}

.panel-content {
  display: flex;
  flex-direction: column;
  height: 100%;
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

/* Иконки (имитация) */
.icon-expand::before,
.icon-collapse::before {
  content: "□";
}
</style>