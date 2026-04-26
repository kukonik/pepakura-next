/**
 * Composable для интерактивного 3D viewer.
 * 
 * Предоставляет:
 * - Orbit navigation (вращение камеры)
 * - Pan navigation (перемещение камеры)
 * - Zoom navigation (приближение)
 * - Выделение граней (face highlighting)
 * - Привязку 3D ↔ 2D (cross-highlighting)
 * - Raycasting для выбора граней
 */

import { ref, onMounted, onUnmounted, type Ref } from 'vue'
import * as THREE from 'three'
import { OrbitControls } from 'three/examples/jsm/controls/OrbitControls.js'

export interface Face3D {
  faceIndex: number
  vertices: number[][]
  normal: number[]
  center: number[]
}

export interface Mesh3DData {
  vertices: number[][]
  faces: number[][]
  normals?: number[][]
  colors?: number[][]
}

export interface UseInteractiveViewer3DOptions {
  backgroundColor?: number
  gridColor?: number
  highlightColor?: number
  enableGrid?: boolean
  enableAxes?: boolean
  autoRotate?: boolean
}

export function useInteractiveViewer3D(options: UseInteractiveViewer3DOptions = {}) {
  const containerRef = ref<HTMLElement | null>(null)
  const isReady = ref(false)
  const selectedFaceIndex = ref<number | null>(null)
  const hoveredFaceIndex = ref<number | null>(null)

  // Three.js объекты
  let scene: THREE.Scene | null = null
  let camera: THREE.PerspectiveCamera | null = null
  let renderer: THREE.WebGLRenderer | null = null
  let controls: OrbitControls | null = null
  let mesh: THREE.Mesh | null = null
  let raycaster: THREE.Raycaster | null = null
  let mouse: THREE.Vector2 | null = null
  let animationFrameId: number | null = null

  // Материалы
  let baseMaterial: THREE.MeshPhongMaterial | null = null
  let highlightMaterial: THREE.MeshPhongMaterial | null = null

  // Данные меша
  let meshData: Mesh3DData | null = null
  let faceIndicesMap: Map<number, number> | null = null

  // Настройки по умолчанию
  const config = {
    backgroundColor: options.backgroundColor ?? 0x0b1120,
    highlightColor: options.highlightColor ?? 0x4a9eff,
    gridColor: options.gridColor ?? 0x334455,
    enableGrid: options.enableGrid ?? true,
    enableAxes: options.enableAxes ?? false,
    autoRotate: options.autoRotate ?? false,
  }

  // Инициализация сцены
  const initScene = () => {
    if (!containerRef.value) return

    const rect = containerRef.value.getBoundingClientRect()
    const width = rect.width || 800
    const height = rect.height || 600

    // Сцена
    scene = new THREE.Scene()
    scene.background = new THREE.Color(config.backgroundColor)

    // Камера
    camera = new THREE.PerspectiveCamera(
      75,
      width / height,
      0.1,
      1000
    )
    camera.position.set(3, 3, 3)
    camera.lookAt(0, 0, 0)

    // Рендерер
    renderer = new THREE.WebGLRenderer({ antialias: true, alpha: true })
    renderer.setSize(width, height)
    renderer.setPixelRatio(window.devicePixelRatio)
    renderer.localClippingEnabled = true
    containerRef.value.appendChild(renderer.domElement)

    // Controls
    controls = new OrbitControls(camera, renderer.domElement)
    controls.enableDamping = true
    controls.dampingFactor = 0.05
    controls.screenSpacePanning = true
    controls.minDistance = 0.5
    controls.maxDistance = 100

    // Освещение
    const ambientLight = new THREE.AmbientLight(0xffffff, 0.6)
    scene.add(ambientLight)

    const directionalLight = new THREE.DirectionalLight(0xffffff, 0.8)
    directionalLight.position.set(5, 5, 5)
    scene.add(directionalLight)

    const backLight = new THREE.DirectionalLight(0xffffff, 0.3)
    backLight.position.set(-5, -5, -5)
    scene.add(backLight)

    // Сетка
    if (config.enableGrid) {
      const gridHelper = new THREE.GridHelper(10, 10, config.gridColor, config.gridColor)
      gridHelper.rotation.x = Math.PI / 2
      scene.add(gridHelper)
    }

    // Оси координат
    if (config.enableAxes) {
      const axesHelper = new THREE.AxesHelper(2)
      scene.add(axesHelper)
    }

    // Raycaster для выделения
    raycaster = new THREE.Raycaster()
    mouse = new THREE.Vector2()

    // Материалы
    baseMaterial = new THREE.MeshPhongMaterial({
      color: 0x888888,
      side: THREE.DoubleSide,
      flatShading: true,
      transparent: true,
      opacity: 0.9,
    })

    highlightMaterial = new THREE.MeshPhongMaterial({
      color: config.highlightColor,
      side: THREE.DoubleSide,
      flatShading: true,
      emissive: config.highlightColor,
      emissiveIntensity: 0.5,
    })

    // Обработчики событий
    setupEventListeners()

    // Запуск рендеринга
    animate()

    isReady.value = true
  }

  // Настройка обработчиков событий
  const setupEventListeners = () => {
    if (!containerRef.value || !renderer.value) return

    // Клик для выделения
    renderer.value.domElement.addEventListener('click', onMouseClick)
    renderer.value.domElement.addEventListener('mousemove', onMouseMove)

    // Изменение размера
    window.addEventListener('resize', onWindowResize)
  }

  // Обработка клика
  const onMouseClick = (event: MouseEvent) => {
    if (!mouse.value || !raycaster.value || !mesh.value) return

    updateMousePosition(event)
    raycaster.value.setFromCamera(mouse.value, camera.value)

    const intersects = raycaster.value.intersectObject(mesh.value)

    if (intersects.length > 0) {
      const face = intersects[0].face
      if (face) {
        const faceIndex = face.materialIndex ?? 0
        selectFace(faceIndex)
      }
    } else {
      deselectFace()
    }
  }

  // Обработка движения мыши (hover)
  const onMouseMove = (event: MouseEvent) => {
    if (!mouse.value || !raycaster.value || !mesh.value) return

    updateMousePosition(event)
    raycaster.value.setFromCamera(mouse.value, camera.value)

    const intersects = raycaster.value.intersectObject(mesh.value)

    if (intersects.length > 0) {
      const face = intersects[0].face
      if (face) {
        const faceIndex = face.materialIndex ?? 0
        hoveredFaceIndex.value = faceIndex
        document.body.style.cursor = 'pointer'
      }
    } else {
      hoveredFaceIndex.value = null
      document.body.style.cursor = 'default'
    }
  }

  // Обновление позиции мыши в нормализованных координатах
  const updateMousePosition = (event: MouseEvent) => {
    if (!mouse.value || !renderer.value) return

    const rect = renderer.value.domElement.getBoundingClientRect()
    mouse.value.x = ((event.clientX - rect.left) / rect.width) * 2 - 1
    mouse.value.y = -((event.clientY - rect.top) / rect.height) * 2 + 1
  }

  // Изменение размера окна
  const onWindowResize = () => {
    if (!containerRef.value || !camera.value || !renderer.value) return

    const rect = containerRef.value.getBoundingClientRect()
    camera.value.aspect = rect.width / rect.height
    camera.value.updateProjectionMatrix()
    renderer.value.setSize(rect.width, rect.height)
  }

  // Рендеринг
  const animate = () => {
    animationFrameId = requestAnimationFrame(animate)

    if (controls) controls.update()

    // Авто-вращение
    if (config.autoRotate && mesh.value) {
      mesh.value.rotation.y += 0.005
    }

    if (renderer.value && scene.value && camera.value) {
      renderer.value.render(scene.value, camera.value)
    }
  }

  // Загрузка меша
  const loadMesh = (data: Mesh3DData) => {
    if (!scene.value) return

    meshData = data
    faceIndicesMap = new Map()

    // Удаляем старый меш
    if (mesh.value) {
      scene.value.remove(mesh.value)
      mesh.value.geometry.dispose()
      if (Array.isArray(mesh.value.material)) {
        mesh.value.material.forEach(m => m.dispose())
      } else {
        mesh.value.material.dispose()
      }
    }

    // Создаём геометрию
    const geometry = new THREE.BufferGeometry()

    // Вершины
    const vertices = new Float32Array(data.vertices.flat())
    geometry.setAttribute('position', new THREE.BufferAttribute(vertices, 3))

    // Индексы граней
    const indices = data.faces.flat()
    geometry.setIndex(indices)

    // Вычисляем нормали
    geometry.computeVertexNormals()

    // Создаём меш с группами для выделения граней
    mesh = new THREE.Mesh(geometry, baseMaterial!.clone())

    // Настраиваем группы для каждой грани
    data.faces.forEach((face, idx) => {
      const start = idx * 3
      const count = 3
      geometry.addGroup(start, count, idx)
      faceIndicesMap!.set(idx, idx)
    })

    scene.value.add(mesh.value)

    // Центрируем камеру
    fitCameraToMesh()
  }

  // Подгонка камеры под меш
  const fitCameraToMesh = () => {
    if (!mesh.value || !camera.value || !controls.value) return

    const box = new THREE.Box3().setFromObject(mesh.value)
    const center = box.getCenter(new THREE.Vector3())
    const size = box.getSize(new THREE.Vector3())

    const maxDim = Math.max(size.x, size.y, size.z)
    const distance = maxDim * 2

    camera.value.position.set(distance, distance, distance)
    camera.value.lookAt(center)

    controls.value.target.copy(center)
    controls.value.update()
  }

  // Выделение грани
  const selectFace = (faceIndex: number) => {
    selectedFaceIndex.value = faceIndex

    if (!mesh.value || !baseMaterial.value) return

    // Сбрасываем все материалы
    if (Array.isArray(mesh.value.material)) {
      mesh.value.material.forEach((mat, idx) => {
        if (mat !== highlightMaterial) {
          mat.color.setHex(0x888888)
          mat.emissive?.setHex(0x000000)
        }
      })
    }

    // Выделяем выбранную грань
    const groups = mesh.value.geometry.groups
    if (groups[faceIndex]) {
      const materialIndex = groups[faceIndex].materialIndex
      
      // Создаём highlight материал для этой грани
      const highlightMat = highlightMaterial!.clone()
      mesh.value.material[materialIndex] = highlightMat
    }

    // Эмитим событие для 2D вида
    emitFaceSelected(faceIndex)
  }

  // Сброс выделения
  const deselectFace = () => {
    selectedFaceIndex.value = null

    if (!mesh.value || !baseMaterial.value) return

    // Сбрасываем все материалы к базовому
    if (Array.isArray(mesh.value.material)) {
      mesh.value.material.forEach((mat, idx) => {
        if (mat !== baseMaterial) {
          mesh.value!.material[idx] = baseMaterial!.clone()
        }
      })
    }
  }

  // Эмит события для 2D вида
  const emitFaceSelected = (faceIndex: number) => {
    window.dispatchEvent(new CustomEvent('face-selected-3d', {
      detail: { faceIndex },
    }))
  }

  // Получение информации о грани
  const getFaceInfo = (faceIndex: number): Face3D | null => {
    if (!meshData || !mesh.value) return null

    const face = meshData.faces[faceIndex]
    if (!face) return null

    const vertices = face.map(vi => meshData!.vertices[vi])
    const center = vertices.reduce(
      (acc, v) => [acc[0] + v[0] / vertices.length, acc[1] + v[1] / vertices.length, acc[2] + v[2] / vertices.length],
      [0, 0, 0]
    )

    // Вычисляем нормаль
    const v0 = new THREE.Vector3(...vertices[0])
    const v1 = new THREE.Vector3(...vertices[1])
    const v2 = new THREE.Vector3(...vertices[2])
    const normal = new THREE.Vector3()
      .crossVectors(
        new THREE.Vector3().subVectors(v1, v0),
        new THREE.Vector3().subVectors(v2, v0)
      )
      .normalize()

    return {
      faceIndex,
      vertices,
      normal: [normal.x, normal.y, normal.z],
      center,
    }
  }

  // Экспорт камеры
  const getCameraState = () => {
    if (!camera.value || !controls.value) return null

    return {
      position: camera.value.position.toArray(),
      target: controls.value.target.toArray(),
      zoom: camera.value.zoom,
    }
  }

  // Импорт камеры
  const setCameraState = (state: { position: number[], target: number[], zoom?: number }) => {
    if (!camera.value || !controls.value) return

    camera.value.position.set(...state.position)
    controls.value.target.set(...state.target)
    if (state.zoom) camera.value.zoom = state.zoom
    controls.value.update()
  }

  // Очистка
  const cleanup = () => {
    if (animationFrameId !== null) {
      cancelAnimationFrame(animationFrameId)
      animationFrameId = null
    }

    if (renderer.value) {
      if (containerRef.value && renderer.value.domElement.parentNode) {
        containerRef.value.removeChild(renderer.value.domElement)
      }
      renderer.value.dispose()
      renderer.value = null
    }

    if (scene.value) {
      scene.value.clear()
      scene.value = null
    }

    window.removeEventListener('resize', onWindowResize)

    isReady.value = false
  }

  // Публичные методы
  return {
    // Refs
    containerRef,
    isReady,
    selectedFaceIndex,
    hoveredFaceIndex,

    // Actions
    initScene,
    loadMesh,
    selectFace,
    deselectFace,
    fitCameraToMesh,
    getFaceInfo,
    getCameraState,
    setCameraState,
    cleanup,

    // Utils
    setAutoRotate: (enabled: boolean) => { config.autoRotate = enabled },
    setBackgroundColor: (color: number) => {
      config.backgroundColor = color
      if (scene.value) scene.value.background = new THREE.Color(color)
    },
  }
}
