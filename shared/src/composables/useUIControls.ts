import { useNotifications } from './useNotifications'; // Импортируем для показа уведомлений

const { showNotification } = useNotifications();

function saveProject(): void { showNotification('Сохранение проекта...', 'info'); }
function openSettings(): void { showNotification('Открытие настроек...', 'info'); }

function toggleTheme(event?: Event): void {
  document.body.classList.toggle('light-theme');
  const icon = (event?.currentTarget as HTMLElement)?.querySelector('i');
  if (document.body.classList.contains('light-theme')) {
    if (icon) icon.className = 'fas fa-sun';
    showNotification('Светлая тема', 'info');
  } else {
    if (icon) icon.className = 'fas fa-moon';
    showNotification('Тёмная тема', 'info');
  }
}

function toggleFullscreen(): void {
  const container = document.getElementById('viewerContainer');
  if (!container) {
     showNotification('Контейнер просмотра не найден.', 'error');
     return;
  }

  if (!document.fullscreenElement) {
    container.requestFullscreen().catch(err => console.error('Ошибка полноэкранного режима:', err.message));
    showNotification('Полноэкранный режим активирован', 'info');
  } else {
    document.exitFullscreen();
    showNotification('Полноэкранный режим отключен', 'info');
  }
}

function resetView(): void { showNotification('Вид 3D модели сброшен', 'info'); }

export function useUIControls() {
  return {
    saveProject,
    openSettings,
    toggleTheme,
    toggleFullscreen,
    resetView,
  };
}