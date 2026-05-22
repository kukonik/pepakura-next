<template>
  <div class="nesting-editor">
    <div class="editor-header">
      <h3>Редактор размещения</h3>
      <div class="editor-controls">
        <button @click="undo" :disabled="!canUndo" class="secondary-button">↩ Отменить</button>
        <button @click="redo" :disabled="!canRedo" class="secondary-button">↪ Повторить</button>
        <button @click="saveChanges" class="primary-button">Сохранить</button>
        <button @click="cancelEditing" class="secondary-button">Отмена</button>
        <button @click="returnToMain" class="secondary-button">Назад</button>
        <div v-if="projectStore.isDirty" class="auto-save-indicator">
          Автосохранение...
        </div>
      </div>
    </div>
    
    <div class="editor-content">
      <div class="sheet-selector">
        <label for="sheetSelect">Лист:</label>
        <select id="sheetSelect" v-model="selectedSheetIndex">
          <option v-for="(sheet, index) in nestResult.sheets" :key="index" :value="index">
            Лист {{ index + 1 }} ({{ sheet.parts.length }} частей)
          </option>
        </select>
      </div>
      
      <div class="sheet-container">
        <svg
          ref="svgElement"
          :viewBox="`0 0 ${sheetWidth} ${sheetHeight}`"
          class="sheet-svg"
          @mousedown="startDrag"
          @mousemove="onDrag"
          @mouseup="endDrag"
          @mouseleave="endDrag"
        >
          <!-- Фон листа -->
          <rect
            x="0"
            y="0"
            :width="sheetWidth"
            :height="sheetHeight"
            fill="white"
            stroke="black"
            stroke-width="0.5"
          />
          
          <!-- Отступы -->
          <rect
            :x="margin"
            :y="margin"
            :width="sheetWidth - 2 * margin"
            :height="sheetHeight - 2 * margin"
            fill="none"
            stroke="lightgray"
            stroke-width="0.2"
            stroke-dasharray="2,2"
          />
          
          <!-- Части на листе -->
          <g v-for="part in displayedParts" :key="part.id">
            <g
              :transform="`translate(${part.x_mm}, ${part.y_mm}) rotate(${part.rotation})`"
              :class="{
                'part-element': true,
                'selected': selectedPartId === part.id,
                'out-of-bounds': isOutOfBounds(part),
                'colliding': hasCollisions(part)
              }"
              @mousedown="selectPart(part.id, $event)"
            >
              <!-- Визуальное представление части (заглушка) -->
              <rect
                :width="part.width_mm"
                :height="part.height_mm"
                fill="lightblue"
                stroke="darkblue"
                stroke-width="0.2"
              />
              <text
                :x="part.width_mm / 2"
                :y="part.height_mm / 2"
                text-anchor="middle"
                dominant-baseline="middle"
                font-size="3"
              >
                {{ part.id }}
              </text>
            </g>
          </g>
        </svg>
      </div>
      
      <div class="part-info" v-if="selectedPart">
        <h4>Информация о части</h4>
        <p>ID: {{ selectedPart.id }}</p>
        <p>Позиция: ({{ selectedPart.x_mm.toFixed(2) }}, {{ selectedPart.y_mm.toFixed(2) }}) мм</p>
        <p>Размер: {{ selectedPart.width_mm.toFixed(2) }} × {{ selectedPart.height_mm.toFixed(2) }} мм</p>
        <p>Поворот: {{ selectedPart.rotation }}°</p>
        
        <div class="part-controls">
          <button @click="rotatePart(-15)" class="secondary-button">↺ -15°</button>
          <button @click="rotatePart(15)" class="secondary-button">↻ +15°</button>
          <button @click="resetPartPosition" class="secondary-button">Сброс</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'
import { useProjectStore } from '@/stores/projectStore'
import { useUndoRedoStore } from '@/stores/undoRedoStore'
import { NestPart, PartOverride } from '@/../../shared/src/types/nesting'

const projectStore = useProjectStore()
const undoRedoStore = useUndoRedoStore()

// Состояние компонента
const svgElement = ref<SVGSVGElement | null>(null)
const selectedSheetIndex = ref(0)
const selectedPartId = ref<number | null>(null)
const isDragging = ref(false)
const dragStartX = ref(0)
const dragStartY = ref(0)
const originalPartX = ref(0)
const originalPartY = ref(0)

// Вычисляемые свойства
const nestResult = computed(() => projectStore.nestResult)
const sheet = computed(() => nestResult.value?.sheets[selectedSheetIndex.value])
const sheetWidth = computed(() => sheet.value?.width_mm || 210)
const sheetHeight = computed(() => sheet.value?.height_mm || 297)
const margin = computed(() => sheet.value?.margin_mm || 5)

const displayedParts = computed(() => {
  if (!sheet.value) return []
  
  return sheet.value.parts.map(part => {
    const override = projectStore.getPartOverride(part.id)
    if (!override) return part
    
    return {
      ...part,
      x_mm: part.x_mm + (override.deltaX || 0),
      y_mm: part.y_mm + (override.deltaY || 0),
      rotation: part.rotation + (override.deltaRotation || 0)
    }
  })
})

// Вычисляемые свойства для Undo/Redo
const canUndo = computed(() => undoRedoStore.canUndo)
const canRedo = computed(() => undoRedoStore.canRedo)

const selectedPart = computed(() => {
  if (selectedPartId.value === null) return null
  return displayedParts.value.find(part => part.id === selectedPartId.value) || null
})

// Методы
const selectPart = (partId: number, event: MouseEvent) => {
  event.stopPropagation()
  selectedPartId.value = partId
}

const startDrag = (event: MouseEvent) => {
  if (!selectedPartId.value || !selectedPart.value) return
  
  isDragging.value = true
  dragStartX.value = event.clientX
  dragStartY.value = event.clientY
  originalPartX.value = selectedPart.value.x_mm
  originalPartY.value = selectedPart.value.y_mm
}

const onDrag = (event: MouseEvent) => {
  if (!isDragging.value || !selectedPartId.value || !svgElement.value) return
  
  // Получаем размеры SVG элемента
  const svgRect = svgElement.value.getBoundingClientRect()
  const svgWidth = svgRect.width
  const svgHeight = svgRect.height
  
  // Преобразуем координаты мыши в координаты SVG
  const mouseX = event.clientX - svgRect.left
  const mouseY = event.clientY - svgRect.top
  
  // Преобразуем координаты в мм
  const x_mm = (mouseX / svgWidth) * sheetWidth.value
  const y_mm = (mouseY / svgHeight) * sheetHeight.value
  
  // Вычисляем смещение относительно начальной позиции
  const deltaX = x_mm - originalPartX.value
  const deltaY = y_mm - originalPartY.value
  
  // Получаем предыдущее переопределение
  const previousOverride = projectStore.getPartOverride(selectedPartId.value)
  
  // Обновляем позицию части через переопределение
  const override: PartOverride = {
    partId: selectedPartId.value,
    deltaX: deltaX,
    deltaY: deltaY,
    isManual: true
  }
  
  projectStore.setPartOverride(override)
  
  // Добавляем действие в историю Undo/Redo
  undoRedoStore.addPartOverrideAction(selectedPartId.value, previousOverride, override)
}

const endDrag = () => {
  isDragging.value = false
}

const rotatePart = (deltaRotation: number) => {
  if (!selectedPartId.value) return
  
  // Получаем предыдущее переопределение
  const previousOverride = projectStore.getPartOverride(selectedPartId.value)
  const currentDeltaRotation = previousOverride?.deltaRotation || 0
  
  const override: PartOverride = {
    partId: selectedPartId.value,
    deltaRotation: currentDeltaRotation + deltaRotation,
    isManual: true
  }
  
  projectStore.setPartOverride(override)
  
  // Добавляем действие в историю Undo/Redo
  undoRedoStore.addPartOverrideAction(selectedPartId.value, previousOverride, override)
}

const resetPartPosition = () => {
  if (!selectedPartId.value) return
  
  // Получаем предыдущее переопределение
  const previousOverride = projectStore.getPartOverride(selectedPartId.value)
  
  projectStore.removePartOverride(selectedPartId.value)
  
  // Добавляем действие в историю Undo/Redo
  const newOverride: PartOverride = {
    partId: selectedPartId.value,
    isManual: true
  }
  undoRedoStore.addPartOverrideAction(selectedPartId.value, previousOverride, newOverride)
}

// Проверка, что часть выходит за границы листа

// Проверка пересечения двух частей с учетом поворота
const isIntersecting = (part1: NestPart, part2: NestPart) => {
  // Для упрощения считаем, что части пересекаются, если их ограничивающие прямоугольники пересекаются
  // Более точная проверка может потребовать алгоритмов обнаружения пересечения полигонов
  return (
    part1.x_mm < part2.x_mm + part2.width_mm &&
    part1.x_mm + part1.width_mm > part2.x_mm &&
    part1.y_mm < part2.y_mm + part2.height_mm &&
    part1.y_mm + part1.height_mm > part2.y_mm
  )
}

// Проверка коллизий для выбранной части
const hasCollisions = (part: NestPart) => {
  if (!sheet.value) return false
  
  // Проверяем пересечения с другими частями на листе
  return displayedParts.value.some(otherPart => {
    // Пропускаем саму себя
    if (otherPart.id === part.id) return false
    
    // Проверяем пересечение
    return isIntersecting(part, otherPart)
  })
}

// Функции Undo/Redo
const undo = () => {
  undoRedoStore.undo()
}

const redo = () => {
  undoRedoStore.redo()
}

const saveChanges = async () => {
  // Отправить изменения в Rust
  if (projectStore.nestResult && projectStore.currentProject) {
    // Создаем параметры размещения из снимка параметров
    const params = projectStore.nestResult.paramsSnapshot;
    
    // Преобразуем overrides в формат, ожидаемый Rust
    const overrides = Object.values(projectStore.partOverrides).map(override => ({
      part_id: override.partId,
      delta_x: override.deltaX,
      delta_y: override.deltaY,
      delta_rotation: override.deltaRotation,
      is_manual: override.isManual
    }));
    
    try {
      // Вызываем Tauri команду для применения переопределений
      const adjustedResult = await invoke<NestResult>('apply_nest_overrides', {
        project: projectStore.currentProject,
        params,
        overrides
      });
      
      // Обновляем результат размещения в хранилище
      projectStore.nestResult = adjustedResult;
      
      // Сбрасываем переопределения после успешного сохранения
      projectStore.resetPartOverrides();
    } catch (error) {
      console.error('Failed to apply overrides:', error);
      // TODO: Показать ошибку пользователю
    }
  }
}

const cancelEditing = () => {
  projectStore.resetPartOverrides()
  returnToMain()
}

const returnToMain = () => {
  // Возврат к основному интерфейсу
  window.location.hash = '#/editor'
}

// Обработчики событий
const handleKeyDown = (event: KeyboardEvent) => {
  if (selectedPartId.value === null) return
  
  // Обрабатываем комбинации клавиш для Undo/Redo
  if (event.ctrlKey || event.metaKey) {
    if (event.key === 'z') {
      event.preventDefault()
      if (event.shiftKey) {
        // Ctrl+Shift+Z или Cmd+Shift+Z - повторить
        redo()
      } else {
        // Ctrl+Z или Cmd+Z - отменить
        undo()
      }
      return
    }
  }
  
  switch (event.key) {
    case 'ArrowLeft':
      // Получаем предыдущее переопределение
      const previousOverrideLeft = projectStore.getPartOverride(selectedPartId.value)
      projectStore.setPartOverride({
        partId: selectedPartId.value,
        deltaX: (projectStore.getPartOverride(selectedPartId.value)?.deltaX || 0) - 1,
        isManual: true
      })
      // Добавляем действие в историю Undo/Redo
      const newOverrideLeft: PartOverride = {
        partId: selectedPartId.value,
        deltaX: (projectStore.getPartOverride(selectedPartId.value)?.deltaX || 0) - 1,
        isManual: true
      }
      undoRedoStore.addPartOverrideAction(selectedPartId.value, previousOverrideLeft, newOverrideLeft)
      break
    case 'ArrowRight':
      // Получаем предыдущее переопределение
      const previousOverrideRight = projectStore.getPartOverride(selectedPartId.value)
      projectStore.setPartOverride({
        partId: selectedPartId.value,
        deltaX: (projectStore.getPartOverride(selectedPartId.value)?.deltaX || 0) + 1,
        isManual: true
      })
      // Добавляем действие в историю Undo/Redo
      const newOverrideRight: PartOverride = {
        partId: selectedPartId.value,
        deltaX: (projectStore.getPartOverride(selectedPartId.value)?.deltaX || 0) + 1,
        isManual: true
      }
      undoRedoStore.addPartOverrideAction(selectedPartId.value, previousOverrideRight, newOverrideRight)
      break
    case 'ArrowUp':
      // Получаем предыдущее переопределение
      const previousOverrideUp = projectStore.getPartOverride(selectedPartId.value)
      projectStore.setPartOverride({
        partId: selectedPartId.value,
        deltaY: (projectStore.getPartOverride(selectedPartId.value)?.deltaY || 0) - 1,
        isManual: true
      })
      // Добавляем действие в историю Undo/Redo
      const newOverrideUp: PartOverride = {
        partId: selectedPartId.value,
        deltaY: (projectStore.getPartOverride(selectedPartId.value)?.deltaY || 0) - 1,
        isManual: true
      }
      undoRedoStore.addPartOverrideAction(selectedPartId.value, previousOverrideUp, newOverrideUp)
      break
    case 'ArrowDown':
      // Получаем предыдущее переопределение
      const previousOverrideDown = projectStore.getPartOverride(selectedPartId.value)
      projectStore.setPartOverride({
        partId: selectedPartId.value,
        deltaY: (projectStore.getPartOverride(selectedPartId.value)?.deltaY || 0) + 1,
        isManual: true
      })
      // Добавляем действие в историю Undo/Redo
      const newOverrideDown: PartOverride = {
        partId: selectedPartId.value,
        deltaY: (projectStore.getPartOverride(selectedPartId.value)?.deltaY || 0) + 1,
        isManual: true
      }
      undoRedoStore.addPartOverrideAction(selectedPartId.value, previousOverrideDown, newOverrideDown)
      break
    case 'r':
    case 'R':
      rotatePart(15)
      break
    case 'Escape':
      selectedPartId.value = null
      break
  }
}

onMounted(() => {
  window.addEventListener('keydown', handleKeyDown)
})

onUnmounted(() => {
  window.removeEventListener('keydown', handleKeyDown)
})
</script>

<style scoped>
.nesting-editor {
  display: flex;
  flex-direction: column;
  height: 100%;
}

.editor-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 10px 20px;
  border-bottom: 1px solid var(--border-color, #334155);
}

.editor-content {
  display: flex;
  flex: 1;
  padding: 20px;
  gap: 20px;
}

.sheet-selector {
  margin-bottom: 15px;
}

.sheet-selector label {
  margin-right: 10px;
}

.sheet-container {
  flex: 1;
  display: flex;
  flex-direction: column;
}

.sheet-svg {
  flex: 1;
  border: 1px solid var(--border-color, #334155);
  border-radius: 4px;
  background-color: white;
}

.part-element {
  cursor: pointer;
  transition: transform 0.1s;
}

.part-element:hover {
  filter: brightness(0.9);
}

.part-element.selected {
  filter: brightness(1.2);
  outline: 2px solid var(--primary-color, #6366f1);
  outline-offset: 1px;
}

.part-element.out-of-bounds {
  fill: #ff6b6b;
}

.part-element.colliding {
  fill: #ffcc00;
}

.part-info {
  width: 250px;
  padding: 15px;
  background-color: var(--panel-bg, #1e293b);
  border-radius: 4px;
  border: 1px solid var(--border-color, #334155);
}

.part-info h4 {
  margin-top: 0;
  color: var(--text-primary, #f8fafc);
}

.part-controls {
  display: flex;
  gap: 10px;
  margin-top: 15px;
}

.primary-button,
.secondary-button {
  padding: 8px 12px;
  border: none;
  border-radius: 4px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: background-color 0.2s;
}

.primary-button {
  background-color: var(--primary-color, #6366f1);
  color: white;
}

.primary-button:hover:not(:disabled) {
  background-color: var(--primary-color-dark, #4f46e5);
}

.secondary-button {
  background-color: var(--secondary-button-bg, #334155);
  color: var(--text-primary, #f8fafc);
}

.secondary-button:hover {
  background-color: var(--secondary-button-hover-bg, #475569);
}

.auto-save-indicator {
  padding: 5px 10px;
  background-color: var(--warning-color, #f59e0b);
  color: white;
  border-radius: 4px;
  font-size: 12px;
  animation: pulse 1s infinite;
  display: flex;
  align-items: center;
  margin-left: 10px;
}

@keyframes pulse {
  0% { opacity: 1; }
  50% { opacity: 0.7; }
  100% { opacity: 1; }
}
</style>