// shared/src/types/model.ts

// Тип для цвета
export interface Color {
  r: number;
  g: number;
  b: number;
  a?: number; // Прозрачность (опционально)
}

// Тип для материала
export interface Material {
  id: number;
  name: string;
  diffuseColor: [number, number, number, number];
  textureId?: number;
}

// Тип для ограничивающей рамки
export interface BoundingBox {
  min: [number, number, number];
  max: [number, number, number];
}

// Тип для меша
export interface PepaMesh {
  positions: number[];
  indices: number[];
  normals: number[];
  uvs: number[];
  materialId?: number;
}

// Тип для сцены проекта Pepakura
export interface PepaScene {
  // Версия сцены
  sceneVersion: string;
  // Мешы сцены
  meshes: PepaMesh[];
  // Материалы сцены
  materials: Material[];
  // Ограничивающая рамка
  boundingBox?: BoundingBox;
}

// Тип для треугольника
export interface Triangle {
  vertices: [number, number, number]; // Индексы вершин
  normals?: [number, number, number]; // Индексы нормалей (опционально)
  material_id?: number;               // ID материала (опционально)
}

// Тип для данных модели
export interface MeshData {
  name: string;
  vertices: number[];      // [x, y, z, x, y, z, ...]
  normals?: number[];      // [nx, ny, nz, nx, ny, nz, ...] (опционально)
  triangles: Triangle[];   // Массив треугольников
  materials?: Material[];  // Массив материалов (опционально)
}
