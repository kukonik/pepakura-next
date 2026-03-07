/**
 * Общие типы для швов и рёбер
 */

/**
 * Точка в 3D пространстве
 */
export interface Point3D {
  /**
   * Координата X
   */
  x: number;
  
  /**
   * Координата Y
   */
  y: number;
  
  /**
   * Координата Z
   */
  z: number;
}

/**
 * Ребро (соединяет две точки)
 */
export interface Edge {
  /**
   * Первая точка ребра
   */
  vertex1: Point3D;
  
  /**
   * Вторая точка ребра
   */
  vertex2: Point3D;
}

/**
 * Набор швов
 */
export interface SeamSet {
  /**
   * Массив рёбер, представляющих швы
   */
  edges: Edge[];
}