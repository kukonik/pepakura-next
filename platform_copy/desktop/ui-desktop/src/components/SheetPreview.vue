<template>
  <div class="sheet-preview-container">
    <div class="toolbar">
      <button
        :class="{ active: isEditMode }"
        @click="toggleEditMode"
      >
        {{ isEditMode ? 'Режим редактирования' : 'Режим просмотра' }}
      </button>
    </div>
    
    <!-- Плавающая панель инструментов для выделенного объекта -->
    <Transition name="toolbar-fade">
      <div
        v-if="isEditMode && activeElementId"
        class="context-toolbar"
      >
        <button @click="rotateLeft" title="Повернуть -90°">↺</button>
        <button @click="rotateRight" title="Повернуть +90°">↻</button>
        <button @click="flipHorizontal" title="Отразить по горизонтали">⇄</button>
      </div>
    </Transition>
    
    <div
      ref="svgContainerRef"
      class="svg-container"
      :class="{ 'edit-mode': isEditMode }"
      @mousedown="handleMouseDown"
      @mousemove="handleMouseMove"
      @mouseup="handleMouseUp"
      @wheel.passive="handleWheel"
    >
      <svg
        ref="svgRef"
        v-html="svgContent"
        class="sheet-svg"
        :style="svgStyle"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted, watch } from 'vue'
import { useProjectStore } from '../stores/projectStore'

interface PartTransform {
  x: number
  y: number
  rotation: number
  scale: number
}

interface MousePosition {
  x: number
  y: number
}

// Ссылки на DOM элементы
const svgContainerRef = ref<HTMLElement | null>(null)
const svgRef = ref<SVGSVGElement | null>(null)

// Состояние режима редактирования
const isEditMode = ref(false)

// Активный элемент (перетаскиваемый)
const activeElementId = ref<string | null>(null)

// Трансформации деталей
const partTransforms = ref<Map<string, PartTransform>>(new Map())

// Позиция мыши при начале перетаскивания
const dragStartPos = ref<MousePosition>({ x: 0, y: 0 })

// Начальная трансформация элемента при начале перетаскивания
const dragStartElementTransform = ref<PartTransform>({ x: 0, y: 0, rotation: 0, scale: 1 })

// Панорамные и зумные параметры
const pan = ref({ x: 0, y: 0 })
const scale = ref(1)

// Стор
const store = useProjectStore()
const svgContent = computed(() => store.svgContent || '')

// Стили для SVG с учетом панорамирования и масштабирования
const svgStyle = computed(() => ({
  transform: `translate(${pan.value.x}px, ${pan.value.y}px) scale(${scale.value})`,
  cursor: isEditMode.value ? 'default' : 'grab'
}))

// Переключение режима редактирования
const toggleEditMode = () => {
  isEditMode.value = !isEditMode.value
  if (!isEditMode.value) {
    activeElementId.value = null
  }
  // Сохраняем состояние режима редактирования в стор
  store.setEditMode(isEditMode.value);
}

// Обработка нажатия мыши
const handleMouseDown = (event: MouseEvent) => {
  if (!svgRef.value || !svgContainerRef.value) return

  // В режиме редактирования обрабатываем перетаскивание элементов
  if (isEditMode.value) {
    const target = event.target as SVGElement
    
    // Проверяем, кликнули ли мы по элементу детали
    if (isSVGPartElement(target)) {
      event.preventDefault()
      
      // Получаем или генерируем ID элемента
      const elementId = getElementId(target)
      
      
      // Устанавливаем активный элемент
      activeElementId.value = elementId
      
      // Сохраняем выбранный элемент в стор
      store.setSelectedPartId(elementId);
      // Сохраняем начальную позицию мыши
      dragStartPos.value = { x: event.clientX, y: event.clientY }
      
      // Сохраняем начальную трансформацию элемента
      const currentTransform = partTransforms.value.get(elementId) || { x: 0, y: 0, rotation: 0, scale: 1 }
      dragStartElementTransform.value = { ...currentTransform }
      
      // Сохраняем трансформацию в стор
      store.setPartTransform({ id: elementId, transform: currentTransform });
      
      // Добавляем класс выделения
      target.classList.add('selected')
    } else {
      // Клик по пустому месту - снимаем выделение
      activeElementId.value = null
      clearSelection()
      // Очищаем выбранный элемент в сторе
      store.setSelectedPartId(null);
    }
  }
  // В режиме просмотра обрабатываем панорамирование
  else {
    // Здесь можно добавить логику панорамирования
    // Пока оставим заглушку
  }
}

// Обработка движения мыши
const handleMouseMove = (event: MouseEvent) => {
  if (!svgRef.value || !isEditMode.value || !activeElementId.value) return
  
  // Получаем активный элемент
  const element = svgRef.value.querySelector(`#${activeElementId.value}`) ||
                  svgRef.value.querySelector(`[data-id="${activeElementId.value}"]`)
  
  if (!element) return
  
  // Вычисляем дельту перемещения с учетом масштаба
  const deltaX = (event.clientX - dragStartPos.value.x) / scale.value
  const deltaY = (event.clientY - dragStartPos.value.y) / scale.value
  
  // Обновляем трансформацию элемента
  const newTransform = {
    x: dragStartElementTransform.value.x + deltaX,
    y: dragStartElementTransform.value.y + deltaY,
    rotation: dragStartElementTransform.value.rotation,
    scale: dragStartElementTransform.value.scale
  }
  
  // Сохраняем новую трансформацию
  partTransforms.value.set(activeElementId.value, newTransform)
  
  // Применяем трансформацию к элементу
  updateElementTransform(element as SVGElement, newTransform)
}

// Обработка отпускания кнопки мыши
const handleMouseUp = () => {
  // Сбрасываем активный элемент
  activeElementId.value = null
}

// Обработка колесика мыши для зума
const handleWheel = (event: WheelEvent) => {
  if (!svgContainerRef.value) return
  
  event.preventDefault()
  
  // Определяем позицию курсора относительно контейнера
  const containerRect = svgContainerRef.value.getBoundingClientRect()
  const mouseX = event.clientX - containerRect.left
  const mouseY = event.clientY - containerRect.top
  
  // Определяем направление зума
  const zoomIntensity = 0.1
  const wheelDelta = event.deltaY > 0 ? -1 : 1
  const zoomFactor = Math.exp(wheelDelta * zoomIntensity)
  
  // Вычисляем новые значения масштаба
  const newScale = scale.value * zoomFactor
  
  // Ограничиваем масштаб
  if (newScale < 0.1 || newScale > 10) return
  
  // Вычисляем новую позицию панорамирования для зума к курсору
  pan.value.x -= (mouseX - pan.value.x) * (zoomFactor - 1)
  pan.value.y -= (mouseY - pan.value.y) * (zoomFactor - 1)
  
  // Обновляем масштаб
  scale.value = newScale
}

// Проверка, является ли элемент частью SVG (деталью)
const isSVGPartElement = (element: Element): boolean => {
  return (
    element instanceof SVGElement &&
    (element.tagName === 'path' ||
     element.tagName === 'polygon' ||
     element.tagName === 'g' ||
     element.tagName === 'circle' ||
     element.tagName === 'rect' ||
     element.tagName === 'ellipse' ||
     element.tagName === 'line' ||
     element.tagName === 'polyline')
  )
}

// Получение или генерация ID элемента
const getElementId = (element: SVGElement): string => {
  // Если у элемента есть ID, используем его
  if (element.id) {
    return element.id
  }
  
  // Если нет ID, генерируем уникальный и присваиваем элементу
  const generatedId = `part-${Date.now()}-${Math.random().toString(36).substr(2, 9)}`
  element.setAttribute('data-id', generatedId)
  return generatedId
}

// Очистка выделения
const clearSelection = () => {
  if (!svgRef.value) return
  
  // Удаляем класс выделения у всех элементов
  const selectedElements = svgRef.value.querySelectorAll('.selected')
  selectedElements.forEach(el => el.classList.remove('selected'))
}

// Обновление трансформации элемента
const updateElementTransform = (element: SVGElement, transform: PartTransform) => {
  // Применяем трансформацию через CSS
  element.style.transform = `translate(${transform.x}px, ${transform.y}px) rotate(${transform.rotation}deg) scale(${transform.scale}, 1)`
  element.style.transformOrigin = 'center center'
}

// Наблюдение за изменением контента SVG
watch(svgContent, () => {
  // При изменении контента SVG сбрасываем трансформации
  partTransforms.value.clear()
  activeElementId.value = null
  clearSelection()
})

// Добавляем слушатель клавиатурных событий при монтировании
onMounted(() => {
  window.addEventListener('keydown', handleKeyDown)
})

// Очистка при размонтировании компонента
onUnmounted(() => {
  partTransforms.value.clear()
  window.removeEventListener('keydown', handleKeyDown)
})

// Функции для вращения и отражения
const rotateLeft = () => {
  if (!activeElementId.value) return
  
  const currentTransform = partTransforms.value.get(activeElementId.value) || { x: 0, y: 0, rotation: 0, scale: 1 }
  const newTransform = {
    ...currentTransform,
    rotation: currentTransform.rotation - 90
  }
  
  partTransforms.value.set(activeElementId.value, newTransform)
  
  const element = svgRef.value?.querySelector(`#${activeElementId.value}`) ||
                  svgRef.value?.querySelector(`[data-id="${activeElementId.value}"]`)
  if (element) {
    updateElementTransform(element as SVGElement, newTransform)
  }
  
  // Сохраняем трансформацию в стор
  store.setPartTransform({ id: activeElementId.value, transform: newTransform });
  
  // Обновляем начальную трансформацию для корректной работы drag
  dragStartElementTransform.value = { ...newTransform }
}

const rotateRight = () => {
  if (!activeElementId.value) return
  
  const currentTransform = partTransforms.value.get(activeElementId.value) || { x: 0, y: 0, rotation: 0, scale: 1 }
  const newTransform = {
    ...currentTransform,
    rotation: currentTransform.rotation + 90
  }
  
  partTransforms.value.set(activeElementId.value, newTransform)
  
  const element = svgRef.value?.querySelector(`#${activeElementId.value}`) ||
                  svgRef.value?.querySelector(`[data-id="${activeElementId.value}"]`)
  if (element) {
    updateElementTransform(element as SVGElement, newTransform)
  }
  
  // Сохраняем трансформацию в стор
  store.setPartTransform({ id: activeElementId.value, transform: newTransform });
  
  // Обновляем начальную трансформацию для корректной работы drag
  dragStartElementTransform.value = { ...newTransform }
}

const flipHorizontal = () => {
  if (!activeElementId.value) return
  
  const currentTransform = partTransforms.value.get(activeElementId.value) || { x: 0, y: 0, rotation: 0, scale: 1 }
  const newTransform = {
    ...currentTransform,
    scale: currentTransform.scale === 1 ? -1 : 1
  }
  
  partTransforms.value.set(activeElementId.value, newTransform)
  
  const element = svgRef.value?.querySelector(`#${activeElementId.value}`) ||
                  svgRef.value?.querySelector(`[data-id="${activeElementId.value}"]`)
  if (element) {
    updateElementTransform(element as SVGElement, newTransform)
  }
  
  // Сохраняем трансформацию в стор
  store.setPartTransform({ id: activeElementId.value, transform: newTransform });
  
  // Обновляем начальную трансформацию для корректной работы drag
  dragStartElementTransform.value = { ...newTransform }
}

// Обработка клавиатурных событий
const handleKeyDown = (event: KeyboardEvent) => {
  // Проверяем, что активен режим редактирования и выбран элемент
  if (!isEditMode.value || !activeElementId.value) return
  
  // Обрабатываем клавиши
  switch (event.key) {
    case 'ArrowLeft':
      event.preventDefault()
      rotateLeft()
      break
    case 'ArrowRight':
      event.preventDefault()
      rotateRight()
      break
    case 'f':
    case 'F':
      event.preventDefault()
      flipHorizontal()
      break
  }
}
</script>

<style scoped>
.sheet-preview-container {
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  position: relative;
}

.toolbar {
  padding: 10px;
  background-color: #f5f5f5;
  border-bottom: 1px solid #ddd;
  display: flex;
  align-items: center;
  gap: 10px;
}

.toolbar button {
  padding: 8px 16px;
  border: 1px solid #ccc;
  background-color: #fff;
  border-radius: 4px;
  cursor: pointer;
  font-size: 14px;
}

.toolbar button.active {
  background-color: #007bff;
  color: white;
  border-color: #007bff;
}

.transform-buttons {
  display: flex;
  gap: 5px;
}

.transform-buttons.disabled {
  opacity: 0.5;
  pointer-events: none;
}

.transform-buttons button {
  padding: 8px 12px;
  border: 1px solid #ccc;
  background-color: #fff;
  border-radius: 4px;
  cursor: pointer;
  font-size: 14px;
}

/* Стили для плавающей панели инструментов */
.context-toolbar {
  position: absolute;
  top: 20px;
  right: 20px;
  display: flex;
  gap: 5px;
  background-color: #2c2c2c;
  padding: 8px;
  border-radius: 8px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
  z-index: 100;
}

.context-toolbar button {
  padding: 6px 10px;
  border: none;
  background-color: #444;
  color: white;
  border-radius: 4px;
  cursor: pointer;
  font-size: 14px;
  transition: background-color 0.2s;
}

.context-toolbar button:hover {
  background-color: #555;
}

/* Анимация появления/исчезновения панели */
.toolbar-fade-enter-active,
.toolbar-fade-leave-active {
  transition: opacity 0.3s, transform 0.3s;
}

.toolbar-fade-enter-from,
.toolbar-fade-leave-to {
  opacity: 0;
  transform: translateY(-20px);
}

.svg-container {
  flex: 1;
  overflow: hidden;
  position: relative;
  background-color: #f0f0f0;
  cursor: grab;
}

.svg-container.edit-mode {
  cursor: default;
}

.sheet-svg {
  width: 100%;
  height: 100%;
  transform-origin: 0 0;
}

/* Стили для выделенного элемента */
.selected {
  stroke: #00ff00 !important;
  stroke-width: 2px !important;
  outline: 2px solid #00ff00;
  outline-offset: 1px;
}

/* Стили для элементов в режиме редактирования */
.edit-mode .sheet-svg path,
.edit-mode .sheet-svg polygon,
.edit-mode .sheet-svg g,
.edit-mode .sheet-svg circle,
.edit-mode .sheet-svg rect,
.edit-mode .sheet-svg ellipse,
.edit-mode .sheet-svg line,
.edit-mode .sheet-svg polyline {
  cursor: move;
}
</style>