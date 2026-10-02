<template>
  <div class="three-d-viewer-canvas">
    <div ref="containerRef" class="viewer-container"></div>
    <div class="viewer-controls">
      <button @click="resetView">Сброс</button>
      <button @click="toggleWireframe">Каркас</button>
      <button @click="toggleTextures">Текстуры</button>
      <button @click="toggleShadows">Тени</button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onBeforeUnmount, watch } from 'vue'
import * as THREE from 'three'
import { OrbitControls } from 'three/examples/jsm/controls/OrbitControls.js'
import { OBJLoader } from 'three/examples/jsm/loaders/OBJLoader.js'
import { MTLLoader } from 'three/examples/jsm/loaders/MTLLoader.js'

const containerRef = ref<HTMLElement | null>(null)
const isWireframe = ref(false)
const showTextures = ref(true)
const showShadows = ref(true)

// Three.js scene objects
let scene: THREE.Scene
let camera: THREE.PerspectiveCamera
let renderer: THREE.WebGLRenderer
let controls: OrbitControls
let currentModel: THREE.Object3D | null = null
let directionalLight: THREE.DirectionalLight
let ambientLight: THREE.AmbientLight

// Initialize Three.js scene
const initScene = () => {
  if (!containerRef.value) return

  // Create scene
  scene = new THREE.Scene()
  scene.background = new THREE.Color(0x333333)

  // Create camera
  camera = new THREE.PerspectiveCamera(75, containerRef.value.clientWidth / containerRef.value.clientHeight, 0.1, 1000)
  camera.position.z = 5

  // Create renderer
  renderer = new THREE.WebGLRenderer({ antialias: true })
  renderer.setSize(containerRef.value.clientWidth, containerRef.value.clientHeight)
  renderer.shadowMap.enabled = true
  renderer.shadowMap.type = THREE.PCFSoftShadowMap
  containerRef.value.appendChild(renderer.domElement)

  // Add orbit controls
  controls = new OrbitControls(camera, renderer.domElement)
  controls.enableDamping = true
  controls.dampingFactor = 0.05

  // Add lights
  ambientLight = new THREE.AmbientLight(0xffffff, 0.5)
  scene.add(ambientLight)

  directionalLight = new THREE.DirectionalLight(0xffffff, 1)
  directionalLight.position.set(5, 10, 7)
  directionalLight.castShadow = true
  directionalLight.shadow.mapSize.width = 1024
  directionalLight.shadow.mapSize.height = 1024
  scene.add(directionalLight)

  // Add test cube
  const geometry = new THREE.BoxGeometry(1, 1, 1)
  const material = new THREE.MeshStandardMaterial({ 
    color: 0x00ff00,
    roughness: 0.5,
    metalness: 0.5
  })
  const cube = new THREE.Mesh(geometry, material)
  cube.castShadow = true
  cube.receiveShadow = true
  scene.add(cube)
  currentModel = cube

  // Add plane for shadows
  const planeGeometry = new THREE.PlaneGeometry(10, 10)
  const planeMaterial = new THREE.MeshStandardMaterial({ color: 0xaaaaaa })
  const plane = new THREE.Mesh(planeGeometry, planeMaterial)
  plane.rotation.x = -Math.PI / 2
  plane.position.y = -1
  plane.receiveShadow = true
  scene.add(plane)

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

// Load 3D model
const loadModel = (objUrl: string, mtlUrl?: string) => {
  if (!scene) return

  // Clear existing model
  if (currentModel) {
    scene.remove(currentModel)
    currentModel = null
  }

  const loader = new OBJLoader()
  
  if (mtlUrl) {
    // Load materials first
    const mtlLoader = new MTLLoader()
    mtlLoader.load(mtlUrl, (materials) => {
      materials.preload()
      loader.setMaterials(materials)
      
      // Load object
      loader.load(objUrl, (object) => {
        processModel(object)
      })
    })
  } else {
    // Load object without materials
    loader.load(objUrl, (object) => {
      processModel(object)
    })
  }
}

// Process loaded model
const processModel = (object: THREE.Group) => {
  if (!scene) return

  // Center the model
  const box = new THREE.Box3().setFromObject(object)
  const center = new THREE.Vector3()
  box.getCenter(center)
  object.position.sub(center)
  
  // Scale the model to fit the view
  const size = new THREE.Vector3()
  box.getSize(size)
  const maxDim = Math.max(size.x, size.y, size.z)
  const scale = 3 / maxDim
  object.scale.set(scale, scale, scale)
  
  // Enable shadows
  object.traverse((child) => {
    if (child instanceof THREE.Mesh) {
      child.castShadow = true
      child.receiveShadow = true
      
      // Apply wireframe if needed
      if (isWireframe.value && child.material) {
        if (Array.isArray(child.material)) {
          child.material.forEach(mat => {
            if (mat instanceof THREE.MeshStandardMaterial) {
              mat.wireframe = true
            }
          })
        } else if (child.material instanceof THREE.MeshStandardMaterial) {
          child.material.wireframe = true
        }
      }
    }
  })
  
  scene.add(object)
  currentModel = object
}

// Reset view
const resetView = () => {
  if (camera && controls) {
    camera.position.set(0, 0, 5)
    camera.lookAt(0, 0, 0)
    controls.target.set(0, 0, 0)
    controls.update()
  }
}

// Toggle wireframe
const toggleWireframe = () => {
  isWireframe.value = !isWireframe.value
  
  if (currentModel) {
    currentModel.traverse((child) => {
      if (child instanceof THREE.Mesh && child.material) {
        if (Array.isArray(child.material)) {
          child.material.forEach(mat => {
            if (mat instanceof THREE.MeshStandardMaterial) {
              mat.wireframe = isWireframe.value
            }
          })
        } else if (child.material instanceof THREE.MeshStandardMaterial) {
          child.material.wireframe = isWireframe.value
        }
      }
    })
  }
}

// Toggle textures
const toggleTextures = () => {
  showTextures.value = !showTextures.value
  // In a real implementation, this would toggle texture visibility
  console.log('Toggle textures:', showTextures.value)
}

// Toggle shadows
const toggleShadows = () => {
  showShadows.value = !showShadows.value
  renderer.shadowMap.enabled = showShadows.value
}

// Handle window resize
const onWindowResize = () => {
  if (!containerRef.value || !camera || !renderer) return
  
  camera.aspect = containerRef.value.clientWidth / containerRef.value.clientHeight
  camera.updateProjectionMatrix()
  renderer.setSize(containerRef.value.clientWidth, containerRef.value.clientHeight)
}

// Watch for resize events
let resizeObserver: ResizeObserver | null = null

onMounted(() => {
  initScene()
  
  if (containerRef.value) {
    resizeObserver = new ResizeObserver(onWindowResize)
    resizeObserver.observe(containerRef.value)
  }
  
  window.addEventListener('resize', onWindowResize)
})

onBeforeUnmount(() => {
  if (resizeObserver && containerRef.value) {
    resizeObserver.unobserve(containerRef.value)
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
.three-d-viewer-canvas {
  width: 100%;
  height: 100%;
  position: relative;
  overflow: hidden;
}

.viewer-container {
  width: 100%;
  height: 100%;
}

.viewer-controls {
  position: absolute;
  top: 16px;
  right: 16px;
  display: flex;
  gap: 8px;
  z-index: 10;
}

.viewer-controls button {
  background: rgba(255, 255, 255, 0.8);
  border: 1px solid var(--border-color);
  border-radius: 4px;
  padding: 8px 12px;
  cursor: pointer;
  font-size: 0.75rem;
}

.viewer-controls button:hover {
  background: rgba(255, 255, 255, 0.9);
}
</style>