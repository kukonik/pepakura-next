// shared/src/composables/useModelLoader.ts
import { ref } from 'vue';
import type { MeshData } from '../types/model';

// Временное хранилище для загруженной модели
const meshData = ref<MeshData | null>(null);
const isLoading = ref(false);
const error = ref<string | null>(null);



/**
 * Загрузка 3D модели по пути (для Tauri)
 */
async function loadModelFromPath(path: string): Promise<void> {
  console.log('useModelLoader: Запрошена загрузка модели по пути:', path);
  isLoading.value = true;
  error.value = null;
  
  try {
    // Здесь будет вызов Tauri команды для загрузки модели
    // Пока что используем фиктивные данные для демонстрации
    meshData.value = {
      name: 'DummyModel',
      vertices: [0, 0, 0, 1, 0, 0, 0, 1, 0], // Треугольник
      triangles: [{ vertices: [0, 1, 2] }],
      materials: [{ name: 'default', ambient: { r: 0.1, g: 0.1, b: 0.1 }, diffuse: { r: 0.8, g: 0.2, b: 0.2 }, specular: { r: 1.0, g: 1.0, b: 1.0 }, shininess: 30 }],
    };
  } catch (err) {
    const errorMessage = err instanceof Error ? err.message : 'Неизвестная ошибка загрузки';
    error.value = errorMessage;
    throw new Error(errorMessage);
  } finally {
    isLoading.value = false;
  }
}

export function useModelLoader() {
  return {
    meshData,
    isLoading,
    error,
    loadModelFromPath,
    loadModelFromFile,
  };
}
