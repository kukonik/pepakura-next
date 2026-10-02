<template>
  <div class="unfold-editor" :class="{ 'is-dragging': editor.isDragging }">
    <!-- Toolbar -->
    <div class="editor-toolbar">
      <div class="toolbar-group">
        <button @click="editor.undo" :disabled="!editor.canUndo" title="Отменить (Ctrl+Z)">
          <i class="fas fa-undo"></i>
        </button>
        <button @click="editor.redo" :disabled="!editor.canRedo" title="Повторить (Ctrl+Y)">
          <i class="fas fa-redo"></i>
        </button>
      </div>

      <div class="toolbar-group">
        <button
          :class="{ active: editor.settings.snapToGrid }"
          @click="toggleSnapToGrid"
          title="Привязка к сетке"
        >
          <i class="fas fa-magnet"></i>
          <span>Сетка</span>
        </button>
        <button
          :class="{ active: editor.settings.showGrid }"
          @click="toggleShowGrid"
          title="Показать сетку"
        >
          <i class="fas fa-th"></i>
        </button>
      </div>

      <div class="toolbar-group" v-if="editor.hasSelection">
        <button @click="() => editor.rotateSelected(-90)" title="Повернуть -90°">
          <i class="fas fa-rotate-left"></i>
        </button>
        <button @click="() => editor.rotateSelected(90)" title="Повернуть +90°">
          <i class="fas fa-rotate-right"></i>
        </button>
        <button @click="() => editor.flipSelected(true)" title="Отразить по горизонтали">
          <i class="fas fa-arrows-left-right"></i>
        </button>
        <button @click="() => editor.flipSelected(false)" title="Отразить по вертикали">
          <i class="fas fa-arrows-up-down"></i>
        </button>
      </div>

      <div class="toolbar-group" v-if="editor.isMultiSelect">
        <span class="toolbar-label">Выровнять:</span>
        <button @click="() => editor.alignParts('left')" title="По левому краю">
          <i class="fas fa-align-left"></i>
        </button>
        <button @click="() => editor.alignParts('center')" title="По центру">
          <i class="fas fa-align-center"></i>
        </button>
        <button @click="() => editor.alignParts('right')" title="По правому краю">
          <i class="fas fa-align-right"></i>
        </button>
        <button @click="() => editor.alignParts('top')" title="По верхнему краю">
          <i class="fas fa-align-left" style="transform: rotate(90deg)"></i>
        </button>
        <button @click="() => editor.alignParts('middle')" title="По середине">
          <i class="fas fa-align-center" style="transform: rotate(90deg)"></i>
        </button>
        <button @click="() => editor.alignParts('bottom')" title="По нижнему краю">
          <i class="fas fa-align-right" style="transform: rotate(90deg)"></i>
        </button>
      </div>

      <div class="toolbar-group">
        <button @click="editor.zoomIn" title="Приблизить">
          <i class="fas fa-search-plus"></i>
        </button>
        <button @click="editor.zoomOut" title="Отдалить">
          <i class="fas fa-search-minus"></i>
        </button>
        <button @click="editor.resetView" title="Сбросить вид">
          <i class="fas fa-expand"></i>
        </button>
        <span class="zoom-indicator">{{ Math.round(editor.state.scale * 100) }}%</span>
      </div>
    </div>

    <!-- Canvas -->
    <div
      ref="canvasRef"
      class="editor-canvas"
      @mousedown="handleCanvasMouseDown"
      @mousemove="handleCanvasMouseMove"
      @mouseup="handleCanvasMouseUp"
      @mouseleave="handleCanvasMouseUp"
      @wheel="handleWheel"
      @keydown="handleKeyDown"
      tabindex="0"
    >
      <svg
        ref="svgRef"
        :width="svgWidth"
        :height="svgHeight"
        class="unfold-svg"
      >
        <!-- Сетка -->
        <defs v-if="editor.settings.showGrid">
          <pattern
            id="grid"
            :width="gridPatternSize"
            :height="gridPatternSize"
            patternUnits="userSpaceOnUse"
          >
            <path
              :d="`M ${gridPatternSize} 0 L 0 0 0 ${gridPatternSize}`"
              fill="none"
              stroke="var(--grid-color, #e0e0e0)"
              stroke-width="0.5"
            />
          </pattern>
        </defs>
        <rect
          v-if="editor.settings.showGrid"
          width="100%"
          height="100%"
          fill="url(#grid)"
        />

        <!-- Части -->
        <g
          v-for="part in editor.state.parts"
          :key="part.id"
          :class="['part-group', {
            selected: editor.selectedPartIds.includes(part.id),
            hovered: editor.state.hoveredPartId === part.id,
          }]"
          @mousedown="handlePartMouseDown($event, part.id)"
          @mouseenter="editor.state.hoveredPartId = part.id"
          @mouseleave="editor.state.hoveredPartId = null"
        >
          <!-- Контур части -->
          <path
            :d="partPath(part)"
            :class="['part-path', {
              selected: editor.selectedPartIds.includes(part.id),
            }]"
            :style="{
              stroke: getPartColor(part),
            }"
          />
          
          <!-- Заполнение -->
          <path
            :d="partPath(part)"
            class="part-fill"
            :style="{
              fill: getPartFill(part),
            }"
          />

          <!-- Номер части -->
          <text
            :x="part.center[0]"
            :y="part.center[1]"
            class="part-number"
            text-anchor="middle"
            dominant-baseline="middle"
          >
            {{ getPartNumber(part) }}
          </text>

          <!-- Bounds (в режиме отладки) -->
          <rect
            v-if="editor.settings.showBounds"
            :x="part.bounds.x"
            :y="part.bounds.y"
            :width="part.bounds.width"
            :height="part.bounds.height"
            class="part-bounds"
          />
        </g>

        <!-- Выделение рамкой -->
        <rect
          v-if="isSelecting"
          :x="selectionRect.x"
          :y="selectionRect.y"
          :width="selectionRect.width"
          :height="selectionRect.height"
          class="selection-rect"
        />
      </svg>
    </div>

    <!-- Индикатор выделенных частей -->
    <div class="selection-info" v-if="editor.hasSelection">
      <span>Выбрано: {{ editor.selectedPartIds.length }}</span>
      <button @click="editor.clearSelection" class="clear-selection">
        <i class="fas fa-times"></i>
      </button>
    </div>

    <!-- Подсказка -->
    <div class="editor-hint">
      <span v-if="!editor.settings.snapToGrid">🖱️ Перетаскивание без привязки</span>
      <span v-else>🧲 Привязка к сетке {{ editor.settings.gridSize }}мм</span>
      <span v-if="editor.isMultiSelect"> • Выделено: {{ editor.selectedPartIds.length }}</span>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'
import { useUnfoldEditor, type Part2D } from '@/composables/useUnfoldEditor'

interface Props {
  unfoldedData?: {
    faces: Array<{
      vertices_2d: number[][]
      face_id: number
    }>
  }
}

const props = withDefaults(defineProps<Props>(), {
  unfoldedData: undefined,
})

const emit = defineEmits<{
  change: [data: Part2D[]]
}>()

// Composable
const editor = useUnfoldEditor({
  gridSize: 10,
  snapToGrid: true,
  snapToParts: false,
  showGrid: true,
})

// Refs
const canvasRef = ref<HTMLElement | null>(null)
const svgRef = ref<SVGSVGElement | null>(null)

// Состояние выделения рамкой
const isSelecting = ref(false)
const selectionStart = ref({ x: 0, y: 0 })
const selectionRect = ref({ x: 0, y: 0, width: 0, height: 0 })

// Вычисляемые
const svgWidth = computed(() => 2000)
const svgHeight = computed(() => 2000)
const gridPatternSize = computed(() => editor.settings.gridSize * editor.state.scale)

// Инициализация
onMounted(() => {
  if (props.unfoldedData) {
    loadUnfoldedData(props.unfoldedData)
  }
  
  // Подписка на клавиатуру
  window.addEventListener('keydown', handleGlobalKeyDown)
})

onUnmounted(() => {
  window.removeEventListener('keydown', handleGlobalKeyDown)
})

// Загрузка данных развёртки
const loadUnfoldedData = (data: Props['unfoldedData']) => {
  if (!data) return
  
  const parts: Part2D[] = data.faces.map((face, idx) => {
    const xs = face.vertices_2d.map(v => v[0])
    const ys = face.vertices_2d.map(v => v[1])
    
    return {
      id: `face-${idx}`,
      name: `Деталь ${idx + 1}`,
      vertices: face.vertices_2d.map(v => [v[0], v[1]]),
      center: [
        xs.reduce((a, b) => a + b, 0) / xs.length,
        ys.reduce((a, b) => a + b, 0) / ys.length,
      ],
      bounds: {
        x: Math.min(...xs),
        y: Math.min(...ys),
        width: Math.max(...xs) - Math.min(...xs),
        height: Math.max(...ys) - Math.min(...ys),
      },
      rotation: 0,
      flipped: false,
    }
  })
  
  editor.loadParts(parts)
}

// Обработчики мыши
const handleCanvasMouseDown = (event: MouseEvent) => {
  if (event.button !== 0) return // Только ЛКМ
  
  const pos = getMousePosition(event)
  
  // Проверка клика на пустом месте
  const clickedOnEmpty = !editor.state.hoveredPartId
  
  if (clickedOnEmpty) {
    if (event.shiftKey) {
      // Начало выделения рамкой
      isSelecting.value = true
      selectionStart.value = pos
      selectionRect.value = { x: pos.x, y: pos.y, width: 0, height: 0 }
    } else if (!event.ctrlKey) {
      // Снять выделение
      editor.clearSelection()
    }
  }
}

const handlePartMouseDown = (event: MouseEvent, partId: string) => {
  if (event.button !== 0) return
  
  event.stopPropagation()
  const pos = getMousePosition(event)
  
  if (event.ctrlKey || event.shiftKey) {
    // Добавить к выделению
    editor.selectPart(partId, true)
  } else {
    // Начало перетаскивания
    editor.startDrag(partId, pos.x, pos.y)
  }
}

const handleCanvasMouseMove = (event: MouseEvent) => {
  const pos = getMousePosition(event)
  
  if (editor.isDragging) {
    editor.drag(pos.x, pos.y)
  } else if (isSelecting.value) {
    // Обновление рамки выделения
    selectionRect.value = {
      x: Math.min(selectionStart.value.x, pos.x),
      y: Math.min(selectionStart.value.y, pos.y),
      width: Math.abs(pos.x - selectionStart.value.x),
      height: Math.abs(pos.y - selectionStart.value.y),
    }
  }
}

const handleCanvasMouseUp = (event: MouseEvent) => {
  if (editor.isDragging) {
    editor.endDrag()
    emitChange()
  }
  
  if (isSelecting.value) {
    // Выделение рамкой
    editor.selectInRect(
      selectionRect.value.x,
      selectionRect.value.y,
      selectionRect.value.x + selectionRect.value.width,
      selectionRect.value.y + selectionRect.value.height
    )
    
    isSelecting.value = false
    selectionRect.value = { x: 0, y: 0, width: 0, height: 0 }
  }
}

const handleWheel = (event: WheelEvent) => {
  event.preventDefault()
  
  if (event.ctrlKey) {
    // Zoom
    if (event.deltaY < 0) {
      editor.zoomIn()
    } else {
      editor.zoomOut()
    }
  } else {
    // Pan
    editor.pan(-event.deltaX, -event.deltaY)
  }
}

const handleKeyDown = (event: KeyboardEvent) => {
  // Обработка горячих клавиш
  switch (event.key.toLowerCase()) {
    case 'delete':
    case 'backspace':
      // TODO: Удаление выделенных частей
      break
    case 'a':
      if (event.ctrlKey) {
        // Выделить всё
        event.preventDefault()
        editor.state.selectedPartIds = editor.state.parts.map(p => p.id)
      }
      break
    case 'escape':
      editor.clearSelection()
      break
  }
}

const handleGlobalKeyDown = (event: KeyboardEvent) => {
  // Глобальные горячие клавиши
  if ((event.ctrlKey || event.metaKey) && event.key.toLowerCase() === 'z') {
    event.preventDefault()
    if (event.shiftKey) {
      editor.redo()
    } else {
      editor.undo()
    }
  }
  
  if ((event.ctrlKey || event.metaKey) && event.key.toLowerCase() === 'y') {
    event.preventDefault()
    editor.redo()
  }
}

// Утилиты
const getMousePosition = (event: MouseEvent) => {
  const rect = canvasRef.value?.getBoundingClientRect()
  if (!rect) return { x: 0, y: 0 }
  
  return {
    x: (event.clientX - rect.left - editor.state.viewOffset.x) / editor.state.scale,
    y: (event.clientY - rect.top - editor.state.viewOffset.y) / editor.state.scale,
  }
}

const partPath = (part: Part2D) => {
  if (part.vertices.length === 0) return ''
  
  const [x, y] = part.vertices[0]
  let path = `M ${x} ${y}`
  
  for (let i = 1; i < part.vertices.length; i++) {
    const [vx, vy] = part.vertices[i]
    path += ` L ${vx} ${vy}`
  }
  
  path += ' Z'
  return path
}

const getPartColor = (part: Part2D) => {
  if (editor.selectedPartIds.includes(part.id)) {
    return editor.settings.selectionColor
  }
  return editor.settings.highlightColor
}

const getPartFill = (part: Part2D) => {
  if (editor.selectedPartIds.includes(part.id)) {
    return `${editor.settings.selectionColor}20` // 20% opacity
  }
  return 'transparent'
}

const getPartNumber = (part: Part2D) => {
  const index = editor.state.parts.findIndex(p => p.id === part.id)
  return (index + 1).toString()
}

// Настройки
const toggleSnapToGrid = () => {
  editor.setSnapToGrid(!editor.settings.snapToGrid)
}

const toggleShowGrid = () => {
  editor.settings.showGrid = !editor.settings.showGrid
}

const emitChange = () => {
  emit('change', editor.state.parts.map(p => ({ ...p })))
}

// Публичные методы
defineExpose({
  editor,
  loadUnfoldedData,
})
</script>

<style scoped>
.unfold-editor {
  display: flex;
  flex-direction: column;
  height: 100%;
  background: var(--bg-secondary, #f5f5f5);
  overflow: hidden;
}

.unfold-editor.is-dragging {
  cursor: grabbing;
}

.editor-toolbar {
  display: flex;
  gap: 8px;
  padding: 8px;
  background: var(--bg-primary, #fff);
  border-bottom: 1px solid var(--border-color, #ddd);
}

.toolbar-group {
  display: flex;
  gap: 4px;
  align-items: center;
  padding: 0 8px;
  border-right: 1px solid var(--border-color, #ddd);
}

.toolbar-group:last-child {
  border-right: none;
}

.toolbar-group button {
  display: flex;
  align-items: center;
  gap: 4px;
  padding: 6px 10px;
  border: 1px solid var(--border-color, #ddd);
  background: var(--bg-secondary, #f9f9f9);
  border-radius: 4px;
  cursor: pointer;
  font-size: 13px;
  transition: all 0.2s;
}

.toolbar-group button:hover:not(:disabled) {
  background: var(--bg-tertiary, #e5e5e5);
}

.toolbar-group button.active {
  background: var(--accent-color, #4a9eff);
  color: white;
  border-color: var(--accent-color, #4a9eff);
}

.toolbar-group button:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.toolbar-label {
  font-size: 12px;
  color: var(--text-secondary, #666);
  margin-right: 4px;
}

.zoom-indicator {
  font-size: 12px;
  color: var(--text-secondary, #666);
  min-width: 40px;
  text-align: center;
}

.editor-canvas {
  flex: 1;
  overflow: hidden;
  position: relative;
  cursor: grab;
}

.editor-canvas:focus {
  outline: none;
}

.unfold-svg {
  display: block;
}

.part-group {
  cursor: grab;
}

.part-group:hover .part-path {
  stroke-width: 2;
}

.part-group.selected .part-path {
  stroke-width: 3;
}

.part-path {
  stroke-width: 1.5;
  fill: none;
  transition: all 0.2s;
}

.part-fill {
  opacity: 0.1;
  pointer-events: none;
}

.part-number {
  font-size: 12px;
  fill: var(--text-primary, #333);
  pointer-events: none;
}

.part-bounds {
  fill: none;
  stroke: #ff0000;
  stroke-width: 0.5;
  stroke-dasharray: 2, 2;
  pointer-events: none;
}

.selection-rect {
  fill: rgba(74, 158, 255, 0.2);
  stroke: var(--accent-color, #4a9eff);
  stroke-width: 1;
  stroke-dasharray: 2, 2;
}

.selection-info {
  position: absolute;
  bottom: 50px;
  right: 10px;
  display: flex;
  align-items: center;
  gap: 8px;
  background: rgba(0, 0, 0, 0.8);
  color: white;
  padding: 8px 12px;
  border-radius: 20px;
  font-size: 13px;
}

.clear-selection {
  background: transparent;
  border: none;
  color: white;
  cursor: pointer;
  padding: 2px;
}

.editor-hint {
  position: absolute;
  bottom: 10px;
  left: 50%;
  transform: translateX(-50%);
  font-size: 12px;
  color: rgba(255, 255, 255, 0.8);
  background: rgba(0, 0, 0, 0.6);
  padding: 6px 12px;
  border-radius: 12px;
  pointer-events: none;
}
</style>
