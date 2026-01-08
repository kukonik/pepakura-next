<template>
  <div id='app'>
    <!-- Используем общий компонент из shared -->
    <PepakuraLayout>
      <template #center>
        <ModelViewer 
          :mesh-data="meshData" 
          @file-dropped="handleFileDrop"
        />
      </template>
    </PepakuraLayout>
  </div>
</template>

<script setup lang='ts'>
// Импортируем компоненты из shared
import PepakuraLayout from '@shared/components/PepakuraLayout.vue';
import ModelViewer from '@shared/components/ModelViewer.vue';
import { useModelLoader } from '@shared/composables/useModelLoader';
import { useNotifications } from '@shared/composables/useNotifications';
import type { MeshData } from '@shared/types/model';

// Используем composables
const { meshData, loadModel } = useModelLoader();
const { showNotification } = useNotifications();

// Обработка перетаскивания файлов
async function handleFileDrop(file: File) {
  console.log('App: Файл перетащен:', file.name, file.type);
  
  const fileName = file.name.toLowerCase();
  const is3D = fileName.endsWith('.obj') || fileName.endsWith('.stl') || fileName.endsWith('.gltf') || fileName.endsWith('.glb');
  const is2D = fileName.endsWith('.png') || fileName.endsWith('.jpg') || fileName.endsWith('.jpeg') || fileName.endsWith('.svg');
  
  if (!is3D && !is2D) {
    showNotification(`Неподдерживаемый формат файла: ${file.name}`, 'warning');
    return;
  }
  
  showNotification(`Обработка файла: ${file.name}...`, 'info');
  
  try {
    // Читаем файл
    const text = await file.text();
    
    // Для OBJ файлов - простая парсинг (упрощенная версия)
    if (fileName.endsWith('.obj')) {
      const parsedModel = parseOBJ(text, file.name);
      if (parsedModel) {
        meshData.value = parsedModel;
        showNotification(`Модель ${file.name} успешно загружена!`, 'success');
      } else {
        showNotification('Ошибка парсинга OBJ файла', 'error');
      }
    } else {
      // Для других форматов пока заглушка
      showNotification(`Формат ${file.name.split('.').pop()} будет поддерживаться в будущем`, 'info');
    }
  } catch (error) {
    console.error('Ошибка при чтении файла:', error);
    showNotification('Ошибка при чтении файла', 'error');
  }
}

// Простой парсер OBJ (базовая версия)
function parseOBJ(text: string, fileName: string): MeshData | null {
  try {
    const lines = text.split('\n');
    const vertices: number[] = [];
    const triangles: Array<{ vertices: [number, number, number] }> = [];
    const materials: Array<{ name: string; ambient: { r: number; g: number; b: number }; diffuse: { r: number; g: number; b: number }; specular: { r: number; g: number; b: number }; shininess: number }> = [];
    
    for (const line of lines) {
      const trimmed = line.trim();
      if (trimmed.startsWith('v ')) {
        // Вершина
        const parts = trimmed.split(/\s+/);
        if (parts.length >= 4) {
          vertices.push(parseFloat(parts[1]), parseFloat(parts[2]), parseFloat(parts[3]));
        }
      } else if (trimmed.startsWith('f ')) {
        // Грань (треугольник)
        const parts = trimmed.split(/\s+/);
        if (parts.length >= 4) {
          // Берем только индексы вершин (до слэша, если есть)
          const v1 = parseInt(parts[1].split('/')[0]) - 1; // OBJ индексы начинаются с 1
          const v2 = parseInt(parts[2].split('/')[0]) - 1;
          const v3 = parseInt(parts[3].split('/')[0]) - 1;
          triangles.push({ vertices: [v1, v2, v3] });
        }
      }
    }
    
    if (vertices.length === 0) {
      return null;
    }
    
    // Добавляем материал по умолчанию
    materials.push({
      name: 'default',
      ambient: { r: 0.1, g: 0.1, b: 0.1 },
      diffuse: { r: 0.8, g: 0.2, b: 0.2 },
      specular: { r: 1.0, g: 1.0, b: 1.0 },
      shininess: 30
    });
    
    return {
      name: fileName,
      vertices,
      triangles,
      materials
    };
  } catch (error) {
    console.error('Ошибка парсинга OBJ:', error);
    return null;
  }
}

// Для демонстрации, можно загрузить фиктивные данные при старте
// const dummyModel: MeshData = {
//   name: 'DummyModel',
//   vertices: [0, 0, 0, 1, 0, 0, 0, 1, 0],
//   triangles: [{ vertices: [0, 1, 2] }],
//   materials: [{ name: 'default', ambient: { r: 0.1, g: 0.1, b: 0.1 }, diffuse: { r: 0.8, g: 0.2, b: 0.2 }, specular: { r: 1.0, g: 1.0, b: 1.0 }, shininess: 30 }],
// };
// meshData.value = dummyModel;
</script>

<style>
/* Глобальные стили уже в style.css */
</style>