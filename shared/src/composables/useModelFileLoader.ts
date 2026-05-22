// shared/src/composables/useModelFileLoader.ts
import { ref } from 'vue';
import * as THREE from 'three';
import { OBJLoader } from 'three/examples/jsm/loaders/OBJLoader.js';
import { STLLoader } from 'three/examples/jsm/loaders/STLLoader.js';
import { PLYLoader } from 'three/examples/jsm/loaders/PLYLoader.js';
import type { MeshData } from '../types/model';

// Состояние загрузки
const isLoading = ref(false);
const error = ref<string | null>(null);

/**
 * Преобразование THREE.BufferGeometry в MeshData
 */
function convertGeometryToMeshData(geometry: THREE.BufferGeometry, name: string = 'Model'): MeshData {
  const vertices: number[] = [];
  const triangles: { vertices: [number, number, number] }[] = [];
  
  // Получаем атрибут позиции
  const position = geometry.getAttribute('position');
  if (position) {
    // Копируем вершины
    for (let i = 0; i < position.count; i++) {
      vertices.push(
        position.getX(i),
        position.getY(i),
        position.getZ(i)
    );
    }
    
    // Получаем индексы
    if (geometry.index) {
      const indices = geometry.index.array;
      for (let i = 0; i < indices.length; i += 3) {
        triangles.push({
          vertices: [indices[i]!, indices[i + 1]!, indices[i + 2]!]
        });
      }
    } else {
      // Если индексов нет, создаем их последовательно
      for (let i = 0; i < position.count; i += 3) {
        if (i + 2 < position.count) {
          triangles.push({
            vertices: [i, i + 1, i + 2]
          });
        }
      }
    }
  }
  
  return {
    name,
    vertices,
    triangles,
    materials: [{
      id: 0,
      name: 'default',
      diffuseColor: [0.8, 0.8, 0.8, 1.0]
    }]
  };
}

/**
 * Преобразование THREE.Object3D в MeshData
 */
function convertObjectToMeshData(object: THREE.Object3D): MeshData {
  const vertices: number[] = [];
  const triangles: { vertices: [number, number, number] }[] = [];
  
  // Обходим все дочерние объекты
  object.traverse((child: THREE.Object3D) => {
    if (child instanceof THREE.Mesh) {
      const mesh = child as THREE.Mesh;
      const geometry = mesh.geometry;
      
      // Проверяем, что геометрия имеет атрибут позиции
      if (geometry.attributes.position) {
        const position = geometry.attributes.position;
        const vertexCount = position.count;
        const vertexOffset = vertices.length / 3;
        
        // Добавляем вершины
        for (let i = 0; i < vertexCount; i++) {
          vertices.push(
            position.getX(i),
            position.getY(i),
            position.getZ(i)
          );
        }
        
        // Добавляем индексы треугольников
        if (geometry.index) {
          const indices = geometry.index.array;
          for (let i = 0; i < indices.length; i += 3) {
            triangles.push({
              vertices: [
                indices[i]! + vertexOffset,
                indices[i + 1]! + vertexOffset,
                indices[i + 2]! + vertexOffset
              ]
            });
          }
        } else {
          // Если индексов нет, создаем их из порядка вершин
          for (let i = 0; i < vertexCount; i += 3) {
            if (i + 2 < vertexCount) {
              triangles.push({
                vertices: [
                  i + vertexOffset,
                  i + 1 + vertexOffset,
                  i + 2 + vertexOffset
                ]
              });
            }
          }
        }
      }
    }
  });
  
  return {
    name: object.name || 'LoadedModel',
    vertices,
    triangles,
    materials: [{
      id: 0,
      name: 'default',
      diffuseColor: [0.8, 0.8, 0.8, 1.0]
    }]
  };
}

/**
 * Загрузка модели из ArrayBuffer
 */
async function loadModelFromArrayBuffer(data: ArrayBuffer, extension: string): Promise<MeshData> {
  isLoading.value = true;
  error.value = null;
  
  try {
    switch (extension.toLowerCase()) {
      case 'obj': {
        const loader = new OBJLoader();
        const object = loader.parse(new TextDecoder().decode(data));
        return convertObjectToMeshData(object);
      }
      
      case 'stl': {
        const loader = new STLLoader();
        const geometry = loader.parse(data);
        return convertGeometryToMeshData(geometry, 'STL Model');
      }
      
      case 'ply': {
        const loader = new PLYLoader();
        const geometry = loader.parse(data);
        return convertGeometryToMeshData(geometry, 'PLY Model');
      }
      
      default:
        throw new Error(`Неподдерживаемый формат файла: ${extension}`);
    }
  } catch (err) {
    const errorMessage = err instanceof Error ? err.message : 'Неизвестная ошибка загрузки';
    error.value = errorMessage;
    throw new Error(errorMessage);
  } finally {
    isLoading.value = false;
  }
}

/**
 * Загрузка 3D модели из файла
 */
async function loadModelFromFile(file: File): Promise<MeshData> {
  const extension = file.name.split('.').pop() || '';
  const arrayBuffer = await file.arrayBuffer();
  return loadModelFromArrayBuffer(arrayBuffer, extension);
}

export function useModelFileLoader() {
  return {
    isLoading,
    error,
    loadModelFromFile,
    loadModelFromArrayBuffer,
  };
}