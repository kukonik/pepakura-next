<template>
  <div class="workspace">
    <!-- Split view: 3D and 2D -->
    <div class="workspace-split">
      <!-- 3D Viewer -->
      <div class="workspace-panel" :style="{ flex: splitPosition }">
        <div class="panel-header">
          <h3>3D Модель</h3>
          <span class="panel-badge">{{ mesh?.name || 'Без названия' }}</span>
        </div>
        <Viewer3D
          ref="viewer3d"
          :mesh="mesh"
          @faceSelect="onFaceSelect3D"
          @faceHover="onFaceHover3D"
        />
      </div>

      <!-- Resizer -->
      <div 
        class="workspace-resizer"
        @mousedown="startResize"
      >
        <div class="resizer-handle"></div>
      </div>

      <!-- 2D Editor -->
      <div class="workspace-panel" :style="{ flex: 1 - splitPosition }">
        <div class="panel-header">
          <h3>2D Развёртка</h3>
          <span class="panel-badge">{{ unfoldedMesh?.faces?.length || 0 }} деталей</span>
        </div>
        <UnfoldEditor
          ref="editor2d"
          :unfolded-mesh="unfoldedMesh"
          @partSelect="onPartSelect2D"
          @export="onExport"
        />
      </div>
    </div>

    <!-- Status bar -->
    <div class="workspace-status">
      <div class="status-left">
        <span v-if="selectedFace !== null">
          Выбрана грань: {{ selectedFace + 1 }}
        </span>
        <span v-else>
          Кликните на грань для выделения
        </span>
      </div>
      <div class="status-right">
        <span>3D: {{ viewLinking.selectedFace3D ?? '-' }}</span>
        <span>2D: {{ viewLinking.selectedFace2D ?? '-' }}</span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import Viewer3D from '@/components/viewer/Viewer3D.vue'
import UnfoldEditor from '@/components/editor/UnfoldEditor.vue'
import { useViewLinking } from '@/composables/useViewLinking'

// Props
interface MeshData {
  vertices: Array<[number, number, number]>
  faces: Array<[number, number, number]>
  name?: string
}

const props = defineProps<{
  mesh: MeshData | null
  unfoldedMesh: any
}>()

// State
const viewer3d = ref<InstanceType<typeof Viewer3D> | null>(null)
const editor2d = ref<InstanceType<typeof UnfoldEditor> | null>(null)
const splitPosition = ref(0.5)
const isResizing = ref(false)

// View linking
const viewLinking = useViewLinking()

// Computed
const selectedFace = computed(() => viewLinking.selectedFace2D.value)
const mesh = computed(() => props.mesh)

// Event handlers
function onFaceSelect3D(faceIndex: number) {
  viewLinking.selectFace3D(faceIndex)
}

function onFaceHover3D(faceIndex: number | null) {
  viewLinking.hoverFace3D(faceIndex)
}

function onPartSelect2D(partIndex: number) {
  viewLinking.selectFace2D(partIndex)
}

function onExport(format: 'svg' | 'pdf') {
  // TODO: Implement export
  console.log('Export to', format)
}

// Resize
function startResize(event: MouseEvent) {
  isResizing.value = true
  document.addEventListener('mousemove', onResize)
  document.addEventListener('mouseup', stopResize)
}

function onResize(event: MouseEvent) {
  if (!isResizing.value) return
  
  const container = document.querySelector('.workspace-split') as HTMLElement
  if (!container) return
  
  const rect = container.getBoundingClientRect()
  const newSplit = (event.clientX - rect.left) / rect.width
  
  splitPosition.value = Math.max(0.2, Math.min(0.8, newSplit))
}

function stopResize() {
  isResizing.value = false
  document.removeEventListener('mousemove', onResize)
  document.removeEventListener('mouseup', stopResize)
}
</script>

<style scoped>
.workspace {
  display: flex;
  flex-direction: column;
  height: 100%;
  overflow: hidden;
}

.workspace-split {
  display: flex;
  flex: 1;
  overflow: hidden;
}

.workspace-panel {
  display: flex;
  flex-direction: column;
  min-width: 200px;
  background: #f5f5f5;
}

.panel-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 8px 12px;
  background: white;
  border-bottom: 1px solid #e0e0e0;
}

.panel-header h3 {
  margin: 0;
  font-size: 14px;
  font-weight: 600;
  color: #333;
}

.panel-badge {
  padding: 2px 8px;
  background: #e0e0e0;
  border-radius: 4px;
  font-size: 11px;
  color: #666;
}

.workspace-resizer {
  width: 8px;
  background: #e0e0e0;
  cursor: col-resize;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: background 0.2s;
}

.workspace-resizer:hover {
  background: #1976d2;
}

.resizer-handle {
  width: 2px;
  height: 40px;
  background: #999;
  border-radius: 1px;
}

.workspace-status {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 8px 16px;
  background: white;
  border-top: 1px solid #e0e0e0;
  font-size: 12px;
  color: #666;
}

.status-left {
  display: flex;
  gap: 16px;
}

.status-right {
  display: flex;
  gap: 16px;
}
</style>
