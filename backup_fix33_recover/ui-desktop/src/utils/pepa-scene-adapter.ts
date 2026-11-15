/**
 * Адаптер для конвертации унифицированного формата PepaScene в объекты Three.js
 */

import * as THREE from 'three'
import type { PepaScene, PepaMesh, PepaMaterial } from '@/types/pepa-types'

// ============================================================================
// Создание материала Three.js из PepaMaterial
// ============================================================================
function createThreeMaterial(pepaMat: PepaMaterial): THREE.MeshStandardMaterial {
  const color = new THREE.Color(
    pepaMat.diffuseColor[0],
    pepaMat.diffuseColor[1],
    pepaMat.diffuseColor[2]
  )
  
  const materialOptions: any = {
    color,
    opacity: pepaMat.opacity,
    transparent: pepaMat.opacity < 1,
    roughness: 0.7,
    metalness: 0.1,
    side: THREE.DoubleSide
  }
  
  // Безопасно добавляем текстуры (только если они есть)
  if (pepaMat.texture?.uri) {
    try {
      const textureLoader = new THREE.TextureLoader()
      const map = textureLoader.load(pepaMat.texture.uri)
      map.flipY = pepaMat.texture.flipY ?? false
      materialOptions.map = map
    } catch (error) {
      console.warn(`⚠️ Ошибка загрузки текстуры для материала ${pepaMat.id}:`, error)
    }
  }
  
  if (pepaMat.normalMap?.uri) {
    try {
      const normalLoader = new THREE.TextureLoader()
      const normalMap = normalLoader.load(pepaMat.normalMap.uri)
      normalMap.flipY = pepaMat.normalMap.flipY ?? false
      materialOptions.normalMap = normalMap
    } catch (error) {
      console.warn(`⚠️ Ошибка загрузки нормальной карты для материала ${pepaMat.id}:`, error)
    }
  }
  
  return new THREE.MeshStandardMaterial(materialOptions)
}

// ============================================================================
// Создание геометрии Three.js из PepaMesh
// ============================================================================
function createThreeGeometry(pepaMesh: PepaMesh): THREE.BufferGeometry {
  const geometry = new THREE.BufferGeometry()
  
  // Позиции вершин
  geometry.setAttribute(
    'position',
    new THREE.BufferAttribute(pepaMesh.vertices, 3)
  )
  
  // Индексы
  if (pepaMesh.indices) {
    geometry.setIndex(
      new THREE.BufferAttribute(pepaMesh.indices, 1)
    )
  }
  
  // Нормали
  if (pepaMesh.normals) {
    geometry.setAttribute(
      'normal',
      new THREE.BufferAttribute(pepaMesh.normals, 3)
    )
  } else {
    geometry.computeVertexNormals()
  }
  
  // UV-координаты
  if (pepaMesh.uvs) {
    geometry.setAttribute(
      'uv',
      new THREE.BufferAttribute(pepaMesh.uvs, 2)
    )
  }
  
  // Цвета вершин (если есть)
  if (pepaMesh.colors) {
    geometry.setAttribute(
      'color',
      new THREE.BufferAttribute(pepaMesh.colors, 4)
    )
  }
  
  return geometry
}

// ============================================================================
// Создание меша Three.js из PepaMesh
// ============================================================================
function createThreeMesh(
  pepaMesh: PepaMesh,
  materialMap: Map<string, THREE.Material>,
  originalObject?: THREE.Object3D  // Оригинальный объект (для FBX)
): THREE.Mesh {
  const geometry = createThreeGeometry(pepaMesh)
  
  // Проверяем, есть ли FBX-расширения
  if (originalObject) {
    // Ищем оригинальный меш по имени
    let originalMesh: THREE.Mesh | null = null
    originalObject.traverse((child) => {
      if ((child as any).isMesh && child.name === pepaMesh.name) {
        originalMesh = child as THREE.Mesh
      }
    })
    
    if (originalMesh) {
      // Используем оригинальный материал FBX
      return new THREE.Mesh(geometry, originalMesh.material)
    }
  }
  
  // Выбираем материал из PepaScene
  let material: THREE.Material
  
  if (pepaMesh.materialId && materialMap.has(pepaMesh.materialId)) {
    material = materialMap.get(pepaMesh.materialId)!
  } else if (pepaMesh.materialIds && pepaMesh.materialIds.length > 0) {
    // Multi-material mesh (не поддерживается в базовой версии)
    material = new THREE.MeshStandardMaterial({ color: 0xcccccc })
  } else {
    // Default material
    material = new THREE.MeshStandardMaterial({ 
      color: 0x6366f1,
      roughness: 0.7,
      metalness: 0.3
    })
  }
  
  const mesh = new THREE.Mesh(geometry, material)
  mesh.name = pepaMesh.name || 'unnamed'
  mesh.castShadow = pepaMesh.castShadow ?? true
  mesh.receiveShadow = pepaMesh.receiveShadow ?? true
  
  // Применяем локальную трансформацию
  if (pepaMesh.position) {
    mesh.position.set(pepaMesh.position[0], pepaMesh.position[1], pepaMesh.position[2])
  }
  
  if (pepaMesh.rotation) {
    mesh.rotation.set(pepaMesh.rotation[0], pepaMesh.rotation[1], pepaMesh.rotation[2])
  }
  
  if (pepaMesh.scale) {
    mesh.scale.set(pepaMesh.scale[0], pepaMesh.scale[1], pepaMesh.scale[2])
  }
  
  return mesh
}

// ============================================================================
// Основная функция: конвертация всей сцены
// ============================================================================
export function createThreeObjectFromPepa(sceneData: PepaScene): THREE.Object3D {
  const group = new THREE.Group()
  
  // Проверяем, есть ли FBX-расширения
  const hasFBXExtension = sceneData.extensions?.fbx
  
  if (hasFBXExtension && (sceneData.extensions!.fbx as any).originalObject) {
    // Используем оригинальный FBX-объект для восстановления материалов
    const originalObject = (sceneData.extensions!.fbx as any).originalObject as THREE.Object3D
    
    // Создаём мешы с оригинальными материалами
    sceneData.meshes.forEach(pepaMesh => {
      const threeMesh = createThreeMesh(pepaMesh, new Map(), originalObject)
      group.add(threeMesh)
    })
  } else {
    // Обычная логика для других форматов
    const materialMap = new Map<string, THREE.Material>()
    for (const mat of sceneData.materials) {
      materialMap.set(mat.id, createThreeMaterial(mat))
    }
    
    // Создаём мешы
    for (const mesh of sceneData.meshes) {
      const threeMesh = createThreeMesh(mesh, materialMap)
      group.add(threeMesh)
    }
  }
  
  // Сохраняем метаданные в пользовательских данных
  (group as any).userData = {
    name: sceneData.name,
    boundingBox: sceneData.boundingBox,
    extensions: sceneData.extensions
  }
  
  return group
}

// ============================================================================
// Вспомогательные функции
// ============================================================================
export function getSceneBoundingBox(group: THREE.Object3D): {
  min: THREE.Vector3
  max: THREE.Vector3
} {
  const box = new THREE.Box3().setFromObject(group)
  return {
    min: box.min,
    max: box.max
  }
}

export function centerCameraOnObject(
  camera: THREE.PerspectiveCamera,
  controls: any, // OrbitControls
  object: THREE.Object3D
) {
  const box = new THREE.Box3().setFromObject(object)
  const center = box.getCenter(new THREE.Vector3())
  const size = box.getSize(new THREE.Vector3())
  
  const maxDim = Math.max(size.x, size.y, size.z)
  const fov = camera.fov * (Math.PI / 180)
  let cameraZ = Math.abs(maxDim / 2 / Math.tan(fov / 2))
  
  cameraZ *= 2.5
  
  camera.position.set(center.x, center.y + maxDim * 0.5, center.z + cameraZ)
  controls.target.copy(center)
  controls.update()
}

