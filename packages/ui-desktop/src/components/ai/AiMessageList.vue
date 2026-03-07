<template>
  <div class="ai-message-list">
    <div 
      v-for="(message, index) in messages" 
      :key="index"
      :class="['message-item', message.role]"
    >
      <div class="message-header">
        <span class="message-role">{{ getMessageRoleName(message.role) }}</span>
        <span class="message-time">{{ formatTime(message.timestamp) }}</span>
      </div>
      <div class="message-content">
        {{ message.content }}
      </div>
    </div>
    
    <div v-if="messages.length === 0" class="no-messages">
      <p>Нет сообщений</p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { defineProps } from 'vue';

// Определяем интерфейс сообщения
interface Message {
  role: string;
  content: string;
  timestamp: Date;
}

// Определяем props
const props = defineProps<{
  messages: Message[];
}>();

/**
 * Получение названия роли сообщения
 * @param role Роль сообщения
 * @returns Название роли
 */
function getMessageRoleName(role: string): string {
  switch (role) {
    case 'user': return 'Пользователь';
    case 'assistant': return 'AI Ассистент';
    case 'system': return 'Система';
    default: return role;
  }
}

/**
 * Форматирование времени
 * @param date Дата для форматирования
 * @returns Отформатированное время
 */
function formatTime(date: Date): string {
  return date.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
}
</script>

<style scoped>
.ai-message-list {
  display: flex;
  flex-direction: column;
  gap: 12px;
  padding: 16px;
  height: 100%;
  overflow-y: auto;
}

.message-item {
  padding: 12px;
  border-radius: 8px;
  border: 1px solid var(--border-color);
}

.message-item.user {
  background-color: var(--primary-light);
  border-color: var(--primary-color);
}

.message-item.assistant {
  background-color: var(--bg-primary);
}

.message-item.system {
  background-color: var(--warning-bg);
  border-color: var(--warning-border);
}

.message-header {
  display: flex;
  justify-content: space-between;
  margin-bottom: 8px;
  font-size: 12px;
  color: var(--text-secondary);
}

.message-role {
  font-weight: 600;
}

.message-content {
  white-space: pre-wrap;
  word-break: break-word;
}

.no-messages {
  display: flex;
  align-items: center;
  justify-content: center;
  height: 100%;
  color: var(--text-secondary);
}
</style>