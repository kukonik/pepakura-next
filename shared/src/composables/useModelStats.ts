import { ref } from 'vue';

// Тип для статистики модели
interface ModelStats {
  parts: number;
  faces: number;
}

// Тип для текущей модели (можно расширить)
interface CurrentModel extends ModelStats {}

// Состояния
const currentModel = ref<CurrentModel | null>(null);

// Функция обновления статистики модели
function updateModelStats(parts: number, faces: number): void {
  currentModel.value = { parts, faces };
  // В реальном приложении обновляется DOM через реактивность Vue
  // const statsEl = document.getElementById('modelStats');
  // if (statsEl) {
  //   statsEl.innerHTML = <div>Деталей: <strong>\</strong></div><div>Граней: <strong>\</strong></div>;
  // }
  // const detailCountEl = document.getElementById('detailCount');
  // if (detailCountEl) {
  //   detailCountEl.textContent = Деталей: \;
  // }
  console.log('Модель обновлена:', { parts, faces });
}

export function useModelStats() {
  return {
    currentModel,
    updateModelStats,
  };
}
