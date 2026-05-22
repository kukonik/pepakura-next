// shared/src/composables/useModelStats.ts
import { computed, type ComputedRef } from 'vue';
import type { MeshData } from '../types/model';

interface ModelStats {
  vertices: number;
  faces: number;
  edges: number;
  volume: number;
  surfaceArea: number;
}

/**
 * Вычисление статистики модели на основе MeshData
 */
export function useModelStats(meshData: MeshData | null): ComputedRef<ModelStats> {
  return computed(() => {
    if (!meshData) {
      return {
        vertices: 0,
        faces: 0,
        edges: 0,
        volume: 0,
        surfaceArea: 0
      };
    }

    // Количество вершин
    const vertices = meshData.vertices.length / 3;

    // Количество граней (треугольников)
    const faces = meshData.triangles.length;

    // Простая оценка количества ребер (каждый треугольник имеет 3 ребра, но ребра могут быть общими)
    // Приблизительно 1.5 * количество граней
    const edges = Math.round(faces * 1.5);

    // Простая оценка площади поверхности
    let surfaceArea = 0;
    for (const triangle of meshData.triangles) {
      // Получаем координаты вершин треугольника
      const [i0, i1, i2] = triangle.vertices;
      
      // Умножаем индексы на 3, так как каждая вершина имеет 3 координаты (x, y, z)
      const v0x = meshData.vertices[i0 * 3] ?? 0;
      const v0y = meshData.vertices[i0 * 3 + 1] ?? 0;
      const v0z = meshData.vertices[i0 * 3 + 2] ?? 0;
      
      const v1x = meshData.vertices[i1 * 3] ?? 0;
      const v1y = meshData.vertices[i1 * 3 + 1] ?? 0;
      const v1z = meshData.vertices[i1 * 3 + 2] ?? 0;
      
      const v2x = meshData.vertices[i2 * 3] ?? 0;
      const v2y = meshData.vertices[i2 * 3 + 1] ?? 0;
      const v2z = meshData.vertices[i2 * 3 + 2] ?? 0;
      
      // Вычисляем векторы сторон треугольника
      const ax = v1x - v0x;
      const ay = v1y - v0y;
      const az = v1z - v0z;
      
      const bx = v2x - v0x;
      const by = v2y - v0y;
      const bz = v2z - v0z;
      
      // Вычисляем векторное произведение
      const cx = ay * bz - az * by;
      const cy = az * bx - ax * bz;
      const cz = ax * by - ay * bx;
      
      // Площадь треугольника равна половине длины векторного произведения
      const area = 0.5 * Math.sqrt(cx * cx + cy * cy + cz * cz);
      surfaceArea += area;
    }

    // Простая оценка объема (для сложных моделей это приближение)
    // Для простых форм типа куба или сферы будет близко к реальному значению
    const volume = surfaceArea / 6; // Очень грубая оценка

    return {
      vertices,
      faces,
      edges,
      surfaceArea: parseFloat(surfaceArea.toFixed(2)),
      volume: parseFloat(volume.toFixed(2))
    };
  });
}