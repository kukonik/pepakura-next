import { ref, reactive, watch } from 'vue';

// Интерфейс для ребра
export interface Edge {
  v1: number;
  v2: number;
}

// Хранилище состояния
const cutEdges = reactive(new Set<string>()); // Храним строки "v1_v2" для быстрого поиска
const edgeList = ref<Edge[]>([]); // Список для перебора

// Добавить/Удалить шов
export function useUnfoldState() {
  
  const toggleEdge = (v1: number, v2: number) => {
    // Нормализуем ключ (всегда меньший индекс первым)
    const key = v1 < v2 ? `${v1}_${v2}` : `${v2}_${v1}`;
    
    if (cutEdges.has(key)) {
      cutEdges.delete(key);
    } else {
      cutEdges.add(key);
    }
    // Обновляем список для 2D вьювера
    edgeList.value = Array.from(cutEdges).map(str => {
      const [a, b] = str.split('_').map(Number);
      return { v1: a, v2: b };
    });
  };

  const hasEdge = (v1: number, v2: number): boolean => {
    const key = v1 < v2 ? `${v1}_${v2}` : `${v2}_${v1}`;
    return cutEdges.has(key);
  };

  const clearCuts = () => {
    cutEdges.clear();
    edgeList.value = [];
  };

  const getCutsCount = () => cutEdges.size;

  return {
    cutEdges, // Set для быстрой проверки
    edgeList, // Array для 2D отрисовки
    toggleEdge,
    hasEdge,
    clearCuts,
    getCutsCount
  };
}
