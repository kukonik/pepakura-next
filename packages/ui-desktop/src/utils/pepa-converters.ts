import * as THREE from 'three'
/**
 * Пример парсера для будущих форматов (PDO, FBX, GLTF)
 * Показывает, как конвертировать любой формат в унифицированный PepaScene
 */

import type { PepaScene, PepaMesh, PepaMaterial } from '@/types/pepa-types'

// ============================================================================
// Вспомогательная функция: конвертация THREE.Mesh в PepaMesh
// ============================================================================
function meshFromThreeMesh(mesh: THREE.Mesh, materialId: string | undefined, index: number): PepaMesh {
  const geom = mesh.geometry as THREE.BufferGeometry
  const posAttr = geom.getAttribute('position')
  const normAttr = geom.getAttribute('normal')
  const uvAttr = geom.getAttribute('uv')
  const indexAttr = geom.getIndex()

  const vertices = new Float32Array(posAttr.array as ArrayLike<number>)
  const indices = indexAttr
    ? new Uint32Array(indexAttr.array as ArrayLike<number>)
    : Uint32Array.from(Array(vertices.length / 3).keys())

  const pepaMesh: PepaMesh = {
    id: mesh.name || 'mesh_' + index,
    name: mesh.name || 'Mesh ' + index,
    topology: 'triangle',
    vertices,
    indices,
    materialId,
    castShadow: mesh.castShadow,
    receiveShadow: mesh.receiveShadow
  }

  if (normAttr) {
    pepaMesh.normals = new Float32Array(normAttr.array as ArrayLike<number>)
  }

  if (uvAttr) {
    pepaMesh.uvs = new Float32Array(uvAttr.array as ArrayLike<number>)
  }

  return pepaMesh
}

// ============================================================================
// Вспомогательная функция: создание дефолтного материала
// ============================================================================
function defaultMaterial(id = 'default'): PepaMaterial {
  return {
    id,
    name: 'Default',
    diffuseColor: [0.8, 0.8, 0.8],
    opacity: 1
  }
}

// ============================================================================
// Конвертер OBJ → PepaScene
// ============================================================================
export function convertOBJToPepaScene(object: THREE.Object3D): PepaScene {
  const meshes: PepaMesh[] = []
  const materialsMap = new Map<string, PepaMaterial>()

  let meshIndex = 0

  object.traverse((child) => {
    if ((child as any).isMesh) {
      const mesh = child as THREE.Mesh
      let materialId: string | undefined

      const mat = mesh.material as THREE.Material | THREE.Material[]

      if (Array.isArray(mat)) {
        // Multi-material mesh: берём первый материал
        const m = mat[0] as any
        const id = m.uuid || 'mat_' + materialsMap.size
        materialId = id
        if (!materialsMap.has(id)) {
          const color = (m.color as THREE.Color | undefined) ?? new THREE.Color(0.8, 0.8, 0.8)
          materialsMap.set(id, {
            id,
            name: m.name || id,
            diffuseColor: [color.r, color.g, color.b],
            opacity: typeof m.opacity === 'number' ? m.opacity : 1,
            texture: m.map?.image
              ? {
                  id: m.map.uuid,
                  uri: (m.map as any).image.currentSrc || m.map.image.src || '',
                  flipY: m.map.flipY
                }
              : undefined
          })
        }
      } else if (mat) {
        const m = mat as any
        const id = m.uuid || 'mat_' + materialsMap.size
        materialId = id
        if (!materialsMap.has(id)) {
          const color = (m.color as THREE.Color | undefined) ?? new THREE.Color(0.8, 0.8, 0.8)
          materialsMap.set(id, {
            id,
            name: m.name || id,
            diffuseColor: [color.r, color.g, color.b],
            opacity: typeof m.opacity === 'number' ? m.opacity : 1,
            texture: m.map?.image
              ? {
                  id: m.map.uuid,
                  uri: (m.map as any).image.currentSrc || m.map.image.src || '',
                  flipY: m.map.flipY
                }
              : undefined
          })
        }
      }

      meshes.push(meshFromThreeMesh(mesh, materialId, meshIndex++))
    }
  })

  // Если материалов нет — добавляем дефолтный
  if (materialsMap.size === 0) {
    materialsMap.set('default', defaultMaterial('default'))
    meshes.forEach(m => { if (!m.materialId) m.materialId = 'default' })
  }

  // Вычисляем bounding box
  const box = new THREE.Box3().setFromObject(object)
  const min = box.min
  const max = box.max

  return {
    meshes,
    materials: Array.from(materialsMap.values()),
    boundingBox: {
      min: [min.x, min.y, min.z],
      max: [max.x, max.y, max.z]
    }
  }
}

// ============================================================================
// Конвертер FBX → PepaScene (использует ту же логику, что и OBJ)
// ============================================================================
export function convertFBXToPepaScene(object: THREE.Object3D): PepaScene {
  // FBX имеет ту же структуру Object3D, что и OBJ
  return convertOBJToPepaScene(object)
}

// ============================================================================
// Конвертер GLTF → PepaScene (заглушка для будущего использования)
// ============================================================================
export function convertGLTFToPepaScene(gltf: any): PepaScene {
  // gltf.scene — это THREE.Object3D
  return convertOBJToPepaScene(gltf.scene)
}

// ============================================================================
// Конвертер PDO → PepaScene (через Tauri-команду)
// ============================================================================
export async function convertPDOToPepaScene(data: ArrayBuffer): Promise<PepaScene> {
  // Вызываем Tauri-команду для парсинга PDO
  const result = await (window as any).__TAURI__.invoke('parse_pdo_to_pepa', { data: Array.from(new Uint8Array(data)) });
  
  if (!result.success) {
    throw new Error(result.error || 'Неизвестная ошибка парсинга PDO');
  }
  
  // Конвертируем данные из Rust-формата в TypeScript-формат
  const rustScene = result.scene;
  
  // Конвертируем материалы
  const materials: PepaMaterial[] = rustScene.materials.map((mat: any) => ({
    id: mat.id.toString(),
    name: mat.name,
    diffuseColor: [
      mat.diffuse_color[0],
      mat.diffuse_color[1],
      mat.diffuse_color[2]
    ],
    opacity: mat.diffuse_color[3],
    // Пока без текстур, добавим позже
  }));
  
  // Конвертируем мешы
  const meshes: PepaMesh[] = rustScene.meshes.map((mesh: any, index: number) => {
    // Конвертируем индексы из Vec<Vec<u32>> в Uint32Array
    let indices: Uint32Array;
    if (Array.isArray(mesh.indices) && mesh.indices.length > 0 && Array.isArray(mesh.indices[0])) {
      // Если это Vec<Vec<u32>>
      const flatIndices: number[] = [];
      for (const face of mesh.indices) {
        // Фан-триангуляция для каждого полигона
        for (let i = 1; i < face.length - 1; i++) {
          flatIndices.push(face[0], face[i], face[i + 1]);
        }
      }
      indices = new Uint32Array(flatIndices);
    } else if (Array.isArray(mesh.indices)) {
      // Если это уже плоский массив
      indices = new Uint32Array(mesh.indices);
    } else {
      // Если это Uint32Array (вдруг)
      indices = mesh.indices;
    }
    
    return {
      id: `mesh_${index}`,
      name: `Mesh ${index}`,
      topology: 'triangle',
      vertices: new Float32Array(mesh.positions),
      indices,
      normals: new Float32Array(mesh.normals),
      materialId: mesh.material_id?.toString() || undefined,
      castShadow: true,
      receiveShadow: true
    };
  });
  
  // Конвертируем bounding box
  let boundingBox = {
    min: [0, 0, 0] as [number, number, number],
    max: [1, 1, 1] as [number, number, number]
  };
  
  if (rustScene.bounding_box) {
    boundingBox = {
      min: [rustScene.bounding_box.min[0], rustScene.bounding_box.min[1], rustScene.bounding_box.min[2]],
      max: [rustScene.bounding_box.max[0], rustScene.bounding_box.max[1], rustScene.bounding_box.max[2]]
    };
  }
  
  return {
    meshes,
    materials,
    boundingBox
  };
}

// ============================================================================
// Пример: парсер для простого формата (например, собственный бинарный формат)
// ============================================================================
export async function parseCustomFormat( ArrayBuffer): Promise<PepaScene> {
  const view = new DataView(data)
  let offset = 0
  
  // 1. Читаем заголовок
  const magic = view.getUint32(offset)
  offset += 4
  
  if (magic !== 0x50455041) { // 'PEPA'
    throw new Error('Invalid file format')
  }
  
  // 2. Читаем количество мешей
  const meshCount = view.getUint32(offset)
  offset += 4
  
  const meshes: PepaMesh[] = []
  const materials: PepaMaterial[] = []
  
  // 3. Читаем мешы
  for (let i = 0; i < meshCount; i++) {
    // Читаем имя меша (упрощённо)
    const nameLength = view.getUint32(offset)
    offset += 4
    
    const name = new TextDecoder().decode(
      new Uint8Array(data, offset, nameLength)
    )
    offset += nameLength
    
    // Читаем вершины
    const vertexCount = view.getUint32(offset)
    offset += 4
    
    const vertices = new Float32Array(vertexCount * 3)
    for (let j = 0; j < vertexCount * 3; j++) {
      vertices[j] = view.getFloat32(offset, true)
      offset += 4
    }
    
    // Читаем индексы
    const indexCount = view.getUint32(offset)
    offset += 4
    
    const indices = new Uint32Array(indexCount)
    for (let j = 0; j < indexCount; j++) {
      indices[j] = view.getUint32(offset, true)
      offset += 4
    }
    
    // Создаём меш
    meshes.push({
      id: 'mesh_' + i,
      name,
      topology: 'triangle',
      vertices,
      indices,
      materialId: 'material_' + i
    })
    
    // Создаём материал (упрощённо)
    materials.push({
      id: 'material_' + i,
      name: 'Material ' + i,
      diffuseColor: [0.6, 0.4, 0.8],
      opacity: 1.0
    })
  }
  
  // 4. Вычисляем bounding box
  const bbox = {
    min: [0, 0, 0] as [number, number, number],
    max: [1, 1, 1] as [number, number, number]
  }
  
  return {
    meshes,
    materials,
    boundingBox: bbox
  }
}



