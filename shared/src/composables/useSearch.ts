import { useNotifications } from './useNotifications'; // Импортируем для показа уведомлений

const { showNotification } = useNotifications();

function aiSearch(): void {
  const query = document.getElementById('searchInput')?.value;
  if (!query) {
    showNotification('Введите запрос для AI поиска', 'warning');
    return;
  }
  showNotification(`AI анализирует запрос: "${query}"`, 'info');
  setTimeout(() => {
    showNotification('AI нашел 3 подходящих шаблона.', 'success');
  }, 1800);
}

function webSearch(): void {
  const query = document.getElementById('searchInput')?.value;
  const url = query ? `https://www.google.com/search?q=${encodeURIComponent(query)}+pepakura+model` : 'https://www.google.com/search?q=pepakura+models';
  window.open(url, '_blank');
  showNotification('Открываю веб-поиск...', 'info');
}

function askAI(): void {
  showNotification('AI ассистент готов помочь!', 'info');
  document.getElementById('searchInput')?.focus();
}

function updateAITip(tip: string): void {
  const tipEl = document.getElementById('aiTip');
  if (tipEl) tipEl.textContent = tip;
}

export function useSearch() {
  return {
    aiSearch,
    webSearch,
    askAI,
    updateAITip,
  };
}



