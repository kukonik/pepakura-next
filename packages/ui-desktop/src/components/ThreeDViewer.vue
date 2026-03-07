<template>
  <div ref="mountRef" class="three-viewer"></div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import * as THREE from 'three'
import { OrbitControls } from 'three/examples/jsm/controls/OrbitControls'
import { GLTFLoader } from 'three/examples/jsm/loaders/GLTFLoader'
import { FBXLoader } from 'three/examples/jsm/loaders/FBXLoader'
import { OBJLoader } from 'three/examples/jsm/loaders/OBJLoader'
import { MTLLoader } from 'three/examples/jsm/loaders/MTLLoader'
import { ColladaLoader } from 'three/examples/jsm/loaders/ColladaLoader'
import { STLLoader } from 'three/examples/jsm/loaders/STLLoader'
import { PLYLoader } from 'three/examples/jsm/loaders/PLYLoader'

// Импорты адаптера и конвертеров
import { createThreeObjectFromPepa } from '@/utils/pepa-scene-adapter'
import { convertOBJToPepaScene, convertGLTFToPepaScene } from '@/utils/pepa-converters'

const mountRef = ref<HTMLDivElement | null>(null)

// Three.js сцена
let scene: THREE.Scene
let camera: THREE.PerspectiveCamera
let renderer: THREE.WebGLRenderer
let controls: OrbitControls
let model: THREE.Object3D | null = null

// Световые источники
let ambientLight: THREE.AmbientLight
let directionalLight: THREE.DirectionalLight
let hemisphereLight: THREE.HemisphereLight

// Инициализация сцены
const initScene = () => {
  // Сцена
  scene = new THREE.Scene()
  scene.background = new THREE.Color(0x0f172a)
  scene.fog = new THREE.Fog(0x0f172a, 20, 100)

  // Камера
  camera = new THREE.PerspectiveCamera(75, window.innerWidth / window.innerHeight, 0.1, 1000)
  camera.position.set(10, 10, 10)

  // Рендерер
  renderer = new THREE.WebGLRenderer({ antialias: true, alpha: true })
  renderer.setSize(window.innerWidth, window.innerHeight)
  renderer.setPixelRatio(Math.min(window.devicePixelRatio, 2))
  renderer.shadowMap.enabled = true

  if (mountRef.value) {
    mountRef.value.appendChild(renderer.domElement)
  }

  // Контролы (орбитальная камера)
  controls = new OrbitControls(camera, renderer.domElement)
  controls.enableDamping = true
  controls.dampingFactor = 0.05
  controls.screenSpacePanning = false
  controls.minDistance = 0.1
  controls.maxDistance = 2000

  // Освещение
  ambientLight = new THREE.AmbientLight(0xffffff, 0.5)
  scene.add(ambientLight)

  directionalLight = new THREE.DirectionalLight(0xffffff, 1.0)
  directionalLight.position.set(10, 20, 15)
  directionalLight.castShadow = true
  directionalLight.shadow.mapSize.width = 1024
  directionalLight.shadow.mapSize.height = 1024
  scene.add(directionalLight)

  hemisphereLight = new THREE.HemisphereLight(0x80deea, 0x4db6ac, 0.3)
  scene.add(hemisphereLight)

  // Сетка на полу
  const gridHelper = new THREE.GridHelper(50, 50, 0x6366f1, 0x334155)
  scene.add(gridHelper)

  // Ось координат
  const axesHelper = new THREE.AxesHelper(5)
  scene.add(axesHelper)

  // Анимация
  const animate = () => {
    requestAnimationFrame(animate)
    controls.update()
    renderer.render(scene, camera)
  }
  animate()

  // Обработчик изменения размера окна
  const handleResize = () => {
    if (!mountRef.value) return
    
    camera.aspect = mountRef.value.clientWidth / mountRef.value.clientHeight
    camera.updateProjectionMatrix()
    renderer.setSize(mountRef.value.clientWidth, mountRef.value.clientHeight)
  }

  window.addEventListener('resize', handleResize)

  onUnmounted(() => {
    window.removeEventListener('resize', handleResize)
    renderer.dispose()
    controls.dispose()
  })
}

// Загрузка модели из массива файлов (поддержка OBJ+MTL+текстуры)
const loadModel = async (files: File[]) => {
  try {
    // Создаем карту файлов: имя -> File
    const fileMap = new Map<string, File>()
    files.forEach(file => fileMap.set(file.name.toLowerCase(), file))
    
    // Находим основной файл модели (первый из поддерживаемых расширений)
    const modelFile = files.find(f => {
      const ext = f.name.split('.').pop()?.toLowerCase()
      return ['obj', 'fbx', 'gltf', 'glb', 'dae', 'stl', 'ply'].includes(ext || '')
    })
    
    if (!modelFile) {
      console.error('❌ Не найден файл модели (obj/fbx/gltf/dae)')
      return
    }
    
    const extension = modelFile.name.split('.').pop()?.toLowerCase()
    const fileUrl = URL.createObjectURL(modelFile)
    
    // Создаем менеджер загрузки для перехвата путей к текстурам
    const manager = new THREE.LoadingManager()
    
    manager.setURLModifier((url) => {
      // Извлекаем имя файла из URL (убираем параметры и путь)
      const urlObj = new URL(url, window.location.href)
      let textureName = urlObj.pathname.split('/').pop()?.toLowerCase() || ''
      
      // Убираем параметры запроса (если есть)
      textureName = textureName.split('?')[0]
      
      // Ищем файл с таким именем в карте
      const file = fileMap.get(textureName)
      if (file) {
        console.log('🖼️  Текстура найдена: ' + textureName)
        return URL.createObjectURL(file)
      }
      
      // Если не найдена, возвращаем оригинальный URL (для внешних ресурсов)
      return url
    })
    
    // Загружаем в зависимости от расширения
    switch (extension) {
      case 'obj':
        await loadOBJWithManager(fileUrl, modelFile.name, manager, fileMap)
        break
      case 'fbx':
        await loadFBXFromFile(fileUrl)
        break
      case 'gltf':
      case 'glb':
        await loadGLTFFromFile(fileUrl)
        break
      case 'dae':
        await loadDAEFromFile(fileUrl)
        break
      case 'stl':
        await loadSTLFromFile(fileUrl)
        break
      case 'ply':
        await loadPLYFromFile(fileUrl)
        break
      case 'pdo':
        await loadPDOFromFile(fileUrl)
        break
      default:
        console.warn('⚠️ Формат .' + extension + ' не поддерживается')
        URL.revokeObjectURL(fileUrl)
        return
    }
    
    // Центрирование камеры
    centerCameraOnModel()
    
    // Освобождаем URL основного файла
    URL.revokeObjectURL(fileUrl)
    
  } catch (err: any) {
    console.error('❌ Ошибка загрузки модели:', err)
  }
}

// Загрузка OBJ с поддержкой MTL и текстур через LoadingManager
const loadOBJWithManager = async (fileUrl: string, fileName: string, manager: THREE.LoadingManager, fileMap: Map<string, File>) => {
  return new Promise<void>((resolve, reject) => {
    const mtlLoader = new MTLLoader(manager)
    const objLoader = new OBJLoader(manager)
    
    // Ищем MTL файл по имени
    const mtlFileName = fileName.replace(/\.obj$/i, '.mtl')
    const mtlFile = fileMap.get(mtlFileName.toLowerCase())
    
    if (mtlFile) {
      const mtlUrl = URL.createObjectURL(mtlFile)
      
      mtlLoader.load(
        mtlUrl,
        (materials) => {
          materials.preload()
          objLoader.setMaterials(materials)
          
          objLoader.load(
            fileUrl,
            (obj) => {
              // Конвертируем в унифицированный формат
              const pepaScene = convertOBJToPepaScene(obj)
              const pepaObject = createThreeObjectFromPepa(pepaScene)
              
              cleanupScene()
              scene.add(pepaObject)
              model = pepaObject
              
              console.log('✅ OBJ модель с текстурами загружена (через PepaScene)')
              URL.revokeObjectURL(mtlUrl)
              resolve()
            },
            undefined,
            (error) => {
              console.error('❌ Ошибка загрузки OBJ:', error)
              URL.revokeObjectURL(mtlUrl)
              reject(error)
            }
          )
        },
        undefined,
        (error) => {
          console.warn('⚠️ MTL не найден или ошибка загрузки, загружаем без текстур')
          URL.revokeObjectURL(mtlUrl)
          loadOBJWithoutMTL(fileUrl, resolve, reject)
        }
      )
    } else {
      console.log('ℹ️ MTL файл не найден, загружаем без текстур')
      loadOBJWithoutMTL(fileUrl, resolve, reject)
    }
  })
}

// Загрузка OBJ без MTL (использует конвертер)
const loadOBJWithoutMTL = (fileUrl: string, resolve: () => void, reject: (error: any) => void) => {
  const loader = new OBJLoader()
  loader.load(
    fileUrl,
    (obj) => {
      // Конвертируем в унифицированный формат
      const pepaScene = convertOBJToPepaScene(obj)
      const pepaObject = createThreeObjectFromPepa(pepaScene)
      
      cleanupScene()
      scene.add(pepaObject)
      model = pepaObject
      
      console.log('✅ OBJ модель загружена без текстур (через PepaScene)')
      resolve()
    },
    undefined,
    reject
  )
}

// Загрузка FBX (НЕ использует конвертер - сохраняет оригинальные материалы)
const loadFBXFromFile = async (fileUrl: string) => {
  return new Promise<void>((resolve, reject) => {
    const loader = new FBXLoader()
    loader.load(
      fileUrl,
      (fbx) => {
        cleanupScene()
        
        // Масштабируем модель перед добавлением
        const box = new THREE.Box3().setFromObject(fbx)
        const size = box.getSize(new THREE.Vector3())
        const maxDim = Math.max(size.x, size.y, size.z)
        
        // МАКСИМАЛЬНОЕ ограничение размера модели
        const MAX_MODEL_SIZE = 10  // Максимальный размер модели (в метрах)
        if (maxDim > MAX_MODEL_SIZE) {
          const scaleFactor = MAX_MODEL_SIZE / maxDim
          fbx.scale.multiplyScalar(scaleFactor)
          console.log('📏 FBX модель масштабирована в ' + scaleFactor.toFixed(3) + ' раз (макс. ' + MAX_MODEL_SIZE + 'м)')
        }
        
        // Устанавливаем позиции и трансформации
        fbx.position.set(0, 0, 0)
        fbx.traverse((child) => {
          if ((child as THREE.Mesh).isMesh) {
            const mesh = child as THREE.Mesh
            mesh.castShadow = true
            mesh.receiveShadow = true
          }
        })
        
        scene.add(fbx)
        model = fbx
        
        console.log('✅ FBX модель загружена (оригинальные материалы)')
        resolve()
      },
      undefined,
      (error) => {
        console.error('❌ Ошибка загрузки FBX:', error)
        reject(error)
      }
    )
  })
}

// Загрузка GLTF/GLB (использует конвертер)
const loadGLTFFromFile = async (fileUrl: string) => {
  return new Promise<void>((resolve, reject) => {
    const loader = new GLTFLoader()
    loader.load(
      fileUrl,
      (gltf) => {
        // Конвертируем в унифицированный формат
        const pepaScene = convertGLTFToPepaScene(gltf)
        const pepaObject = createThreeObjectFromPepa(pepaScene)
        
        cleanupScene()
        scene.add(pepaObject)
        model = pepaObject
        
        console.log('✅ GLTF/GLB модель загружена (через PepaScene)')
        resolve()
      },
      undefined,
      (error) => {
        console.error('❌ Ошибка загрузки GLTF:', error)
        reject(error)
      }
    )
  })
}

// Загрузка DAE (Collada)
const loadDAEFromFile = async (fileUrl: string) => {
  return new Promise<void>((resolve, reject) => {
    const loader = new ColladaLoader()
    loader.load(
      fileUrl,
      (collada) => {
        cleanupScene()
        collada.scene.position.set(0, 0, 0)
        collada.scene.scale.set(1, 1, 1)
        
        collada.scene.traverse((child) => {
          if ((child as THREE.Mesh).isMesh) {
            const mesh = child as THREE.Mesh
            mesh.castShadow = true
            mesh.receiveShadow = true
          }
        })
        
        scene.add(collada.scene)
        model = collada.scene
        console.log('✅ DAE (Collada) модель загружена')
        resolve()
      },
      undefined,
      (error) => {
        console.error('❌ Ошибка загрузки DAE:', error)
        reject(error)
      }
    )
  })
}

// Загрузка STL
const loadSTLFromFile = async (fileUrl: string) => {
  return new Promise<void>((resolve, reject) => {
    const loader = new STLLoader()
    loader.load(
      fileUrl,
      (geometry) => {
        cleanupScene()
        
        // Создаём стандартный материал для STL
        const material = new THREE.MeshStandardMaterial({
          color: 0x6366f1,
          roughness: 0.7,
          metalness: 0.3,
          transparent: true,
          opacity: 0.9
        })
        
        const mesh = new THREE.Mesh(geometry, material)
        mesh.position.set(0, 0, 0)
        mesh.castShadow = true
        mesh.receiveShadow = true
        
        scene.add(mesh)
        model = mesh
        console.log('✅ STL модель загружена (геометрия)')
        resolve()
      },
      undefined,
      (error) => {
        console.error('❌ Ошибка загрузки STL:', error)
        reject(error)
      }
    )
  })
}

// Загрузка PLY

// Загрузка PDO (через Rust-парсер)
const loadPDOFromFile = async (fileUrl: string) => {
  return new Promise<void>((resolve, reject) => {
    // Загружаем файл как ArrayBuffer
    fetch(fileUrl)
      .then(response => response.arrayBuffer())
      .then(async (arrayBuffer) => {
        // Используем конвертер PDO
        const { convertPDOToPepaScene } = await import('@/utils/pepa-converters')
        const pepaScene = await convertPDOToPepaScene(arrayBuffer)
        
        // Создаём Three.js объект
        const pepaObject = createThreeObjectFromPepa(pepaScene)
        
        cleanupScene()
        scene.add(pepaObject)
        model = pepaObject
        
        console.log('✅ PDO модель загружена (через Rust-парсер)')
        resolve()
      })
      .catch(error => {
        console.error('❌ Ошибка загрузки PDO:', error)
        reject(error)
      })
  })
}

const loadPLYFromFile = async (fileUrl: string) => {
  return new Promise<void>((resolve, reject) => {
    const loader = new PLYLoader()
    loader.load(
      fileUrl,
      (geometry) => {
        cleanupScene()
        
        // Для точечных облаков используем PointsMaterial
        if (geometry.attributes.color) {
          const material = new THREE.PointsMaterial({ size: 0.01, vertexColors: true })
          const points = new THREE.Points(geometry, material)
          points.position.set(0, 0, 0)
          scene.add(points)
          model = points
          console.log('✅ PLY точечное облако загружено')
        } else {
          // Для мешей используем стандартный материал
          const material = new THREE.MeshStandardMaterial({
            color: 0x6366f1,
            roughness: 0.7,
            metalness: 0.3
          })
          const mesh = new THREE.Mesh(geometry, material)
          mesh.position.set(0, 0, 0)
          mesh.castShadow = true
          mesh.receiveShadow = true
          scene.add(mesh)
          model = mesh
          console.log('✅ PLY модель загружена')
        }
        resolve()
      },
      undefined,
      (error) => {
        console.error('❌ Ошибка загрузки PLY:', error)
        reject(error)
      }
    )
  })
}

// Центрирование камеры на модели
const centerCameraOnModel = () => {
  if (model) {
    const box = new THREE.Box3().setFromObject(model)
    const center = box.getCenter(new THREE.Vector3())
    const size = box.getSize(new THREE.Vector3())
    
    const maxDim = Math.max(size.x, size.y, size.z)
    
    // МАКСИМАЛЬНОЕ ограничение размера модели
    const MAX_MODEL_SIZE = 10  // Максимальный размер модели (в метрах)
    const scaleFactor = maxDim > MAX_MODEL_SIZE ? MAX_MODEL_SIZE / maxDim : 1
    
    if (scaleFactor !== 1) {
      model.scale.multiplyScalar(scaleFactor)
      console.log('📏 Модель масштабирована в ' + scaleFactor.toFixed(3) + ' раз (макс. ' + MAX_MODEL_SIZE + 'м)')
    }
    
    // Пересчитываем размер после масштабирования
    const scaledBox = new THREE.Box3().setFromObject(model)
    const scaledSize = scaledBox.getSize(new THREE.Vector3())
    const scaledMaxDim = Math.max(scaledSize.x, scaledSize.y, scaledSize.z)
    
    // Устанавливаем расстояние камеры для оптимального вида
    const fov = camera.fov * (Math.PI / 180)
    let cameraZ = Math.abs(scaledMaxDim / 2 / Math.tan(fov / 2))
    
    // Увеличиваем отступ, но не больше разумного предела
    cameraZ *= 3  // Увеличиваем отступ для лучшего вида
    cameraZ = Math.max(cameraZ, 5)  // Минимальное расстояние
    cameraZ = Math.min(cameraZ, 50)  // Максимальное расстояние
    
    // Устанавливаем позицию камеры
    camera.position.set(center.x, center.y + scaledMaxDim * 0.5, center.z + cameraZ)
    
    // Центрируем точку просмотра
    controls.target.copy(center)
    controls.update()
    
    console.log('🎯 Камера центрирована на модели', {
      позиция: camera.position.toArray(),
      цель: controls.target.toArray(),
      расстояние: cameraZ.toFixed(2),
      размер: scaledSize.toArray(),
      масштаб: scaleFactor.toFixed(3)
    })
  }
}

// Очистка сцены перед загрузкой новой модели
const cleanupScene = () => {
  if (model) {
    scene.remove(model)
    model.traverse((child) => {
      if ((child as THREE.Mesh).isMesh) {
        const mesh = child as THREE.Mesh
        if (mesh.geometry) {
          mesh.geometry.dispose()
        }
        if (mesh.material) {
          if (Array.isArray(mesh.material)) {
            mesh.material.forEach((material) => material.dispose())
          } else {
            mesh.material.dispose()
          }
        }
      }
    })
    model = null
  }
}

// Обновление освещения
const updateLighting = (ambient: number, directional: number) => {
  ambientLight.intensity = ambient
  directionalLight.intensity = directional
  console.log('💡 Освещение обновлено:', { ambient, directional })
}

// Установка масштаба модели
const setScale = (x: number, y: number, z: number) => {
  if (model) {
    model.scale.set(x, y, z)
    console.log('📏 Масштаб модели изменён:', { x, y, z })
    // Перецентрировать камеру
    centerCameraOnModel()
  }
}

// Экспорт методов
defineExpose({
  loadModel,
  updateLighting,
  setScale
})

// Инициализация при монтировании
onMounted(() => {
  initScene()
})
</script>

<style scoped>
.three-viewer {
  width: 100%;
  height: 100%;
  position: relative;
}
</style>


