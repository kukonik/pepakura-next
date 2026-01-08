// shared/src/composables/useModelLoader.ts
import { ref } from 'vue';
import type { MeshData } from '../types/model';

// Временное хранилище для загруженной модели
const meshData = ref<MeshData | null>(null);

// Заглушка для функции загрузки
// В реальности она будет принимать функцию загрузки (платформо-специфичную)
async function loadModel(path: string): Promise<void> {
  console.log('useModelLoader: Запрошена загрузка модели по пути (заглушка):', path);
  // Здесь будет вызов platformSpecificLoadFunction(path)
  // И обновление meshData.value
  // Пока что просто обновим meshData на фиктивные данные для демонстрации
  meshData.value = {
    name: 'DummyModel',
    vertices: [0, 0, 0, 1, 0, 0, 0, 1, 0], // Треугольник
    triangles: [{ vertices: [0, 1, 2] }],
    materials: [{ name: 'default', ambient: { r: 0.1, g: 0.1, b: 0.1 }, diffuse: { r: 0.8, g: 0.2, b: 0.2 }, specular: { r: 1.0, g: 1.0, b: 1.0 }, shininess: 30 }],
  };
}

export function useModelLoader() {
  return {
    meshData,
    loadModel,
  };
}
