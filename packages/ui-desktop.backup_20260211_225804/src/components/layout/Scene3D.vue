<template>
  <div ref="containerRef" class="scene3d-root">
    <button class="fullscreen-btn" @click="toggleFullscreen">
      ⤢
    </button>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onBeforeUnmount, watch, nextTick } from 'vue'
import * as THREE from 'three'
import { OrbitControls } from 'three/addons/controls/OrbitControls.js'
import { GLTFLoader } from 'three/addons/loaders/GLTFLoader.js'
import { OBJLoader } from 'three/addons/loaders/OBJLoader.js'
import { STLLoader } from 'three/addons/loaders/STLLoader.js'
import { MTLLoader } from 'three/addons/loaders/MTLLoader.js'
import { useProjectStore } from '@/stores/project'

type SupportedFormat = 'obj' | 'obj+mtl' | 'stl' | 'gltf' | 'glb'

const props = defineProps<{
  modelPath: string | null
  format?: SupportedFormat | null
  mtlPath?: string | null
}>()

const project = useProjectStore()

const containerRef = ref<HTMLElement | null>(null)

let scene: THREE.Scene | null = null
let camera: THREE.PerspectiveCamera | null = null
let renderer: THREE.WebGLRenderer | null = null
let controls: OrbitControls | null = null
let currentObject: THREE.Object3D | null = null

onMounted(() => {
  nextTick(() => {
    initScene()
    if (props.modelPath && props.format) {
      loadModel(props.modelPath, props.format, props.mtlPath || null)
    }
  })
})

onBeforeUnmount(() => {
  disposeScene()
})

watch(
  () => [props.modelPath, props.format, props.mtlPath] as const,
  ([modelPath, format, mtlPath]) => {
    if (!modelPath || !format) {
      clearCurrentObject()
      updateStats(null)
      return
    }
    loadModel(modelPath, format, mtlPath || null)
  }
)

function initScene() {
  const container = containerRef.value
  if (!container) return

  scene = new THREE.Scene()
  scene.background = new THREE.Color(0xf3f4f6)

  const width = container.clientWidth || 800
  const height = container.clientHeight || 600

  camera = new THREE.PerspectiveCamera(60, width / height, 0.1, 1000)
  camera.position.set(5, 5, 5)
  camera.lookAt(0, 0, 0)

  renderer = new THREE.WebGLRenderer({ antialias: true })
  renderer.setPixelRatio(window.devicePixelRatio)
  renderer.setSize(width, height)
  renderer.shadowMap.enabled = true
  renderer.shadowMap.type = THREE.PCFSoftShadowMap
  container.appendChild(renderer.domElement)

  const grid = new THREE.GridHelper(10, 10, 0x9ca3af, 0xd1d5db)
  grid.position.y = 0
  scene.add(grid)

  const floor = new THREE.Mesh(
    new THREE.PlaneGeometry(10, 10),
    new THREE.MeshStandardMaterial({ color: 0xffffff })
  )
  floor.rotation.x = -Math.PI / 2
  floor.receiveShadow = true
  scene.add(floor)

  const dirLight = new THREE.DirectionalLight(0xffffff, 1)
  dirLight.position.set(5, 10, 7)
  dirLight.castShadow = true
  scene.add(dirLight)

  const ambient = new THREE.AmbientLight(0x9ca3af, 0.6)
  scene.add(ambient)

  controls = new OrbitControls(camera, renderer.domElement)
  controls.enableDamping = true
  controls.target.set(0, 0, 0)
  controls.update()

  window.addEventListener('resize', onResize)
  document.addEventListener('fullscreenchange', onResize)

  animate()
}

function clearCurrentObject() {
  if (!scene || !currentObject) return
  scene.remove(currentObject)
  currentObject.traverse((child: THREE.Object3D) => {
    if ((child as any).isMesh) {
      const mesh = child as THREE.Mesh
      mesh.geometry.dispose()
      const mats = Array.isArray(mesh.material) ? mesh.material : [mesh.material]
      mats.forEach(m => m && m.dispose())
    }
  })
  currentObject = null
}

function loadModel(path: string, format: SupportedFormat, mtlPath: string | null) {
  if (!scene) return

  clearCurrentObject()
  updateStats(null)

  console.log('Scene3D load:', { path, format, mtlPath })

  switch (format) {
    case 'gltf':
    case 'glb': {
      const loader = new GLTFLoader()
      loader.load(
        path,
        gltf => {
          currentObject = gltf.scene
          normalizeAndFit(currentObject!)
          scene!.add(currentObject!)
          updateStats(currentObject)
        },
        undefined,
        err => console.error('GLTF load error', err)
      )
      break
    }

    case 'stl': {
      const loader = new STLLoader()
      loader.load(
        path,
        geometry => {
          const material = new THREE.MeshPhongMaterial({ color: 0x9ca3af })
          const mesh = new THREE.Mesh(geometry, material)
          currentObject = mesh
          normalizeAndFit(currentObject!)
          scene!.add(currentObject!)
          updateStats(currentObject)
        },
        undefined,
        err => console.error('STL load error', err)
      )
      break
    }

    case 'obj+mtl': {
      if (!mtlPath) {
        console.error('OBJ+MTL format requires mtlPath')
        return
      }
      const mtlLoader = new MTLLoader()
      mtlLoader.load(
        mtlPath,
        mtl => {
          mtl.preload()
          const objLoader = new OBJLoader()
          objLoader.setMaterials(mtl)
          objLoader.load(
            path,
            obj => {
              currentObject = obj
              normalizeAndFit(currentObject!)
              scene!.add(currentObject!)
              updateStats(currentObject)
            },
            undefined,
            err => console.error('OBJ load error', err)
          )
        },
        undefined,
        err => console.error('MTL load error', err)
      )
      break
    }

    case 'obj': {
      const objLoader = new OBJLoader()
      objLoader.load(
        path,
        obj => {
          currentObject = obj
          normalizeAndFit(currentObject!)
          scene!.add(currentObject!)
          updateStats(currentObject)
        },
        undefined,
        err => console.error('OBJ load error', err)
      )
      break
    }
  }
}

function enableShadows(obj: THREE.Object3D) {
  obj.traverse(child => {
    if ((child as any).isMesh) {
      const mesh = child as THREE.Mesh
      mesh.castShadow = true
      mesh.receiveShadow = true
    }
  })
}

function normalizeAndFit(obj: THREE.Object3D) {
  if (!camera || !controls) return

  enableShadows(obj)

  const box = new THREE.Box3().setFromObject(obj)
  const size = box.getSize(new THREE.Vector3())
  const center = box.getCenter(new THREE.Vector3())
  console.log('[Scene3D] bbox size/center:', size, center)

  let maxDim = Math.max(size.x, size.y, size.z)
  if (!isFinite(maxDim) || maxDim <= 0) {
    maxDim = 1
  }
  const scale = 1 / maxDim
  obj.scale.setScalar(scale)
  obj.position.sub(center.multiplyScalar(scale))

  fitToView(obj)
}

function fitToView(obj: THREE.Object3D) {
  if (!camera || !controls) return

  const box = new THREE.Box3().setFromObject(obj)
  const size = box.getSize(new THREE.Vector3())
  const center = box.getCenter(new THREE.Vector3())
  const maxDim = Math.max(size.x, size.y, size.z) || 1

  const fov = camera.fov * (Math.PI / 180)
  let cameraZ = Math.abs(maxDim / (2 * Math.tan(fov / 2)))
  cameraZ *= 1.5

  camera.position.set(center.x + maxDim, center.y + maxDim * 0.3, center.z + cameraZ)
  camera.near = maxDim / 100
  camera.far = maxDim * 100
  camera.updateProjectionMatrix()

  controls.target.copy(center)
  controls.update()
}

function updateStats(root: THREE.Object3D | null) {
  if (!root) {
    project.set3DModel({ faces: null, parts: null })
    return
  }

  let triangles = 0
  let parts = 0

  root.traverse(obj => {
    if ((obj as any).isMesh) {
      parts += 1
      const mesh = obj as THREE.Mesh
      const geom = mesh.geometry as THREE.BufferGeometry
      const index = geom.index
      if (index) {
        triangles += index.count / 3
      } else {
        const pos = geom.getAttribute('position')
        triangles += pos.count / 3
      }
    }
  })

  const faces = Math.floor(triangles)
  project.set3DModel({ faces, parts })
  console.log('[Scene3D] stats updated:', { faces, parts })
}
 
function onResize() {
  if (!renderer || !camera || !containerRef.value) return
  const rect = containerRef.value.getBoundingClientRect()
  const width = rect.width || 1
  const height = rect.height || 1
  camera.aspect = width / height
  camera.updateProjectionMatrix()
  renderer.setSize(width, height)
}

function animate() {
  requestAnimationFrame(animate)
  if (controls) controls.update()
  if (renderer && scene && camera) renderer.render(scene, camera)
}

function disposeScene() {
  window.removeEventListener('resize', onResize)
  document.removeEventListener('fullscreenchange', onResize)
  clearCurrentObject()
  if (renderer) {
    renderer.dispose()
    renderer.forceContextLoss()
    renderer.domElement.remove()
  }
  scene = null
  camera = null
  renderer = null
  controls = null
}

function toggleFullscreen() {
  const container = containerRef.value
  if (!container) return

  const anyDoc = document as any
  const anyEl = container as any

  if (
    anyDoc.fullscreenElement ||
    anyDoc.webkitFullscreenElement ||
    anyDoc.msFullscreenElement
  ) {
    if (anyDoc.exitFullscreen) anyDoc.exitFullscreen()
    else if (anyDoc.webkitExitFullscreen) anyDoc.webkitExitFullscreen()
    else if (anyDoc.msExitFullscreen) anyDoc.msExitFullscreen()
  } else {
    if (anyEl.requestFullscreen) anyEl.requestFullscreen()
    else if (anyEl.webkitRequestFullscreen) anyEl.webkitRequestFullscreen()
    else if (anyEl.msRequestFullscreen) anyEl.msRequestFullscreen()
  }
}
</script>

<style scoped>
.scene3d-root {
  width: 100%;
  height: 100%;
  position: relative;
  background: #f3f4f6;
}

.fullscreen-btn {
  position: absolute;
  top: 0.5rem;
  right: 0.5rem;
  z-index: 10;
  border: none;
  border-radius: 999px;
  padding: 0.25rem 0.5rem;
  background: rgba(31, 41, 55, 0.7);
  color: #f9fafb;
  cursor: pointer;
  font-size: 0.875rem;
}
.fullscreen-btn:hover {
  background: rgba(17, 24, 39, 0.9);
}
</style>
