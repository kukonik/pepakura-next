<template>
  <div 
    ref="containerRef" 
    class="sheet-preview"
    @mousedown="startPan"
    @wheel="handleWheel"
  >
    <div 
      class="sheet-content"
      :style="{
        transform: `translate(${offsetX}px, ${offsetY}px) scale(${scale})`,
        cursor: isPanning ? 'grabbing' : 'grab'
      }"
    >
      <div v-if="svgContent" v-html="svgContent"></div>
      <div v-else class="placeholder">
        <div class="placeholder-icon">📐</div>
        <p>Нет данных для отображения</p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'
import { useProjectStore } from '@/stores/projectStore'

const containerRef = ref<HTMLElement | null>(null)
const offsetX = ref(0)
const offsetY = ref(0)
const scale = ref(1)
const isPanning = ref(false)
const startX = ref(0)
const startY = ref(0)
const startOffsetX = ref(0)
const startOffsetY = ref(0)

// Project store
const projectStore = useProjectStore()

// SVG content from store
const svgContent = computed(() => {
  return projectStore.unfoldedSvg || null
})

// Start panning
const startPan = (event: MouseEvent) => {
  if (!containerRef.value) return
  
  isPanning.value = true
  startX.value = event.clientX
  startY.value = event.clientY
  startOffsetX.value = offsetX.value
  startOffsetY.value = offsetY.value
  
  containerRef.value.style.cursor = 'grabbing'
  event.preventDefault()
}

// Handle mouse move during panning
const handlePan = (event: MouseEvent) => {
  if (!isPanning.value) return
  
  offsetX.value = startOffsetX.value + (event.clientX - startX.value)
  offsetY.value = startOffsetY.value + (event.clientY - startY.value)
}

// End panning
const endPan = () => {
  isPanning.value = false
  if (containerRef.value) {
    containerRef.value.style.cursor = 'grab'
  }
}

// Handle wheel for zooming
const handleWheel = (event: WheelEvent) => {
  if (!containerRef.value) return
  
  event.preventDefault()
  
  const rect = containerRef.value.getBoundingClientRect()
  const mouseX = event.clientX - rect.left
  const mouseY = event.clientY - rect.top
  
  // Calculate zoom factor
  const zoomIntensity = 0.1
  const wheel = event.deltaY < 0 ? 1 : -1
  const zoom = Math.exp(wheel * zoomIntensity)
  
  // Apply zoom
  scale.value *= zoom
  
  // Adjust offset to zoom towards mouse position
  offsetX.value -= (mouseX - offsetX.value) * (zoom - 1)
  offsetY.value -= (mouseY - offsetY.value) * (zoom - 1)
}

// Reset view
const resetView = () => {
  offsetX.value = 0
  offsetY.value = 0
  scale.value = 1
}

// Event listeners
onMounted(() => {
  document.addEventListener('mousemove', handlePan)
  document.addEventListener('mouseup', endPan)
})

onUnmounted(() => {
  document.removeEventListener('mousemove', handlePan)
  document.removeEventListener('mouseup', endPan)
})
</script>

<style scoped>
.sheet-preview {
  width: 100%;
  height: 100%;
  overflow: hidden;
  position: relative;
  background: #f0f0f0;
  border: 1px solid #ddd;
}

.sheet-content {
  width: 100%;
  height: 100%;
  transform-origin: 0 0;
}

.placeholder {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 100%;
  color: #999;
}

.placeholder-icon {
  font-size: 3rem;
  margin-bottom: 16px;
}
</style>