<template>
  <div class="viewer-3d" ref="viewerContainer">
    <!-- Toolbar -->
    <div class="viewer-toolbar">
      <div class="toolbar-group">
        <button 
          @click="resetView" 
          title="Сбросить вид"
          class="toolbar-btn"
        >
          🏠
        </button>
        <button 
          @click="toggleWireframe" 
          :title="wireframe ? 'Сплошной' : 'Каркас'"
          class="toolbar-btn"
          :class="{ active: wireframe }"
        >
          🔲
        </button>
      </div>
      
      <div class="toolbar-group">
        <button 
          @click="toggleAutoRotate" 
          :title="autoRotate ? 'Стоп' : 'Вращение'"
          class="toolbar-btn"
          :class="{ active: autoRotate }"
        >
          🔄
        </button>
        <button 
          @click="fitToScreen" 
          title="По центру"
          class="toolbar-btn"
        >
          ⛶
        </button>
      </div>
      
      <div class="toolbar-group">
        <button 
          @click="toggleSelection" 
          :title="selectionEnabled ? 'Выключение выделения' : 'Выделение граней'"
          class="toolbar-btn"
          :class="{ active: selectionEnabled }"
        >
          🎯
        </button>
      </div>
      
      <div class="toolbar-info">
        <span v-if="hoveredFace !== null">
          Грань: {{ hoveredFace + 1 }} / {{ faceCount }}
        </span>
        <span v-if="selectedFace !== null">
          Выбрано: {{ selectedFace + 1 }}
        </span>
      </div>
    </div>

    <!-- Loading indicator -->
    <div v-if="isLoading" class="viewer-loading">
      <div class="spinner"></div>
      <span>Загрузка...</span>
    </div>

    <!-- Canvas container -->
    <div class="viewer-canvas-container" @mousedown="onMouseDown" @mouseup="onMouseUp">
      <canvas ref="canvas"></canvas>
    </div>

    <!-- Help overlay -->
    <div v-if="showHelp" class="viewer-help">
      <div class="help-content">
        <h4>Управление</h4>
        <ul>
          <li>🖱️ ЛКМ + drag — вращение</li>
          <li>🖱️ ПКМ + drag — панорамирование</li>
          <li>🖱️ Колесо — масштаб</li>
          <li>🖱️ Клик — выделение грани</li>
          <li>⌨️ R — сброс вида</li>
          <li>⌨️ W — каркас</li>
          <li>⌨️ H — помощь</li>
        </ul>
        <button @click="showHelp = false">Закрыть</button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted, watch, computed } from 'vue'
import * as THREE from 'three'
import { OrbitControls } from 'three/examples/jsm/controls/OrbitControls'

// Props
interface MeshData {
  vertices: Array<[number, number, number]>
  faces: Array<[number, number, number]>
  name?: string
}

const props = defineProps<{
  mesh: MeshData | null
  unfoldedMesh?: any
  width?: number
  height?: number
}>()

// Emits
const emit = defineEmits<{
  faceSelect: [faceIndex: number]
  faceHover: [faceIndex: number | null]
}>()

// Refs
const viewerContainer = ref<HTMLElement | null>(null)
const canvas = ref<HTMLCanvasElement | null>(null)
const isLoading = ref(false)
const wireframe = ref(false)
const autoRotate = ref(false)
const selectionEnabled = ref(true)
const hoveredFace = ref<number | null>(null)
const selectedFace = ref<number | null>(null)
const showHelp = ref(false)

// Computed
const faceCount = computed(() => props.mesh?.faces.length ?? 0)

// Three.js variables
let scene: THREE.Scene | null = null
let camera: THREE.PerspectiveCamera | null = null
let renderer: THREE.WebGLRenderer | null = null
let controls: OrbitControls | null = null
let meshGroup: THREE.Group | null = null
let raycaster: THREE.Raycaster | null = null
let mouseVector: THREE.Vector2 | null = null
let animationId: number | null = null

// Initialize Three.js
function initThree() {
  if (!canvas.value || !viewerContainer.value) return

  // Scene
  scene = new THREE.Scene()
  scene.background = new THREE.Color(0xf5f5f5)

  // Camera
  const width = viewerContainer.value.clientWidth
  const height = viewerContainer.value.clientHeight
  camera = new THREE.PerspectiveCamera(45, width / height, 0.1, 10000)
  camera.position.set(5, 5, 5)

  // Renderer
  renderer = new THREE.WebGLRenderer({
    canvas: canvas.value,
    antialias: true,
  })
  renderer.setSize(width, height)
  renderer.setPixelRatio(window.devicePixelRatio)

  // Controls
  controls = new OrbitControls(camera, renderer.domElement)
  controls.enableDamping = true
  controls.dampingFactor = 0.05
  controls.screenSpacePanning = false

  // Lights
  const ambientLight = new THREE.AmbientLight(0xffffff, 0.6)
  scene.add(ambientLight)

  const directionalLight = new THREE.DirectionalLight(0xffffff, 0.8)
  directionalLight.position.set(5, 10, 7)
  scene.add(directionalLight)

  // Raycaster
  raycaster = new THREE.Raycaster()
  mouseVector = new THREE.Vector2()

  // Grid helper
  const gridHelper = new THREE.GridHelper(10, 10, 0x888888, 0xcccccc)
  scene.add(gridHelper)

  // Axes helper
  const axesHelper = new THREE.AxesHelper(2)
  scene.add(axesHelper)

  // Start animation loop
  animate()
}

// Create mesh from data
function createMesh(meshData: MeshData) {
  if (!scene) return

  // Remove old mesh
  if (meshGroup) {
    scene.remove(meshGroup)
  }

  meshGroup = new THREE.Group()

  // Geometry
  const geometry = new THREE.BufferGeometry()

  // Positions
  const positions: number[] = []
  const colors: number[] = []
  const faceIndices: number[] = []

  meshData.faces.forEach((face, faceIndex) => {
    const color = new THREE.Color().setHSL(
      faceIndex / meshData.faces.length,
      0.7,
      0.5
    )

    face.forEach((vertexIndex) => {
      const vertex = meshData.vertices[vertexIndex]
      positions.push(vertex[0], vertex[1], vertex[2])
      colors.push(color.r, color.g, color.b)
      faceIndices.push(faceIndex)
    })
  })

  geometry.setAttribute(
    'position',
    new THREE.Float32BufferAttribute(positions, 3)
  )
  geometry.setAttribute('color', new THREE.Float32BufferAttribute(colors, 3))
  geometry.setAttribute(
    'faceIndex',
    new THREE.Float32BufferAttribute(faceIndices, 1)
  )

  // Material
  const material = new THREE.MeshPhongMaterial({
    vertexColors: true,
    wireframe: wireframe.value,
    side: THREE.DoubleSide,
    transparent: true,
    opacity: 0.9,
  })

  // Mesh
  const mesh = new THREE.Mesh(geometry, material)
  mesh.name = 'main-mesh'
  meshGroup.add(mesh)

  // Edges
  const edgesGeometry = new THREE.EdgesGeometry(geometry)
  const edgesMaterial = new THREE.LineBasicMaterial({ color: 0x000000 })
  const edges = new THREE.LineSegments(edgesGeometry, edgesMaterial)
  meshGroup.add(edges)

  scene.add(meshGroup)

  // Center and fit to screen
  fitToScreen()
}

// Animation loop
function animate() {
  animationId = requestAnimationFrame(animate)

  if (controls && autoRotate.value) {
    controls.autoRotate = true
    controls.autoRotateSpeed = 2.0
  } else if (controls) {
    controls.autoRotate = false
  }

  controls?.update()
  renderer?.render(scene!, camera!)
}

// Fit mesh to screen
function fitToScreen() {
  if (!meshGroup || !camera || !controls) return

  const bbox = new THREE.Box3().setFromObject(meshGroup)
  const center = bbox.getCenter(new THREE.Vector3())
  const size = bbox.getSize(new THREE.Vector3())

  const maxDim = Math.max(size.x, size.y, size.z)
  const fov = camera.fov * (Math.PI / 180)
  let cameraZ = Math.abs(maxDim / 2 / Math.tan(fov / 2))
  cameraZ *= 1.5 // Zoom out a bit

  camera.position.set(center.x, center.y + cameraZ * 0.5, centerZ)
  camera.lookAt(center)
  controls.target.copy(center)
  controls.update()
}

// Reset view
function resetView() {
  fitToScreen()
  wireframe.value = false
  autoRotate.value = false
  if (meshGroup) {
    const mesh = meshGroup.getObjectByName('main-mesh') as THREE.Mesh
    if (mesh) {
      ;(mesh.material as THREE.Material).wireframe = false
    }
  }
}

// Toggle wireframe
function toggleWireframe() {
  wireframe.value = !wireframe.value
  if (meshGroup) {
    const mesh = meshGroup.getObjectByName('main-mesh') as THREE.Mesh
    if (mesh) {
      ;(mesh.material as THREE.Material).wireframe = wireframe.value
    }
  }
}

// Toggle auto rotate
function toggleAutoRotate() {
  autoRotate.value = !autoRotate.value
}

// Toggle selection
function toggleSelection() {
  selectionEnabled.value = !selectionEnabled.value
}

// Mouse events
let isDragging = false

function onMouseDown(event: MouseEvent) {
  isDragging = false
}

function onMouseUp(event: MouseEvent) {
  if (!isDragging && selectionEnabled.value && raycaster && mouseVector && meshGroup) {
    // Check if it was a click (not drag)
    const rect = canvas.value?.getBoundingClientRect()
    if (rect) {
      mouseVector.x = ((event.clientX - rect.left) / rect.width) * 2 - 1
      mouseVector.y = -((event.clientY - rect.top) / rect.height) * 2 + 1

      raycaster.setFromCamera(mouseVector, camera!)

      const intersects = raycaster.intersectObjects(meshGroup.children)
      if (intersects.length > 0) {
        const faceIndex = intersects[0].face?.materialIndex ?? 0
        selectFace(faceIndex)
      }
    }
  }
}

// Select face
function selectFace(faceIndex: number) {
  selectedFace.value = faceIndex
  emit('faceSelect', faceIndex)

  // Highlight selected face
  highlightFace(faceIndex)
}

// Highlight face
function highlightFace(faceIndex: number) {
  if (!meshGroup) return

  const mesh = meshGroup.getObjectByName('main-mesh') as THREE.Mesh
  if (!mesh) return

  const geometry = mesh.geometry as THREE.BufferGeometry
  const colors = geometry.getAttribute('color') as THREE.Float32BufferAttribute
  const faceIndices = geometry.getAttribute(
    'faceIndex'
  ) as THREE.Float32BufferAttribute

  // Reset colors
  const baseColors = colors.array.slice()

  // Highlight selected face
  for (let i = 0; i < faceIndices.count; i++) {
    if (faceIndices.getX(i) === faceIndex) {
      colors.setXYZ(i, 1, 0, 0) // Red
    }
  }

  colors.needsUpdate = true
}

// Keyboard events
function onKeyDown(event: KeyboardEvent) {
  switch (event.key.toLowerCase()) {
    case 'r':
      resetView()
      break
    case 'w':
      toggleWireframe()
      break
    case 'h':
      showHelp.value = !showHelp.value
      break
  }
}

// Watch for mesh changes
watch(
  () => props.mesh,
  (newMesh) => {
    if (newMesh) {
      isLoading.value = true
      createMesh(newMesh)
      isLoading.value = false
    }
  },
  { immediate: true }
)

// Lifecycle
onMounted(() => {
  initThree()
  window.addEventListener('keydown', onKeyDown)
  window.addEventListener('resize', onWindowResize)
})

onUnmounted(() => {
  if (animationId) {
    cancelAnimationFrame(animationId)
  }
  window.removeEventListener('keydown', onKeyDown)
  window.removeEventListener('resize', onWindowResize)
  renderer?.dispose()
})

function onWindowResize() {
  if (!viewerContainer.value || !camera || !renderer) return

  const width = viewerContainer.value.clientWidth
  const height = viewerContainer.value.clientHeight

  camera.aspect = width / height
  camera.updateProjectionMatrix()
  renderer.setSize(width, height)
}

// Expose methods
defineExpose({
  resetView,
  fitToScreen,
  toggleWireframe,
  selectFace,
})
</script>

<style scoped>
.viewer-3d {
  position: relative;
  width: 100%;
  height: 100%;
  background: #f5f5f5;
  overflow: hidden;
}

.viewer-toolbar {
  position: absolute;
  top: 10px;
  left: 10px;
  display: flex;
  gap: 8px;
  z-index: 10;
  background: rgba(255, 255, 255, 0.9);
  padding: 8px;
  border-radius: 8px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.toolbar-group {
  display: flex;
  gap: 4px;
}

.toolbar-btn {
  width: 32px;
  height: 32px;
  border: none;
  background: #f0f0f0;
  border-radius: 4px;
  cursor: pointer;
  font-size: 16px;
  transition: all 0.2s;
}

.toolbar-btn:hover {
  background: #e0e0e0;
}

.toolbar-btn.active {
  background: #1976d2;
  color: white;
}

.toolbar-info {
  display: flex;
  align-items: center;
  gap: 12px;
  padding-left: 12px;
  border-left: 1px solid #ddd;
  font-size: 12px;
  color: #666;
}

.viewer-loading {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  background: rgba(255, 255, 255, 0.9);
  z-index: 20;
}

.spinner {
  width: 40px;
  height: 40px;
  border: 4px solid #f0f0f0;
  border-top-color: #1976d2;
  border-radius: 50%;
  animation: spin 1s linear infinite;
}

@keyframes spin {
  to {
    transform: rotate(360deg);
  }
}

.viewer-canvas-container {
  width: 100%;
  height: 100%;
}

.viewer-canvas-container canvas {
  width: 100%;
  height: 100%;
  display: block;
}

.viewer-help {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 30;
}

.help-content {
  background: white;
  padding: 24px;
  border-radius: 8px;
  max-width: 400px;
}

.help-content h4 {
  margin-top: 0;
  margin-bottom: 16px;
}

.help-content ul {
  list-style: none;
  padding: 0;
  margin-bottom: 16px;
}

.help-content li {
  padding: 4px 0;
  font-size: 14px;
}

.help-content button {
  padding: 8px 16px;
  background: #1976d2;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
}
</style>
