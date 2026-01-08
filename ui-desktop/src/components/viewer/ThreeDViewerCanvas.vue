<template>
  <div class="three-root" ref="container">
    <canvas ref="canvas" class="three-canvas"></canvas>
    <div v-if="error" class="three-overlay error">
      <p>Ошибка загрузки 3D‑модели</p>
      <p class="msg">{{ error }}</p>
    </div>
    <div v-else-if="loading" class="three-overlay loading">
      <p>Загрузка 3D‑модели…</p>
    </div>
  </div>
</template>

<script setup lang="ts">
import {
  onMounted,
  onBeforeUnmount,
  ref,
  watch,
  nextTick
} from 'vue'
import * as THREE from 'three'
import { OBJLoader } from 'three/examples/jsm/loaders/OBJLoader.js'
import { MTLLoader } from 'three/examples/jsm/loaders/MTLLoader.js'
import { OrbitControls } from 'three/examples/jsm/controls/OrbitControls.js'

interface Props {
  modelPath: string | null
  mtlPath?: string | null
}

const props = defineProps<Props>()

const container = ref<HTMLDivElement | null>(null)
const canvas = ref<HTMLCanvasElement | null>(null)

let renderer: THREE.WebGLRenderer | null = null
let scene: THREE.Scene | null = null
let camera: THREE.PerspectiveCamera | null = null
let controls: OrbitControls | null = null
let animationId: number | null = null
let model: THREE.Object3D | null = null

const loading = ref(false)
const error = ref<string | null>(null)

function initScene() {
  if (!container.value || !canvas.value) return

  const width = container.value.clientWidth || 1
  const height = container.value.clientHeight || 1

  renderer = new THREE.WebGLRenderer({
    canvas: canvas.value,
    antialias: true
  })
  renderer.setSize(width, height)
  renderer.setPixelRatio(window.devicePixelRatio || 1)
  renderer.setClearColor(0x020617, 1)

  scene = new THREE.Scene()
  scene.background = new THREE.Color(0x020617)

  camera = new THREE.PerspectiveCamera(45, width / height, 0.1, 1000)
  camera.position.set(0, 1.5, 3)

  const light = new THREE.DirectionalLight(0xffffff, 1)
  light.position.set(3, 5, 2)
  scene.add(light)

  const ambient = new THREE.AmbientLight(0xffffff, 0.35)
  scene.add(ambient)

  const grid = new THREE.GridHelper(4, 8, 0x444444, 0x222222)
  grid.position.y = -0.5
  scene.add(grid)

  const axes = new THREE.AxesHelper(1.2)
  scene.add(axes)

  controls = new OrbitControls(camera, canvas.value)
  controls.enableDamping = true
  controls.dampingFactor = 0.08
  controls.minDistance = 0.8
  controls.maxDistance = 10
  controls.target.set(0, 0, 0)

  animate()
}

function clearModel() {
  if (scene && model) {
    scene.remove(model)
    model.traverse(obj => {
      const mesh = obj as THREE.Mesh
      if (mesh.geometry) mesh.geometry.dispose()
      if (Array.isArray(mesh.material)) {
        mesh.material.forEach(m => m.dispose())
      } else if (mesh.material) {
        mesh.material.dispose()
      }
    })
  }
  model = null
}

function loadModel() {
  clearModel()
  error.value = null

  if (!scene || !props.modelPath) return

  loading.value = true

  const objPath = props.modelPath
  const mtlPath = props.mtlPath ?? null

  const lastSlash = objPath.lastIndexOf('/')
  const basePath = lastSlash >= 0 ? objPath.slice(0, lastSlash + 1) : ''
  const objFile = lastSlash >= 0 ? objPath.slice(lastSlash + 1) : objPath

  if (mtlPath) {
    const mtlLastSlash = mtlPath.lastIndexOf('/')
    const mtlBasePath = mtlLastSlash >= 0 ? mtlPath.slice(0, mtlLastSlash + 1) : ''
    const mtlFile = mtlLastSlash >= 0 ? mtlPath.slice(mtlLastSlash + 1) : mtlPath

    const mtlLoader = new MTLLoader()
    mtlLoader.setPath(mtlBasePath)
    mtlLoader.load(
      mtlFile,
      materials => {
        materials.preload()

        const objLoader = new OBJLoader()
        objLoader.setMaterials(materials)
        objLoader.setPath(basePath)
        objLoader.load(
          objFile,
          obj => {
            handleLoadedObject(obj)
          },
          undefined,
          err => {
            console.error('OBJ load error', err)
            error.value = 'Не удалось загрузить OBJ (см. консоль).'
            loading.value = false
          }
        )
      },
      undefined,
      err => {
        console.error('MTL load error', err)
        error.value = 'Не удалось загрузить MTL (см. консоль).'
        loading.value = false
      }
    )
  } else {
    const objLoader = new OBJLoader()
    objLoader.setPath(basePath)
    objLoader.load(
      objFile,
      obj => {
        handleLoadedObject(obj)
      },
      undefined,
      err => {
        console.error('OBJ load error', err)
        error.value = 'Не удалось загрузить OBJ (см. консоль).'
        loading.value = false
      }
    )
  }
}

function handleLoadedObject(obj: THREE.Object3D) {
  model = obj

  const box = new THREE.Box3().setFromObject(obj)
  const size = new THREE.Vector3()
  const center = new THREE.Vector3()
  box.getSize(size)
  box.getCenter(center)

  obj.position.sub(center)
  const maxDim = Math.max(size.x, size.y, size.z) || 1
  const scale = 1.5 / maxDim
  obj.scale.setScalar(scale)

  scene!.add(obj)

  if (controls) {
    controls.target.set(0, 0, 0)
    controls.update()
  }

  loading.value = false
}

function animate() {
  if (!renderer || !scene || !camera) return

  animationId = window.requestAnimationFrame(animate)

  if (controls) {
    controls.update()
  }

  renderer.render(scene, camera)
}

function handleResize() {
  if (!renderer || !camera || !container.value) return
  const width = container.value.clientWidth || 1
  const height = container.value.clientHeight || 1
  camera.aspect = width / height
  camera.updateProjectionMatrix()
  renderer.setSize(width, height)
}

onMounted(async () => {
  await nextTick()
  initScene()
  handleResize()
  if (props.modelPath) {
    loadModel()
  }
  window.addEventListener('resize', handleResize)
})

onBeforeUnmount(() => {
  if (animationId != null) {
    cancelAnimationFrame(animationId)
  }
  window.removeEventListener('resize', handleResize)
  clearModel()
  if (renderer) {
    renderer.dispose()
  }
  renderer = null
  scene = null
  camera = null
  if (controls) {
    controls.dispose()
  }
  controls = null
})

watch(
  () => [props.modelPath, props.mtlPath] as const,
  () => {
    if (!scene) return
    if (props.modelPath) {
      loadModel()
    } else {
      clearModel()
    }
  }
)
</script>

<style scoped>
.three-root {
  position: relative;
  width: 100%;
  height: 100%;
}

.three-canvas {
  width: 100%;
  height: 100%;
  display: block;
}

.three-overlay {
  position: absolute;
  inset: 0.5rem;
  border-radius: 0.6rem;
  background: rgba(15, 23, 42, 0.9);
  border: 1px solid rgba(148, 163, 184, 0.7);
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  text-align: center;
  font-size: 0.8rem;
  color: #e5e7eb;
}

.three-overlay.loading {
  border-style: dashed;
}

.three-overlay.error {
  border-color: rgba(248, 113, 113, 0.9);
  color: #fecaca;
}

.msg {
  margin-top: 0.3rem;
  font-size: 0.74rem;
}
</style>
