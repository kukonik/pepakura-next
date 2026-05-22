/**
 * Composable для редактирования развёрток.
 * 
 * Предоставляет:
 * - Перемещение деталей (drag & drop)
 * - Snap-to-grid
 * - Поворот деталей (90°, 180°, 270°)
 * - Выравнивание (left, right, center, top, bottom, middle)
 * - Выделение нескольких деталей
 * - Групповое перемещение
 * - Отмена/повтор действий (undo/redo)
 */

import { ref, reactive, computed, type Ref } from 'vue'

export interface Part2D {
  id: string
  name?: string
  vertices: number[][]
  center: [number, number]
  bounds: { x: number, y: number, width: number, height: number }
  rotation: number
  flipped: boolean
}

export interface EditorSettings {
  gridSize: number
  snapToGrid: boolean
  snapToParts: boolean
  showGrid: boolean
  showBounds: boolean
  highlightColor: string
  selectionColor: string
}

export interface UnfoldEditorState {
  parts: Part2D[]
  selectedPartIds: string[]
  hoveredPartId: string | null
  isDragging: boolean
  dragOffset: { x: number, y: number }
  viewOffset: { x: number, y: number }
  scale: number
}

export function useUnfoldEditor(initialSettings?: Partial<EditorSettings>) {
  // Настройки
  const settings = reactive<EditorSettings>({
    gridSize: 10,
    snapToGrid: true,
    snapToParts: false,
    showGrid: true,
    showBounds: false,
    highlightColor: '#4a9eff',
    selectionColor: '#ff6b6b',
    ...initialSettings,
  })

  // Состояние редактора
  const state = reactive<UnfoldEditorState>({
    parts: [],
    selectedPartIds: [],
    hoveredPartId: null,
    isDragging: false,
    dragOffset: { x: 0, y: 0 },
    viewOffset: { x: 0, y: 0 },
    scale: 1,
  })

  // История для undo/redo
  const undoStack = reactive<Array<{ parts: Part2D[] }>>([])
  const redoStack = reactive<Array<{ parts: Part2D[] }>>([])
  const maxHistorySize = 50

  // Вычисляемые свойства
  const selectedParts = computed(() => 
    state.parts.filter(p => state.selectedPartIds.includes(p.id))
  )

  const hasSelection = computed(() => state.selectedPartIds.length > 0)
  const isMultiSelect = computed(() => state.selectedPartIds.length > 1)

  // Инициализация
  const loadParts = (parts: Part2D[]) => {
    pushUndoState()
    state.parts = parts.map(p => ({ ...p }))
  }

  // Сохранение состояния для undo
  const pushUndoState = () => {
    undoStack.push({
      parts: state.parts.map(p => ({ ...p, vertices: p.vertices.map(v => [...v]) })),
    })
    
    // Ограничиваем размер истории
    if (undoStack.length > maxHistorySize) {
      undoStack.shift()
    }
    
    // Очищаем redo при новом действии
    redoStack.length = 0
  }

  // Отмена действия
  const undo = () => {
    if (undoStack.length === 0) return
    
    // Сохраняем текущее состояние в redo
    redoStack.push({
      parts: state.parts.map(p => ({ ...p, vertices: p.vertices.map(v => [...v]) })),
    })
    
    // Восстанавливаем из undo
    const prevState = undoStack.pop()!
    state.parts = prevState.parts.map(p => ({ ...p }))
  }

  // Повтор действия
  const redo = () => {
    if (redoStack.length === 0) return
    
    // Сохраняем текущее состояние в undo
    undoStack.push({
      parts: state.parts.map(p => ({ ...p, vertices: p.vertices.map(v => [...v]) })),
    })
    
    // Восстанавливаем из redo
    const nextState = redoStack.pop()!
    state.parts = nextState.parts.map(p => ({ ...p }))
  }

  // Выделение части
  const selectPart = (partId: string, addToSelection = false) => {
    if (addToSelection) {
      if (!state.selectedPartIds.includes(partId)) {
        state.selectedPartIds.push(partId)
      }
    } else {
      state.selectedPartIds = [partId]
    }
  }

  // Снять выделение
  const deselectPart = (partId: string) => {
    state.selectedPartIds = state.selectedPartIds.filter(id => id !== partId)
  }

  // Снять все выделения
  const clearSelection = () => {
    state.selectedPartIds = []
  }

  // Выделение рамкой
  const selectInRect = (x1: number, y1: number, x2: number, y2: number) => {
    const minX = Math.min(x1, x2)
    const maxX = Math.max(x1, x2)
    const minY = Math.min(y1, y2)
    const maxY = Math.max(y1, y2)
    
    state.selectedPartIds = state.parts
      .filter(p => {
        const [cx, cy] = p.center
        return cx >= minX && cx <= maxX && cy >= minY && cy <= maxY
      })
      .map(p => p.id)
  }

  // Начало перетаскивания
  const startDrag = (partId: string, mouseX: number, mouseY: number) => {
    const part = state.parts.find(p => p.id === partId)
    if (!part) return
    
    state.isDragging = true
    state.dragOffset = {
      x: mouseX - part.center[0],
      y: mouseY - part.center[1],
    }
    
    // Выделяем часть если ещё не выделена
    if (!state.selectedPartIds.includes(partId)) {
      selectPart(partId)
    }
    
    pushUndoState()
  }

  // Перетаскивание
  const drag = (mouseX: number, mouseY: number) => {
    if (!state.isDragging) return
    
    const targetX = mouseX - state.dragOffset.x
    const targetY = mouseY - state.dragOffset.y
    
    // Перемещаем все выделенные части
    state.selectedPartIds.forEach(partId => {
      const part = state.parts.find(p => p.id === partId)
      if (!part) return
      
      let newX = targetX
      let newY = targetY
      
      // Snap to grid
      if (settings.snapToGrid) {
        newX = snapToGrid(newX, settings.gridSize)
        newY = snapToGrid(newY, settings.gridSize)
      }
      
      // Snap to parts
      if (settings.snapToParts) {
        const snapped = snapToOtherParts(part, newX, newY)
        if (snapped) {
          newX = snapped.x
          newY = snapped.y
        }
      }
      
      // Вычисляем смещение
      const dx = newX - part.center[0]
      const dy = newY - part.center[1]
      
      // Перемещаем часть
      part.center = [newX, newY]
      part.vertices = part.vertices.map(v => [v[0] + dx, v[1] + dy])
      part.bounds.x += dx
      part.bounds.y += dy
    })
  }

  // Конец перетаскивания
  const endDrag = () => {
    state.isDragging = false
  }

  // Snap to grid
  const snapToGrid = (value: number, gridSize: number): number => {
    return Math.round(value / gridSize) * gridSize
  }

  // Snap to other parts
  const snapToOtherParts = (
    part: Part2D,
    x: number,
    y: number,
    snapDistance: number = 5
  ): { x: number, y: number } | null => {
    const newBounds = {
      x,
      y,
      width: part.bounds.width,
      height: part.bounds.height,
    }
    
    for (const other of state.parts) {
      if (other.id === part.id) continue
      
      // Проверяем близость к границам
      const snaps = [
        { x: other.bounds.x - newBounds.width, y: other.bounds.y }, // Left
        { x: other.bounds.x + other.bounds.width, y: other.bounds.y }, // Right
        { x: other.bounds.x, y: other.bounds.y - newBounds.height }, // Top
        { x: other.bounds.x, y: other.bounds.y + other.bounds.height }, // Bottom
      ]
      
      for (const snap of snaps) {
        const dx = Math.abs(snap.x - x)
        const dy = Math.abs(snap.y - y)
        
        if (dx < snapDistance && dy < snapDistance) {
          return { x: snap.x, y: snap.y }
        }
      }
    }
    
    return null
  }

  // Поворот части
  const rotatePart = (partId: string, angle: number) => {
    const part = state.parts.find(p => p.id === partId)
    if (!part) return
    
    pushUndoState()
    
    const [cx, cy] = part.center
    const rad = (angle * Math.PI) / 180
    const cos = Math.cos(rad)
    const sin = Math.sin(rad)
    
    // Поворачиваем вершины
    part.vertices = part.vertices.map(v => {
      const dx = v[0] - cx
      const dy = v[1] - cy
      return [
        cx + dx * cos - dy * sin,
        cy + dx * sin + dy * cos,
      ]
    })
    
    part.rotation = (part.rotation + angle) % 360
    
    // Пересчитываем bounds
    updatePartBounds(part)
  }

  // Поворот выделенных частей
  const rotateSelected = (angle: number) => {
    state.selectedPartIds.forEach(id => rotatePart(id, angle))
  }

  // Отражение части
  const flipPart = (partId: string, horizontal: boolean) => {
    const part = state.parts.find(p => p.id === partId)
    if (!part) return
    
    pushUndoState()
    
    const [cx, cy] = part.center
    
    part.vertices = part.vertices.map(v => {
      if (horizontal) {
        return [cx - (v[0] - cx), v[1]]
      } else {
        return [v[0], cy - (v[1] - cy)]
      }
    })
    
    part.flipped = !part.flipped
    updatePartBounds(part)
  }

  // Отражение выделенных частей
  const flipSelected = (horizontal: boolean) => {
    state.selectedPartIds.forEach(id => flipPart(id, horizontal))
  }

  // Обновление bounds части
  const updatePartBounds = (part: Part2D) => {
    const xs = part.vertices.map(v => v[0])
    const ys = part.vertices.map(v => v[1])
    
    part.bounds = {
      x: Math.min(...xs),
      y: Math.min(...ys),
      width: Math.max(...xs) - Math.min(...xs),
      height: Math.max(...ys) - Math.min(...ys),
    }
  }

  // Выравнивание
  const alignParts = (alignment: 'left' | 'right' | 'top' | 'bottom' | 'center' | 'middle') => {
    if (state.selectedPartIds.length < 2) return
    
    pushUndoState()
    
    const selected = selectedParts.value
    const bounds = getCombinedBounds(selected)
    
    selected.forEach(part => {
      let newX = part.center[0]
      let newY = part.center[1]
      
      switch (alignment) {
        case 'left':
          newX = bounds.x + part.bounds.width / 2
          break
        case 'right':
          newX = bounds.x + bounds.width - part.bounds.width / 2
          break
        case 'top':
          newY = bounds.y + part.bounds.height / 2
          break
        case 'bottom':
          newY = bounds.y + bounds.height - part.bounds.height / 2
          break
        case 'center':
          newX = bounds.x + bounds.width / 2
          break
        case 'middle':
          newY = bounds.y + bounds.height / 2
          break
      }
      
      const dx = newX - part.center[0]
      const dy = newY - part.center[1]
      
      part.center = [newX, newY]
      part.vertices = part.vertices.map(v => [v[0] + dx, v[1] + dy])
      part.bounds.x += dx
      part.bounds.y += dy
    })
  }

  // Получение общих bounds
  const getCombinedBounds = (parts: Part2D[]) => {
    if (parts.length === 0) return { x: 0, y: 0, width: 0, height: 0 }
    
    const xs = parts.flatMap(p => p.vertices.map(v => v[0]))
    const ys = parts.flatMap(p => p.vertices.map(v => v[1]))
    
    return {
      x: Math.min(...xs),
      y: Math.min(...ys),
      width: Math.max(...xs) - Math.min(...xs),
      height: Math.max(...ys) - Math.min(...ys),
    }
  }

  // Масштабирование
  const zoomIn = () => {
    state.scale = Math.min(state.scale * 1.2, 5)
  }

  const zoomOut = () => {
    state.scale = Math.max(state.scale / 1.2, 0.2)
  }

  const resetZoom = () => {
    state.scale = 1
  }

  // Панорамирование
  const pan = (dx: number, dy: number) => {
    state.viewOffset.x += dx
    state.viewOffset.y += dy
  }

  const resetView = () => {
    state.viewOffset = { x: 0, y: 0 }
    state.scale = 1
  }

  // Экспорт состояния
  const exportState = () => {
    return {
      parts: state.parts.map(p => ({ ...p })),
      settings: { ...settings },
    }
  }

  // Импорт состояния
  const importState = (exported: { parts: Part2D[], settings?: Partial<EditorSettings> }) => {
    pushUndoState()
    state.parts = exported.parts.map(p => ({ ...p }))
    if (exported.settings) {
      Object.assign(settings, exported.settings)
    }
  }

  return {
    // State
    settings,
    state,
    selectedParts,
    hasSelection,
    isMultiSelect,
    
    // Undo/Redo
    undo,
    redo,
    canUndo: computed(() => undoStack.length > 0),
    canRedo: computed(() => redoStack.length > 0),
    
    // Selection
    selectPart,
    deselectPart,
    clearSelection,
    selectInRect,
    
    // Drag & Drop
    startDrag,
    drag,
    endDrag,
    
    // Transform
    rotatePart,
    rotateSelected,
    flipPart,
    flipSelected,
    alignParts,
    
    // View
    zoomIn,
    zoomOut,
    resetZoom,
    pan,
    resetView,
    
    // Settings
    setSnapToGrid: (enabled: boolean) => { settings.snapToGrid = enabled },
    setGridSize: (size: number) => { settings.gridSize = size },
    setSnapToParts: (enabled: boolean) => { settings.snapToParts = enabled },
    
    // Utils
    loadParts,
    exportState,
    importState,
    getCombinedBounds,
  }
}
