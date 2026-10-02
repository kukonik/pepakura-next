/**
 * Унифицированный формат 3D-модели для Pepakura Next
 * Используется как промежуточное представление между парсерами (OBJ/FBX/GLTF/PDO) и рендерером (Three.js)
 */

// ============================================================================
// Топология меша
// ============================================================================
export type PepaTopology = 'triangle' | 'quad' | 'ngon'

// ============================================================================
// Материал
// ============================================================================
export interface PepaTexture {
  id: string
  uri: string        // blob:... или ...
  width?: number
  height?: number
  flipY?: boolean    // Для корректного отображения текстур
}

export interface PepaMaterial {
  id: string
  name: string
  diffuseColor: [number, number, number]  // RGB 0..1
  specularColor?: [number, number, number]
  ambientColor?: [number, number, number]
  emissiveColor?: [number, number, number]
  opacity: number                         // 0..1
  shininess?: number                      // 0..100
  texture?: PepaTexture
  normalMap?: PepaTexture
  roughnessMap?: PepaTexture
  metalnessMap?: PepaTexture
}

// ============================================================================
// Меш (геометрия)
// ============================================================================
export interface PepaMesh {
  id: string
  name: string
  topology: PepaTopology
  
  // Геометрия
  vertices: Float32Array      // [x, y, z, x, y, z, ...]
  indices: Uint32Array        // Индексы вершин (треугольники)
  
  // Опциональные атрибуты
  normals?: Float32Array      // [nx, ny, nz, ...]
  uvs?: Float32Array          // [u, v, u, v, ...]
  colors?: Float32Array       // [r, g, b, a, ...]
  
  // Ссылки на материалы
  materialId?: string
  materialIds?: string[]      // Для multi-material мешей
  
  // Метаданные
  visible?: boolean
  castShadow?: boolean
  receiveShadow?: boolean
  
  // Трансформация (локальная)
  position?: [number, number, number]
  rotation?: [number, number, number]
  scale?: [number, number, number]
}

// ============================================================================
// Сцена (коллекция мешей)
// ============================================================================
export interface PepaScene {
  // Основные данные
  meshes: PepaMesh[]
  materials: PepaMaterial[]
  
  // Bounding box
  boundingBox: {
    min: [number, number, number]
    max: [number, number, number]
  }
  
  // Метаданные сцены
  name?: string
  author?: string
  createdAt?: string
  unit?: 'meters' | 'centimeters' | 'millimeters' | 'inches'
  
  // Расширения для специфичных форматов (PDO, FBX и т.д.)
  extensions?: {
    // PDO-специфичные данные (будут добавлены позже)
    pdo?: {
      version?: string
      unfoldData?: any  // Данные развёртки
      glueTabs?: any[]  // Ярлыки склеивания
      foldLines?: any[] // Линии сгиба
    }
    
    // FBX-специфичные данные
    fbx?: {
      animations?: any[]
      skeletons?: any[]
    }
    
    // Другие расширения
    [key: string]: any
  }
}

// ============================================================================
// Утилиты для работы с типами
// ============================================================================
export function computeBoundingBox(meshes: PepaMesh[]): {
  min: [number, number, number]
  max: [number, number, number]
} {
  let minX = Infinity, minY = Infinity, minZ = Infinity
  let maxX = -Infinity, maxY = -Infinity, maxZ = -Infinity
  
  for (const mesh of meshes) {
    const vertices = mesh.vertices
    for (let i = 0; i < vertices.length; i += 3) {
      const x = vertices[i]
      const y = vertices[i + 1]
      const z = vertices[i + 2]
      
      minX = Math.min(minX, x)
      minY = Math.min(minY, y)
      minZ = Math.min(minZ, z)
      
      maxX = Math.max(maxX, x)
      maxY = Math.max(maxY, y)
      maxZ = Math.max(maxZ, z)
    }
  }
  
  return {
    min: [minX, minY, minZ],
    max: [maxX, maxY, maxZ]
  }
}

export function triangulateQuadIndices(indices: Uint32Array): Uint32Array {
  // Конвертирует квады в треугольники (0,1,2,3 → 0,1,2 + 0,2,3)
  const triangles = new Uint32Array(indices.length * 3 / 2)
  let triIndex = 0
  
  for (let i = 0; i < indices.length; i += 4) {
    triangles[triIndex++] = indices[i]
    triangles[triIndex++] = indices[i + 1]
    triangles[triIndex++] = indices[i + 2]
    
    triangles[triIndex++] = indices[i]
    triangles[triIndex++] = indices[i + 2]
    triangles[triIndex++] = indices[i + 3]
  }
  
  return triangles
}

export function triangulateNgonIndices(indices: Uint32Array, vertexCount: number): Uint32Array {
  // Простая триангуляция через ухо (ear clipping) для многоугольников
  // В реальном парсере нужно использовать полноценный алгоритм
  const triangles: number[] = []
  
  // Фан-триангуляция (работает только для выпуклых многоугольников)
  for (let i = 1; i < vertexCount - 1; i++) {
    triangles.push(indices[0], indices[i], indices[i + 1])
  }
  
  return new Uint32Array(triangles)
}
