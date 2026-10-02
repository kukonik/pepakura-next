<template>
  <div class="three-d-viewer-root">
    <div ref="canvasContainer" class="three-d-viewer-canvas-container"></div>

    <!-- Верхняя панель режимов: ⚙ ⓘ ⤢ -->
    <div class="three-d-viewer-overlay three-d-viewer-overlay-top-bar">
      <div class="three-d-viewer-top-bar">
        <div class="three-d-viewer-top-bar-left">
          <button
            class="three-d-viewer-top-bar-button"
            :class="{ 'three-d-viewer-top-bar-button-active': showProjectPanel }"
            @click="toggleProjectPanel"
          >
            ⚙
          </button>
          <button
            class="three-d-viewer-top-bar-button"
            :class="{ 'three-d-viewer-top-bar-button-active': showInfoPanel }"
            @click="toggleInfoPanel"
          >
            ⓘ
          </button>
        </div>
        <div class="three-d-viewer-top-bar-right">
          <button class="three-d-viewer-top-bar-button" @click="toggleFullscreenHint">
            ⤢
          </button>
        </div>
      </div>
    </div>

    <!-- Левая панель: Проекты / Режимы -->
    <div
      v-if="showProjectPanel"
      class="three-d-viewer-overlay three-d-viewer-overlay-left-panel three-d-viewer-panel"
    >
      <div class="three-d-viewer-panel-inner three-d-viewer-panel-inner-left">
        <h3 class="three-d-viewer-panel-title">
          Проекты / Режим
        </h3>
        <div class="three-d-viewer-panel-section">
          <div class="three-d-viewer-panel-section-title">
            Проекты
          </div>
          <ul class="three-d-viewer-list">
            <li class="three-d-viewer-list-item">
              Текущий 3D-проект
            </li>
            <li class="three-d-viewer-list-item three-d-viewer-list-item-disabled">
              Список проектов (WIP)
            </li>
          </ul>
        </div>
        <div class="three-d-viewer-panel-section">
          <div class="three-d-viewer-panel-section-title">
            Режимы
          </div>
          <ul class="three-d-viewer-list">
            <li class="three-d-viewer-list-item three-d-viewer-list-item-active">
              Просмотр 3D модели
            </li>
            <li class="three-d-viewer-list-item three-d-viewer-list-item-disabled">
              Редактор развёртки (WIP)
            </li>
            <li class="three-d-viewer-list-item three-d-viewer-list-item-disabled">
              Бумажная разметка (WIP)
            </li>
          </ul>
        </div>
      </div>
    </div>

    <!-- Правая панель: Инфо -->
    <div
      v-if="showInfoPanel"
      class="three-d-viewer-overlay three-d-viewer-overlay-right-panel three-d-viewer-panel"
    >
      <div class="three-d-viewer-panel-inner three-d-viewer-panel-inner-right">
        <h3 class="three-d-viewer-panel-title">
          Инфо
        </h3>
        <div class="three-d-viewer-panel-section">
          <div class="three-d-viewer-panel-section-title">
            3D модель
          </div>
          <p class="three-d-viewer-panel-text">
            Полноэкранный просмотр с базовыми действиями. Редактирование пока в разработке.
          </p>
        </div>
        <div class="three-d-viewer-panel-section">
          <div class="three-d-viewer-panel-section-title">
            Управление
          </div>
          <ul class="three-d-viewer-list">
            <li class="three-d-viewer-list-item">
              ЛКМ — вращение
            </li>
            <li class="three-d-viewer-list-item">
              Колёсико — зум
            </li>
            <li class="three-d-viewer-list-item">
              Shift + ЛКМ или СКМ — панорамирование
            </li>
            <li class="three-d-viewer-list-item">
              "Сброс камеры" — вернуть вид к центру
            </li>
          </ul>
        </div>
      </div>
    </div>

    <!-- Всплывающее уведомление по кнопке ⤢ -->
    <div
      v-if="showFullscreenHint"
      class="three-d-viewer-overlay three-d-viewer-overlay-center three-d-viewer-panel"
    >
      <div class="three-d-viewer-panel-inner three-d-viewer-panel-inner-center">
        <div class="three-d-viewer-panel-section">
          <div class="three-d-viewer-panel-section-title">
            Полноэкранный режим
          </div>
          <p class="three-d-viewer-panel-text">
            Браузерный fullscreen можно включить через F11. Внутренний layout уже отдаёт всё окно под 3D.
          </p>
        </div>
        <div class="three-d-viewer-panel-actions">
          <button class="three-d-viewer-button" @click="toggleFullscreenHint">
            Готово
          </button>
        </div>
      </div>
    </div>

    <!-- Нижняя панель загрузки OBJ/MTL + URL + сброс файла -->
    <div class="three-d-viewer-overlay three-d-viewer-overlay-bottom">
      <div class="three-d-viewer-overlay-group">
        <button class="three-d-viewer-button" @click="openFileDialog">
          OBJ/MTL
        </button>
        <button class="three-d-viewer-button" @click="clearFileSelection">
          Сброс файла
        </button>
        <button class="three-d-viewer-button" @click="toggleUrlPanel">
          URL
        </button>
        <button class="three-d-viewer-button" @click="resetCamera">
          Сброс камеры
        </button>
      </div>
    </div>

    <!-- Панель ввода URL -->
    <div
      v-if="showUrlPanel"
      class="three-d-viewer-overlay three-d-viewer-overlay-top-center three-d-viewer-panel"
    >
      <div class="three-d-viewer-panel-inner">
        <label class="three-d-viewer-label">
          OBJ URL:
          <input
            v-model="objUrl"
            type="text"
            class="three-d-viewer-input"
            placeholder="https://example.com/model.obj"
          />
        </label>
        <label class="three-d-viewer-label">
          MTL URL (опционально):
          <input
            v-model="mtlUrl"
            type="text"
            class="three-d-viewer-input"
            placeholder="https://example.com/materials.mtl"
          />
        </label>
        <div class="three-d-viewer-panel-actions">
          <button class="three-d-viewer-button" @click="loadFromUrl">
            Готово
          </button>
          <button class="three-d-viewer-button" @click="toggleUrlPanel">
            ✕
          </button>
        </div>
      </div>
    </div>

    <input
      ref="fileInput"
      type="file"
      class="three-d-viewer-file-input-hidden"
      multiple
      accept=".obj,.mtl"
      @change="onFilesSelected"
      @click="clearFileSelection"
    />
  </div>
</template>

<script setup lang="ts">
import { onMounted, onBeforeUnmount, ref } from 'vue'
import * as THREE from 'three'
import { OrbitControls } from 'three/examples/jsm/controls/OrbitControls.js'
import { MTLLoader } from 'three/examples/jsm/loaders/MTLLoader.js'
import { OBJLoader } from 'three/examples/jsm/loaders/OBJLoader.js'

const canvasContainer = ref<HTMLDivElement | null>(null)
const fileInput = ref<HTMLInputElement | null>(null)

const showUrlPanel = ref(false)
const showProjectPanel = ref(false)
const showInfoPanel = ref(false)
const showFullscreenHint = ref(false)

const objUrl = ref('')
const mtlUrl = ref('')

let scene: THREE.Scene | null = null
let camera: THREE.PerspectiveCamera | null = null
let renderer: THREE.WebGLRenderer | null = null
let controls: OrbitControls | null = null
let currentObject: THREE.Object3D | null = null
let animationFrameId: number | null = null

const resizeObserver = new ResizeObserver(() => {
  handleResize()
})

function initScene() {
  if (!canvasContainer.value) return

  const width = canvasContainer.value.clientWidth || window.innerWidth
  const height = canvasContainer.value.clientHeight || window.innerHeight

  scene = new THREE.Scene()
  scene.background = new THREE.Color(0x202020)

  camera = new THREE.PerspectiveCamera(45, width / height, 0.1, 1000)
  camera.position.set(5, 5, 5)

  renderer = new THREE.WebGLRenderer({ antialias: true })
  renderer.setPixelRatio(window.devicePixelRatio)
  renderer.setSize(width, height)
  renderer.outputEncoding = THREE.sRGBEncoding

  const container = canvasContainer.value
  container.innerHTML = ''
  container.appendChild(renderer.domElement)

  controls = new OrbitControls(camera, renderer.domElement)
  controls.enableDamping = true
  controls.target.set(0, 0, 0)

  const gridHelper = new THREE.GridHelper(10, 10)
  scene.add(gridHelper)

  const axesHelper = new THREE.AxesHelper(3)
  scene.add(axesHelper)

  const ambientLight = new THREE.AmbientLight(0xffffff, 0.6)
  scene.add(ambientLight)

  const directionalLight = new THREE.DirectionalLight(0xffffff, 0.8)
  directionalLight.position.set(10, 10, 10)
  scene.add(directionalLight)

  startRenderingLoop()
  handleResize()
}

function startRenderingLoop() {
  if (!renderer || !scene || !camera || !controls) return

  const renderLoop = () => {
    controls!.update()
    renderer!.render(scene!, camera!)
    animationFrameId = window.requestAnimationFrame(renderLoop)
  }

  renderLoop()
}

function stopRenderingLoop() {
  if (animationFrameId !== null) {
    window.cancelAnimationFrame(animationFrameId)
    animationFrameId = null
  }
}

function handleResize() {
  if (!canvasContainer.value || !camera || !renderer) return
  const width = canvasContainer.value.clientWidth || window.innerWidth
  const height = canvasContainer.value.clientHeight || window.innerHeight
  camera.aspect = width / height
  camera.updateProjectionMatrix()
  renderer.setSize(width, height)
}

function resetCamera() {
  if (!camera || !controls) return
  camera.position.set(5, 5, 5)
  controls.target.set(0, 0, 0)
  controls.update()
}

function disposeCurrentObject() {
  if (!scene || !currentObject) return
  scene.remove(currentObject)
  currentObject.traverse((child: any) => {
    if (child.isMesh) {
      if (child.geometry) {
        child.geometry.dispose()
      }
      if (child.material) {
        const material = child.material
        if (Array.isArray(material)) {
          material.forEach((m) => disposeMaterial(m))
        } else {
          disposeMaterial(material)
        }
      }
    }
  })
  currentObject = null
}

function disposeMaterial(material: any) {
  if (!material) return
  if (material.map) material.map.dispose()
  if (material.lightMap) material.lightMap.dispose()
  if (material.bumpMap) material.bumpMap.dispose()
  if (material.normalMap) material.normalMap.dispose()
  if (material.specularMap) material.specularMap.dispose()
  if (material.envMap) material.envMap.dispose()
  material.dispose()
}

function autoscaleAndCenterObject(object: THREE.Object3D) {
  const box = new THREE.Box3().setFromObject(object)
  const size = new THREE.Vector3()
  const center = new THREE.Vector3()
  box.getSize(size)
  box.getCenter(center)

  const maxDimension = Math.max(size.x, size.y, size.z)
  if (maxDimension > 0) {
    const scaleFactor = 2 / maxDimension
    object.scale.setScalar(scaleFactor)
  }

  object.position.sub(center)

  if (controls) {
    controls.target.set(0, 0, 0)
    controls.update()
  }

  if (camera) {
    camera.position.set(3, 3, 3)
    camera.lookAt(0, 0, 0)
  }
}

function loadObjMtlFromFiles(files: FileList) {
  if (!scene) return

  let objFile: File | null = null
  let mtlFile: File | null = null

  Array.from(files).forEach((file) => {
    if (file.name.toLowerCase().endsWith('.obj')) {
      objFile = file
    } else if (file.name.toLowerCase().endsWith('.mtl')) {
      mtlFile = file
    }
  })

  if (!objFile) {
    return
  }

  const objUrlLocal = URL.createObjectURL(objFile)
  let mtlUrlLocal: string | null = null

  if (mtlFile) {
    mtlUrlLocal = URL.createObjectURL(mtlFile)
  }

  const manager = new THREE.LoadingManager()

  const mtlLoader = new MTLLoader(manager)
  const objLoader = new OBJLoader(manager)

  if (mtlUrlLocal) {
    mtlLoader.load(
      mtlUrlLocal,
      (materials) => {
        materials.preload()
        objLoader.setMaterials(materials)
        objLoader.load(
          objUrlLocal,
          (object) => {
            disposeCurrentObject()
            currentObject = object
            scene!.add(object)
            autoscaleAndCenterObject(object)
          },
          undefined,
          (error) => {
            console.error('Ошибка загрузки OBJ с MTL из файлов', error)
          }
        )
      },
      undefined,
      (error) => {
        console.error('Ошибка загрузки MTL из файлов', error)
        objLoader.load(
          objUrlLocal,
          (object) => {
            disposeCurrentObject()
            currentObject = object
            scene!.add(object)
            autoscaleAndCenterObject(object)
          },
          undefined,
          (err) => {
            console.error('Ошибка загрузки OBJ без MTL из файлов', err)
          }
        )
      }
    )
  } else {
    objLoader.load(
      objUrlLocal,
      (object) => {
        disposeCurrentObject()
        currentObject = object
        scene!.add(object)
        autoscaleAndCenterObject(object)
      },
      undefined,
      (error) => {
        console.error('Ошибка загрузки OBJ из файлов (без MTL)', error)
      }
    )
  }
}

function loadFromUrl() {
  if (!objUrl.value) return
  if (!scene) return

  const manager = new THREE.LoadingManager()

  const mtlLoader = new MTLLoader(manager)
  const objLoader = new OBJLoader(manager)

  if (mtlUrl.value) {
    mtlLoader.load(
      mtlUrl.value,
      (materials) => {
        materials.preload()
        objLoader.setMaterials(materials)
        objLoader.load(
          objUrl.value,
          (object) => {
            disposeCurrentObject()
            currentObject = object
            scene!.add(object)
            autoscaleAndCenterObject(object)
          },
          undefined,
          (error) => {
            console.error('Ошибка загрузки OBJ с MTL по URL', error)
          }
        )
      },
      undefined,
      (error) => {
        console.error('Ошибка загрузки MTL по URL', error)
        objLoader.load(
          objUrl.value,
          (object) => {
            disposeCurrentObject()
            currentObject = object
            scene!.add(object)
            autoscaleAndCenterObject(object)
          },
          undefined,
          (err) => {
            console.error('Ошибка загрузки OBJ без MTL по URL', err)
          }
        )
      }
    )
  } else {
    objLoader.load(
      objUrl.value,
      (object) => {
        disposeCurrentObject()
        currentObject = object
        scene!.add(object)
        autoscaleAndCenterObject(object)
      },
      undefined,
      (error) => {
        console.error('Ошибка загрузки OBJ по URL (без MTL)', error)
      }
    )
  }
}

function openFileDialog() {
  if (!fileInput.value) return
  // Очистим значение перед открытием, чтобы выбор того же файла гарантированно дал change
  fileInput.value.value = ''
  fileInput.value.click()
}

function clearFileSelection() {
  if (!fileInput.value) return
  fileInput.value.value = ''
  // Уберём текущий объект со сцены, чтобы реально был «сброс»
  disposeCurrentObject()
}

function onFilesSelected(event: Event) {
  const input = event.target as HTMLInputElement
  if (!input.files || input.files.length === 0) return
  loadObjMtlFromFiles(input.files)
}

function toggleUrlPanel() {
  showUrlPanel.value = !showUrlPanel.value
}

function toggleProjectPanel() {
  showProjectPanel.value = !showProjectPanel.value
}

function toggleInfoPanel() {
  showInfoPanel.value = !showInfoPanel.value
}

function toggleFullscreenHint() {
  showFullscreenHint.value = !showFullscreenHint.value
}

onMounted(() => {
  if (!canvasContainer.value) return
  initScene()
  resizeObserver.observe(canvasContainer.value)
})

onBeforeUnmount(() => {
  stopRenderingLoop()
  if (canvasContainer.value) {
    resizeObserver.unobserve(canvasContainer.value)
  }
  if (renderer) {
    renderer.dispose()
    renderer.forceContextLoss()
    renderer.domElement.remove()
    renderer = null
  }
  controls = null
  camera = null
  scene = null
})
</script>

<style scoped>
.three-d-viewer-root {
  position: relative;
  width: 100%;
  height: 100%;
  overflow: hidden;
}

.three-d-viewer-canvas-container {
  position: absolute;
  inset: 0;
  width: 100%;
  height: 100%;
}

/* Общие оверлеи */

.three-d-viewer-overlay {
  position: absolute;
  z-index: 10;
  pointer-events: none;
}

.three-d-viewer-overlay-top-bar {
  top: 8px;
  left: 8px;
  right: 8px;
}

.three-д-viewер-overlay-top-center {
  top: 56px;
  left: 50%;
  transform: translateX(-50%);
}

.three-d-viewer-overlay-left-panel {
  top: 64px;
  bottom: 16px;
  left: 16px;
}

.three-d-viewer-overlay-right-panel {
  top: 64px;
  bottom: 16px;
  right: 16px;
}

.three-d-viewer-overlay-center {
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
}

.three-d-viewer-overlay-bottom {
  left: 16px;
  bottom: 16px;
}

.three-d-viewer-overlay-group {
  display: flex;
  gap: 8px;
  pointer-events: auto;
}

/* Верхняя панель */

.three-d-viewer-top-bar {
  display: flex;
  justify-content: space-between;
  align-items: center;
  pointer-events: auto;
  padding: 4px 8px;
  background: rgba(0, 0, 0, 0.7);
  border-radius: 6px;
  border: 1px solid rgba(255, 255, 255, 0.25);
}

.three-d-viewer-top-bar-left,
.three-d-viewer-top-bar-right {
  display: flex;
  gap: 4px;
}

.three-d-viewer-top-bar-button {
  padding: 2px 8px;
  font-size: 14px;
  border-radius: 4px;
  border: 1px solid transparent;
  background: transparent;
  color: #ffffff;
  cursor: pointer;
}

.three-d-viewer-top-bar-button:hover {
  background: rgba(255, 255, 255, 0.08);
}

.three-d-viewer-top-bar-button-active {
  border-color: rgba(255, 255, 255, 0.7);
  background: rgba(255, 255, 255, 0.1);
}

/* Панели */

.three-d-viewer-panel {
  pointer-events: auto;
}

.three-d-viewer-panel-inner {
  padding: 8px 10px;
  border-radius: 6px;
  background: rgba(0, 0, 0, 0.85);
  border: 1px solid rgba(255, 255, 255, 0.25);
  display: flex;
  flex-direction: column;
  gap: 6px;
  color: #ffffff;
}

.three-d-viewer-panel-inner-left {
  width: 260px;
}

.three-d-viewer-panel-inner-right {
  width: 260px;
}

.three-d-viewer-panel-inner-center {
  min-width: 280px;
}

.three-d-viewer-panel-title {
  margin: 0 0 4px 0;
  font-size: 13px;
  font-weight: 600;
}

.three-d-viewer-panel-section {
  margin-top: 4px;
}

.three-d-viewer-panel-section-title {
  font-size: 12px;
  font-weight: 500;
  margin-bottom: 2px;
}

.three-d-viewer-panel-text {
  margin: 0;
  font-size: 12px;
  line-height: 1.3;
}

/* Списки */

.three-d-viewer-list {
  list-style: none;
  padding: 0;
  margin: 0;
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.three-d-viewer-list-item {
  font-size: 12px;
}

.three-d-viewer-list-item-active {
  color: #8be88b;
}

.three-d-viewer-list-item-disabled {
  color: #777777;
}

/* Кнопки / инпуты */

.three-d-viewer-button {
  padding: 4px 8px;
  font-size: 12px;
  border-radius: 4px;
  border: 1px solid rgba(255, 255, 255, 0.25);
  background: rgba(0, 0, 0, 0.65);
  color: #ffffff;
  cursor: pointer;
  white-space: nowrap;
}

.three-d-viewer-button:hover {
  background: rgba(255, 255, 255, 0.1);
}

.three-d-viewer-file-input-hidden {
  display: none;
}

.three-d-viewer-label {
  display: flex;
  flex-direction: column;
  font-size: 12px;
  color: #ffffff;
  gap: 2px;
}

.three-d-viewer-input {
  padding: 4px 6px;
  font-size: 12px;
  border-radius: 4px;
  border: 1px solid rgba(255, 255, 255, 0.4);
  background: rgba(10, 10, 10, 0.9);
  color: #ffffff;
}

.three-d-viewer-panel-actions {
  display: flex;
  justify-content: flex-end;
  gap: 8px;
  margin-top: 4px;
}
</style>
