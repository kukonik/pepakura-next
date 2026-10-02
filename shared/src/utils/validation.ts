/**
 * Утилиты для валидации общих типов
 */

import type { Point3D, Edge, SeamSet } from '../models/SeamTypes';

/**
 * Проверка валидности точки 3D
 * @param point Точка для проверки
 * @returns true, если точка валидна, иначе false
 */
export function isValidPoint3D(point: any): point is Point3D {
  // Проверяем, что point - объект
  if (!point || typeof point !== 'object') {
    return false;
  }
  
  // Проверяем, что все координаты существуют и являются числами
  return (
    typeof point.x === 'number' &&
    typeof point.y === 'number' &&
    typeof point.z === 'number' &&
    !isNaN(point.x) &&
    !isNaN(point.y) &&
    !isNaN(point.z)
  );
}

/**
 * Проверка валидности ребра
 * @param edge Ребро для проверки
 * @returns true, если ребро валидно, иначе false
 */
export function isValidEdge(edge: any): edge is Edge {
  // Проверяем, что edge - объект
  if (!edge || typeof edge !== 'object') {
    return false;
  }
  
  // Проверяем, что обе вершины существуют и валидны
  return (
    isValidPoint3D(edge.vertex1) &&
    isValidPoint3D(edge.vertex2)
  );
}

/**
 * Проверка валидности набора швов
 * @param seamSet Набор швов для проверки
 * @returns true, если набор швов валиден, иначе false
 */
export function isValidSeamSet(seamSet: any): seamSet is SeamSet {
  // Проверяем, что seamSet - объект
  if (!seamSet || typeof seamSet !== 'object') {
    return false;
  }
  
  // Проверяем, что edges - массив
  if (!Array.isArray(seamSet.edges)) {
    return false;
  }
  
  // Проверяем, что все ребра валидны
  return seamSet.edges.every(isValidEdge);
}