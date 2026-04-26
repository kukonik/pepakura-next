<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import * as THREE from 'three'
import { OrbitControls } from 'three/examples/jsm/controls/OrbitControls'
import { usePlatform } from '@pepakura/shared/composables/usePlatform'
import type { UnfoldResult } from '@pepakura/shared/types/core'
import SanitizePanel from './SanitizePanel.vue'

const { getBridge } = usePlatform()

// Состояние UI
const isLoading = ref<boolean>(false)
const error = ref<string | null>(null)
const resultData = ref<UnfoldResult | null>(null)

// Данные из Rust
const vertices2d = ref<number[]>([])
const faces = ref<number[][]>([])

// 3D Сцена
let scene: THREE.Scene | null = null
let camera: THREE.PerspectiveCamera | null = null
let renderer: THREE.WebGLRenderer | null = null
let controls: OrbitControls | null = null
let animationFrameId: number = 0
const viewerContainer = ref<HTMLElement | null>(null)

// Инициализация 3D сцены
const initScene = () => {
  if (!viewerContainer.value) return

  scene = new THREE.Scene()
  scene.background = new THREE.Color(0x0a0a0a)
  camera = new THREE.PerspectiveCamera(75, viewerContainer.value.clientWidth / viewerContainer.value.clientHeight, 0.1, 1000)
  camera.position.set(0, 5, 10)
  camera.lookAt(0, 0, 0)
  
  renderer = new THREE.WebGLRenderer({ antialias: true })
  renderer.setSize(viewerContainer.value.clientWidth, viewerContainer.value.clientHeight)
  viewerContainer.value.appendChild(renderer.domElement)

  controls = new OrbitControls(camera, renderer.domElement)
  scene.add(new THREE.GridHelper(20, 20, 0x444444))

  const ambient = new THREE.AmbientLight(0xffffff, 0.5)
  scene.add(ambient)
  const dirLight = new THREE.DirectionalLight(0xffffff, 1)
  dirLight.position.set(5, 10, 7.5)
  scene.add(dirLight)

  const animate = () => {
    animationFrameId = requestAnimationFrame(animate)
    controls?.update()
    if (scene && camera && renderer) {
      renderer.render(scene, camera)
    }
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

// Отрисовка 2D выкройки в виде точек на плоскости
const render2DPoints = () => {
  if (!scene || vertices2d.value.length === 0) return

  const points = new THREE.BufferGeometry();
  const positions = new Float32Array(vertices2d.value);
  points.setAttribute('position', new THREE.BufferAttribute(positions, 3))
  
  const material = new THREE.PointsMaterial({
    color: 0x00ff00, // Зеленые точки (2D выкройка)
    size: 3.0,
    sizeAttenuation: true,
  });

  const pointsMesh = new THREE.Points(points, material)
  pointsMesh.name = 'unfolded-2d-points'
  scene.add(pointsMesh)
}

// Вызов к Rust
const handleUnfold = async () => {
  isLoading.value = true
  error.value = null
  resultData.value = null

  try {
    const bridge = getBridge()
    
    // Вызываем LSCM (передаем JSON из оригинальной функции)
    const result = await bridge.unfold_lscm('[]', '{}')
    
    // Заполняем данные в реактивность
    if (result.error_msg) {
      error.value = result.error_msg
      isLoading.value = false
      return
    }

    // Рендерим точки
    vertices2d.value = result.vertices_2d
    faces.value = result.faces as number[][]
    resultData.value = result
    render2DPoints()
  } catch (err: unknown) {
    error.value = err instanceof Error ? err.message : 'Неизвестная ошибка при разворачивании'
  } finally {
    isLoading.value = false
  }
}

const handleLoadObj = () => {
  // Заглушка для загрузки .obj файла
  console.log('Загрузка .obj файла')
}

onMounted(() => {
  initScene()
})

onUnmounted(() => {
  cancelAnimationFrame(animationFrameId)
  controls?.dispose()
  renderer?.dispose()
})
</script>

<template>
  <div class="split-layout">
    <!-- Левая панель: 3D вьювер + загрузка -->
    <div class="panel-left">
      <div class="toolbar">
        <button @click="handleLoadObj" :disabled="isLoading" class="load-btn">
          {{ isLoading ? 'Загрузка...' : 'Загрузить .obj файл' }}
        </button>
      </div>
      
      <!-- 3D Сцена -->
      <div ref="viewerContainer" class="viewer-3d"></div>
    </div>

    <!-- Правая панель: Результаты 2D развёртки -->
    <div class="panel-right">
      <div class="panel-header">2D Выкройка</div>
      
      <button @click="handleUnfold" :disabled="isLoading" class="unfold-btn">
        ⚡️ Запустить LSCM разворот
      </button>

      <!-- Панель оптимизации и экспорта -->
      <SanitizePanel class="mt-4" />

      <!-- Ошибка -->
      <div v-if="error" class="error-msg">{{ error }}</div>

      <!-- Успех -->
      <div v-if="resultData && !error" class="success-msg">
        <p class="success-title">Успешно!</p>
        <p>Граней: {{ resultData.faces.length }}</p>
        <!-- В АЛЬФА-1 мы нарисуем точки здесь через Canvas или SVG, а пока просто покажем JSON -->
        <div class="json-preview">
          <pre>{{ JSON.stringify(resultData.vertices_2d.slice(0, 50)) }}...</pre>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.split-layout {
  display: flex;
  flex-direction: row;
  height: 100vh;
  width: 100vw;
  overflow: hidden;
}
.panel-left {
  flex: 7; /* 70% ширины на 3D */
  height: 100%;
  background: #050505;
  border-right: 2px solid rgba(255, 255, 255, 0.1);
  display: flex;
  flex-direction: column;
}
.panel-right {
  flex: 3; /* 30% ширины под 2D */
  height: 100%;
  background: #0f172a;
  border-left: 2px solid rgba(255, 255, 255, 0.1);
  display: flex;
  flex-direction: column;
  padding: 1.5rem;
  overflow-y: auto;
}
.panel-header {
  font-size: 16px;
  font-weight: 600;
  color: #e2e8f0;
  margin-bottom: 1rem;
  padding-bottom: 1rem;
  border-bottom: 1px solid rgba(255, 255, 255, 0.1);
}

.toolbar {
  padding: 1rem;
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
}
.load-btn:hover:not(:disabled) { background: rgba(255, 255, 255, 0.2); }
.load-btn:disabled { opacity: 0.5; cursor: not-allowed; }

.unfold-btn {
  background: rgba(139, 92, 246, 0.8);
  color: white;
  border: none;
  padding: 12px 24px;
  border-radius: 6px;
  cursor: pointer;
  font-weight: 600;
  font-size: 15px;
  box-shadow: 0 4px 15px rgba(0, 0, 0, 0.4);
  transition: box-shadow 0.2s;
  margin-bottom: 1rem;
}
.unfold-btn:hover:not(:disabled) { 
  background: rgba(34, 197, 94, 0.9); 
  box-shadow: 0 4px 20px rgba(34, 197, 94, 0.6); 
}

.error-msg {
  color: #f87171;
  background: rgba(0, 0, 0, 0.9);
  padding: 10px 12px;
  border-radius: 4px;
  font-size: 12px;
  max-width: 100%;
  word-break: break-word;
  margin-top: 1rem;
}
.success-msg {
  background: rgba(34, 197, 94, 0.15);
  border: 1px solid rgba(34, 197, 94, 0.4);
  border-radius: 6px;
  padding: 1rem;
}
.success-title {
  color: #4ade80;
  margin-bottom: 8px;
  font-size: 16px;
  font-weight: 600;
}
.json-preview {
  background: rgba(0, 0, 0, 0.7);
  border: 1px solid rgba(255, 255, 255, 0.1);
  padding: 1rem;
  border-radius: 4px;
  overflow-y: auto;
  max-height: 200px;
  font-family: monospace;
  font-size: 11px;
  color: #94a3b8;
  line-height: 1.4;
  white-space: pre-wrap;
  word-break: break-word;
  margin-top: 0.5rem;
}
.viewer-3d {
  flex: 1;
  width: 100%;
}
</style>