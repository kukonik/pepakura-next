<template>
  <canvas ref="canvasRef" style="width: 100%; height: 100%; display: block;" />
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted, watch } from 'vue'
import * as THREE from 'three'
import { OBJLoader } from 'three/examples/jsm/loaders/OBJLoader.js'
import { OrbitControls } from 'three/examples/jsm/controls/OrbitControls.js'

const canvasRef = ref<HTMLCanvasElement>()
let scene: THREE.Scene | null = null
let renderer: THREE.WebGLRenderer
let camera: THREE.PerspectiveCamera
let controls: OrbitControls
let animationId: number

const props = defineProps<{ objText: string }>()

onMounted(() => {
  initScene()
  if (props.objText) {
    loadModel(props.objText)
  }
})

onUnmounted(() => {
  cancelAnimationFrame(animationId)
  window.removeEventListener('resize', onWindowResize)
  renderer.dispose()
})

watch(
  () => props.objText,
  (newText) => {
    if (newText) {
      if (scene) {
        loadModel(newText)
      } else {
        setTimeout(() => {
          if (scene) loadModel(newText)
        }, 100)
      }
    }
  }
)

function initScene() {
  if (!canvasRef.value) return

  // Сцена
  scene = new THREE.Scene()
  scene.background = new THREE.Color(0x1a1a2e)

  // Камера
  camera = new THREE.PerspectiveCamera(
    45,
    canvasRef.value.clientWidth / canvasRef.value.clientHeight,
    0.1,
    100
  )
  camera.position.set(5, 5, 5)
  camera.lookAt(0, 0, 0)

  // Рендерер
  renderer = new THREE.WebGLRenderer({
    canvas: canvasRef.value,
    antialias: true,
  })
  renderer.setSize(canvasRef.value.clientWidth, canvasRef.value.clientHeight)
  renderer.setPixelRatio(Math.min(window.devicePixelRatio, 2))
  renderer.shadowMap.enabled = true // на будущее

  // OrbitControls
  controls = new OrbitControls(camera, renderer.domElement)
  controls.enableDamping = true
  controls.dampingFactor = 0.1

  // === Улучшенное освещение ===
  // 1. Полусферический свет (небо/земля) для мягкой заливки
  const hemiLight = new THREE.HemisphereLight(
    0xddeeff, // цвет неба (холодный)
    0x3b3020, // цвет земли (тёплый)
    0.8        // интенсивность
  )
  scene.add(hemiLight)

  // 2. Направленный свет (солнце) с тенями (опционально)
  const dirLight = new THREE.DirectionalLight(0xffffff, 1.2)
  dirLight.position.set(5, 10, 7)
  dirLight.castShadow = true
  dirLight.receiveShadow = true
  dirLight.shadow.mapSize.width = 1024
  dirLight.shadow.mapSize.height = 1024
  dirLight.shadow.camera.near = 0.5
  dirLight.shadow.camera.far = 50
  scene.add(dirLight)

  // 3. Фоновый свет (подсветка теней)
  const ambientLight = new THREE.AmbientLight(0x404066) // чуть синеватый
  scene.add(ambientLight)

  // Сетка и оси
  const grid = new THREE.GridHelper(5, 10)
  scene.add(grid)
  const axes = new THREE.AxesHelper(3)
  axes.name = 'axesHelper'
  scene.add(axes)

  // Отладочный куб (исчезнет после загрузки модели)
  const debugCube = new THREE.Mesh(
    new THREE.BoxGeometry(0.5, 0.5, 0.5),
    new THREE.MeshStandardMaterial({ color: 0xff0000, roughness: 0.5, metalness: 0.1 })
  )
  debugCube.name = 'debugCube'
  debugCube.castShadow = true
  debugCube.receiveShadow = true
  scene.add(debugCube)
  console.log('🟥 Debug Cube added')

  // Ресайз
  window.addEventListener('resize', onWindowResize)

  // Запуск анимации
  animate()
}

function onWindowResize() {
  if (!canvasRef.value || !camera || !renderer) return
  const width = canvasRef.value.clientWidth
  const height = canvasRef.value.clientHeight
  camera.aspect = width / height
  camera.updateProjectionMatrix()
  renderer.setSize(width, height)
}

function animate() {
  animationId = requestAnimationFrame(animate)
  if (controls) controls.update()
  renderer.render(scene!, camera)
}

function removeDebugCube() {
  if (!scene) return
  const cube = scene.getObjectByName('debugCube')
  if (cube) scene.remove(cube)
  console.log('🟥 Debug Cube removed')
}

function removeOldModel() {
  if (!scene) return
  const toRemove: THREE.Object3D[] = []
  scene.traverse((obj) => {
    if (obj instanceof THREE.Mesh && obj.name !== 'debugCube') {
      toRemove.push(obj)
    }
  })
  toRemove.forEach((obj) => scene!.remove(obj))
}

async function loadModel(text: string) {
  if (!scene) {
    console.warn('⚠️ loadModel called before scene init. Skipping.')
    return
  }

  try {
    console.log('⚡ Loading model, length:', text.length)
    removeDebugCube()

    const cleanText = sanitizeObjText(text)
    console.log('🧹 Sanitized. Length:', cleanText.length)

    const loader = new OBJLoader()
    const group = loader.parse(cleanText)
    console.log('✅ Parsed. Meshes:', group.children.length)

    if (!group.children.length) {
      console.warn('⚠️ No meshes in OBJ group')
      return
    }

    removeOldModel()

    // Назначим всем мешам стандартный материал, если его нет, и включим тени
    group.traverse((child) => {
      if (child instanceof THREE.Mesh) {
        const geom = child.geometry as THREE.BufferGeometry
        if (geom) cleanGeometry(geom)
        // Если материал отсутствует или базовый, заменим на Standard
        if (!child.material || Array.isArray(child.material)) {
          child.material = new THREE.MeshStandardMaterial({
            color: 0xcccccc,
            roughness: 0.6,
            metalness: 0.1,
          })
        } else if (child.material instanceof THREE.MeshBasicMaterial) {
          // BasicMaterial не реагирует на свет — заменим
          child.material = new THREE.MeshStandardMaterial({
            color: (child.material as THREE.MeshBasicMaterial).color,
            roughness: 0.6,
            metalness: 0.1,
          })
        }
        child.castShadow = true
        child.receiveShadow = true
      }
    })

    scene!.add(group)

    // Вычисление ограничивающего параллелепипеда и позиционирование камеры
    const box = new THREE.Box3().setFromObject(group)
    const size = box.getSize(new THREE.Vector3())
    console.log(`📐 Size: ${size.x.toFixed(2)} x ${size.y.toFixed(2)} x ${size.z.toFixed(2)}`)

    if (size.x > 0 && size.y > 0 && size.z > 0) {
      const center = box.getCenter(new THREE.Vector3())
      camera.position.copy(
        center.clone().add(new THREE.Vector3(size.x * 1.2, size.y * 1.2, size.z * 1.2))
      )
      camera.lookAt(center)
      controls.target.copy(center)
      controls.update()
    } else {
      console.warn('⚠️ Zero bounding box, camera unchanged.')
    }
  } catch (err) {
    console.error('❌ Failed to load model:', err)
    if (scene && !scene.getObjectByName('debugCube')) {
      const debugCube = new THREE.Mesh(
        new THREE.BoxGeometry(0.5, 0.5, 0.5),
        new THREE.MeshStandardMaterial({ color: 0xff0000, roughness: 0.5, metalness: 0.1 })
      )
      debugCube.name = 'debugCube'
      debugCube.castShadow = true
      debugCube.receiveShadow = true
      scene.add(debugCube)
    }
  }
}

function sanitizeObjText(text: string): string {
  const lines = text.split(/\r?\n/)
  const validLines = lines.filter((line) => {
    const trimmed = line.trim()
    if (trimmed === '' || trimmed.startsWith('#')) return true
    if (!/^[\x00-\x7F]*$/.test(line)) return false
    const prefix = trimmed.split(/\s+/)[0]
    const validPrefixes = ['v', 'vt', 'vn', 'f', 'o', 'g', 's', 'usemtl', 'mtllib']
    if (!validPrefixes.includes(prefix.toLowerCase())) {
      console.warn('Removing unsupported line:', line.substring(0, 80))
      return false
    }
    return true
  })
  return validLines.join('\n')
}

function cleanGeometry(geom: THREE.BufferGeometry) {
  const posAttr = geom.getAttribute('position')
  if (!posAttr) return
  const array = posAttr.array as Float32Array
  let nanCount = 0
  for (let i = 0; i < array.length; i++) {
    if (isNaN(array[i])) {
      array[i] = 0
      nanCount++
    }
  }
  if (nanCount > 0) {
    console.warn(`⚠️ Replaced ${nanCount} NaN values`)
    posAttr.needsUpdate = true
  }
  geom.computeBoundingSphere()
  geom.computeBoundingBox()
}
</script>
