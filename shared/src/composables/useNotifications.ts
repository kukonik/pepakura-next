import { ref } from 'vue';

// Состояния
const currentTool = ref<string>('import');
const statusMessage = ref<string>('Готов к работе. Загрузите модель для начала.');
const statusColor = ref<string>('#10b981'); // green for success/info

// Функция отображения уведомления
function showNotification(message: string, type: 'info' | 'success' | 'warning' | 'error' = 'info'): void {
  statusMessage.value = message;
  const colors: Record<string, string> = { info: '#3b82f6', success: '#10b981', warning: '#f59e0b', error: '#ef4444' };
  statusColor.value = colors[type] || colors.info;
  console.log(`${type.toUpperCase()}: ${message}`);
}

// Функция переключения инструмента
function switchTool(tool: string): void {
  currentTool.value = tool;
  showNotification(`Активирован инструмент: ${tool}`, 'info');
}

export function useNotifications() {
  return {
    currentTool,
    statusMessage,
    statusColor,
    showNotification,
    switchTool,
  };
}