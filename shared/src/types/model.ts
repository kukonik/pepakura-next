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
  name?: string;
  ambient: Color;
  diffuse: Color;
  specular: Color;
  shininess: number;
  diffuse_texture?: string | null;
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
