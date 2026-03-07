<template>
  <div ref="canvasContainer" class="viewer-3d"></div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted, watch } from 'vue'
import * as THREE from 'three'
import { OrbitControls } from 'three/examples/jsm/controls/OrbitControls.js'
import { useProjectStore } from '@/stores/projectStore'

const canvasContainer = ref<HTMLElement | null>(null)

// Three.js scene objects
let scene: THREE.Scene
let camera: THREE.PerspectiveCamera
let renderer: THREE.WebGLRenderer
let controls: OrbitControls
let currentModel: THREE.Object3D | null = null

// Project store
const projectStore = useProjectStore()

// Initialize Three.js scene
const initScene = () => {
  if (!canvasContainer.value) return

  // Create scene
  scene = new THREE.Scene()
  scene.background = new THREE.Color(0x333333)

  // Create camera
  camera = new THREE.PerspectiveCamera(75, canvasContainer.value.clientWidth / canvasContainer.value.clientHeight, 0.1, 1000)
  camera.position.z = 5

  // Create renderer
  renderer = new THREE.WebGLRenderer({ antialias: true })
  renderer.setSize(canvasContainer.value.clientWidth, canvasContainer.value.clientHeight)
  renderer.shadowMap.enabled = true
  renderer.shadowMap.type = THREE.PCFSoftShadowMap
  canvasContainer.value.appendChild(renderer.domElement)

  // Add orbit controls
  controls = new OrbitControls(camera, renderer.domElement)
  controls.enableDamping = true
  controls.dampingFactor = 0.05

  // Add lights
  const ambientLight = new THREE.AmbientLight(0xffffff, 0.5)
  scene.add(ambientLight)

  const directionalLight = new THREE.DirectionalLight(0xffffff, 1)
  directionalLight.position.set(5, 10, 7)
  directionalLight.castShadow = true
  directionalLight.shadow.mapSize.width = 1024
  directionalLight.shadow.mapSize.height = 1024
  scene.add(directionalLight)

  // Start animation loop
  animate()
}

// Animation loop
const animate = () => {
  requestAnimationFrame(animate)
  
  if (controls) {
    controls.update()
  }
  
  if (renderer && scene && camera) {
    renderer.render(scene, camera)
  }
}

// Update model when parsedPdoData changes
watch(() => projectStore.parsedPdoData, (newData) => {
  if (newData && newData.vertices && newData.faces) {
    updateModel(newData)
  }
}, { immediate: true })

// Expose method for unfolding
defineExpose({
  unfoldModel: async () => {
    if (projectStore.modelPath) {
      await projectStore.unfoldModel()
    }
  }
})

// Update model in the scene
const updateModel = (data: any) => {
  if (!scene) return

  // Clear existing model
  if (currentModel) {
    scene.remove(currentModel)
    currentModel = null
  }

  // Create geometry from parsed data
  const geometry = new THREE.BufferGeometry()
  
  // Set vertices
  const vertices = new Float32Array(data.vertices)
  geometry.setAttribute('position', new THREE.BufferAttribute(vertices, 3))
  
  // Set faces (indices)
  const indices = new Uint32Array(data.faces)
  geometry.setIndex(new THREE.BufferAttribute(indices, 1))
  
  // Compute normals for proper lighting
  geometry.computeVertexNormals()

  // Create material
  const material = new THREE.MeshStandardMaterial({ 
    color: 0x00aaff,
    roughness: 0.5,
    metalness: 0.5,
    wireframe: false
  })

  // Create mesh
  const mesh = new THREE.Mesh(geometry, material)
  mesh.castShadow = true
  mesh.receiveShadow = true
  
  // Center the model
  const box = new THREE.Box3().setFromObject(mesh)
  const center = new THREE.Vector3()
  box.getCenter(center)
  mesh.position.sub(center)
  
  // Scale the model to fit the view
  const size = new THREE.Vector3()
  box.getSize(size)
  const maxDim = Math.max(size.x, size.y, size.z)
  const scale = 3 / maxDim
  mesh.scale.set(scale, scale, scale)
  
  scene.add(mesh)
  currentModel = mesh
}

// Handle window resize
const onWindowResize = () => {
  if (!canvasContainer.value || !camera || !renderer) return
  
  camera.aspect = canvasContainer.value.clientWidth / canvasContainer.value.clientHeight
  camera.updateProjectionMatrix()
  renderer.setSize(canvasContainer.value.clientWidth, canvasContainer.value.clientHeight)
}

// Watch for resize events
let resizeObserver: ResizeObserver | null = null

onMounted(() => {
  initScene()
  
  if (canvasContainer.value) {
    resizeObserver = new ResizeObserver(onWindowResize)
    resizeObserver.observe(canvasContainer.value)
  }
  
  window.addEventListener('resize', onWindowResize)
})

onUnmounted(() => {
  if (resizeObserver && canvasContainer.value) {
    resizeObserver.unobserve(canvasContainer.value)
  }
  
  window.removeEventListener('resize', onWindowResize)
  
  // Clean up Three.js resources
  if (renderer) {
    renderer.dispose()
  }
  
  if (controls) {
    controls.dispose()
  }
})
</script>

<style scoped>
.viewer-3d {
  width: 100%;
  height: 100%;
  position: relative;
  overflow: hidden;
}
</style>