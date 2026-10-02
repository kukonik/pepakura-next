<template>
  <div class="viewer-wrapper">
    <div class="toolbar">
      <button @click="handleLoadFile" :disabled="isLoading" class="load-btn">
        {{ isLoading ? 'Загрузка...' : 'Загрузить .obj файл' }}
      </button>
      <p v-if="stats" class="stats-msg">{{ stats }}</p>
      <p v-if="error" class="error-msg">{{ error }}</p>
    </div>
    <div ref="viewerContainer" class="threejs-viewer"></div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import * as THREE from 'three'
import { OrbitControls } from 'three/examples/jsm/controls/OrbitControls.js'
import { usePlatform } from '@pepakura/shared/composables/usePlatform'
import type { ObjGeometry } from '@pepakura/shared/types/core'

const { getBridge } = usePlatform()
const viewerContainer = ref<HTMLElement | null>(null)
const isLoading = ref<boolean>(false)
const error = ref<string | null>(null)
const stats = ref<string | null>(null)

let scene: THREE.Scene | null = null
let camera: THREE.PerspectiveCamera | null = null
let renderer: THREE.WebGLRenderer | null = null
let controls: OrbitControls | null = null
let animationFrameId = 0

const initScene = () => {
  if (!viewerContainer.value) return
  scene = new THREE.Scene()
  scene.background = new THREE.Color(0x0a0a0a)

  camera = new THREE.PerspectiveCamera(50, window.innerWidth / window.innerHeight, 0.1, 1000)
  camera.position.set(0, 5, 10)
  camera.lookAt(0, 0, 0)

  renderer = new THREE.WebGLRenderer({ antialias: true })
  renderer.setSize(viewerContainer.value.clientWidth, viewerContainer.value.clientHeight)
  renderer.toneMapping = THREE.ACESFilmicToneMapping
  viewerContainer.value.appendChild(renderer.domElement)

  controls = new OrbitControls(camera, renderer.domElement)
  scene.add(new THREE.GridHelper(20, 20, 0x444444))
  scene.add(new THREE.AmbientLight(0xffffff, 0.5))
  const dirLight = new THREE.DirectionalLight(0xffffff, 1)
  dirLight.position.set(5, 10, 7.5)
  scene.add(dirLight)

  const animate = () => {
    animationFrameId = requestAnimationFrame(animate)
    controls?.update()
    renderer?.render(scene, camera)
  }
  animate()

  const resizeObserver = new ResizeObserver(() => {
    if (!viewerContainer.value) return
    if (!camera || !renderer) return
    camera.aspect = viewerContainer.value.clientWidth / viewerContainer.value.clientHeight
    camera.updateProjectionMatrix()
    renderer.setSize(viewerContainer.value.clientWidth, viewerContainer.value.clientHeight)
  })
  resizeObserver.observe(viewerContainer.value)
}

const loadObjToScene = (data: ObjGeometry) => {
  if (!scene) return
  const toRemove = scene.getObjectByName('loaded-mesh')
  if (toRemove) scene.remove(toRemove)

  const geometry = new THREE.BufferGeometry()
  geometry.setAttribute('position', new THREE.Float32BufferAttribute(data.positions, 3))
  geometry.setIndex(data.indices)
  
  // ЗАЩИТА ОТ ПЛОХОЙ ГЕОМЕТРИИ: Если есть ворнинги или индексы кривые - не считаем нормали
  const isCleanGeometry = data.warnings.length === 0
  const material = isCleanGeometry 
    ? new THREE.MeshStandardMaterial({ color: 0x8b5cf6, flatShading: false })
    : new THREE.MeshNormalMaterial({ color: 0x8b5cf6, flatShading: true }); // Защита от краша

  const mesh = new THREE.Mesh(geometry, material)
  mesh.name = 'loaded-mesh'
  scene.add(mesh)

  stats.value = `Граней: ${data.face_count} | Вершин: ${(data.positions.length / 3).toFixed(0)}${data.warnings.length > 0 ? ` | ⚠️ ${data.warnings.length} предупреждений` : ''}`
}

const handleLoadFile = async () => {
  error.value = null
  stats.value = null
  isLoading.value = true

  try {
    const bridge = getBridge()
    const { open } = await import('@tauri-apps/plugin-dialog')
    const selected = await open({
      multiple: false,
      filters: [{ name: '3D Models', extensions: ['obj'] }]
    })

    if (!selected) {
      isLoading.value = false
      return
    }

    const { readTextFile } = await import('@tauri-apps/plugin-fs')
    const objString = await readTextFile(selected)
    const geometryData = await bridge.loadRealObj(objString)
    
    loadObjToScene(geometryData)
  } catch (err: unknown) {
    error.value = err instanceof Error ? err.message : 'Неизвестная ошибка при загрузке файла'
  } finally {
    isLoading.value = false
  }
}

onMounted(() => initScene())
onUnmounted(() => {
  cancelAnimationFrame(animationFrameId)
  controls?.dispose()
  renderer?.dispose()
})
</script>

<style scoped>
.viewer-wrapper {
  width: 100%;
  height: 100vh;
  background: black;
  display: flex;
  flex-direction: column;
  position: relative;
}
.toolbar {
  position: absolute;
  top: 20px;
  left: 20px;
  z-index: 100;
  display: flex;
  flex-direction: column;
  gap: 8px;
}
.load-btn {
  background: rgba(255, 255, 255, 0.1);
  color: #e2e8f0;
  border: 1px solid rgba(255, 255, 255, 0.2);
  padding: 8px 16px;
  border-radius: 6px;
  cursor: pointer;
  font-size: 14px;
  transition: background 0.2s;
  width: fit-content;
}
.load-btn:hover:not(:disabled) { background: rgba(255, 255, 255, 0.2); }
.load-btn:disabled { opacity: 0.5; cursor: not-allowed; }
.error-msg {
  color: #f87171;
  background: rgba(0, 0, 0, 0.9);
  padding: 8px 12px;
  border-radius: 4px;
  font-size: 12px;
  max-width: 400px;
  word-break: break-word;
}
.stats-msg {
  color: #94a3b8;
  font-size: 12px;
  background: rgba(0, 0, 0, 0.6);
  padding: 6px 12px;
  border-radius: 4px;
}
.threejs-viewer {
  flex: 1;
  width: 100%;
  overflow: hidden;
}
</style>