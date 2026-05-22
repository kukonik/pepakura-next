<template>
  <div class="paper-nester">
    <div class="controls">
      <div class="paper-size-selector">
        <label>Размер бумаги:</label>
        <select v-model="selectedPaperSize" @change="onPaperSizeChange">
          <option v-for="size in paperSizes" :key="size.id" :value="size.id">
            {{ size.name }} ({{ size.width }}x{{ size.height }} мм)
          </option>
        </select>
        <div class="custom-size" v-if="selectedPaperSize === 'custom'">
          <input type="number" v-model="customWidth" placeholder="Ширина" min="1" step="1"> ×
          <input type="number" v-model="customHeight" placeholder="Высота" min="1" step="1"> мм
        </div>
      </div>
      <div class="parameters">
        <label>Отступ (margin):</label>
        <input type="number" v-model="margin" min="0" step="1"> мм
        <label>Зазор (gap):</label>
        <input type="number" v-model="gap" min="0" step="0.5"> мм
      </div>
      <div class="actions">
        <button @click="loadParts" :disabled="loading">Загрузить части</button>
        <button @click="runNesting" :disabled="loading || parts.length === 0">Выполнить упаковку</button>
        <button @click="exportSVG" :disabled="sheets.length === 0">Экспорт SVG</button>
      </div>
    </div>

    <div class="results">
      <div class="metrics" v-if="metrics">
        <h3>Метрики упаковки</h3>
        <p>Листов: {{ metrics.totalSheets }}</p>
        <p>Частей: {{ metrics.totalParts }}</p>
        <p>Утилизация: {{ metrics.utilization.toFixed(1) }}%</p>
        <p>Общая площадь частей: {{ metrics.totalPartsArea.toFixed(0) }} мм²</p>
        <p>Общая площадь листов: {{ metrics.totalSheetsArea.toFixed(0) }} мм²</p>
      </div>

      <div class="sheets-preview">
        <div v-for="(sheet, index) in sheets" :key="index" class="sheet-container">
          <h4>Лист {{ index + 1 }}</h4>
          <div class="sheet-svg-wrapper" :style="sheetStyle(sheet)">
            <svg :width="sheet.width" :height="sheet.height" class="sheet-svg">
              <!-- Контур листа -->
              <rect :x="0" :y="0" :width="sheet.width" :height="sheet.height"
                    fill="none" stroke="#000" stroke-width="0.5"/>
              <!-- Части -->
              <g v-for="part in sheet.parts" :key="part.id">
                <rect :x="part.x" :y="part.y" :width="part.width" :height="part.height"
                      fill="rgba(100, 149, 237, 0.3)" stroke="#1e3a8a" stroke-width="0.2"/>
                <text :x="part.x + part.width/2" :y="part.y + part.height/2"
                      text-anchor="middle" font-size="3" fill="#000">{{ part.id }}</text>
              </g>
            </svg>
          </div>
          <p class="sheet-utilization">Заполнение: {{ sheet.utilization.toFixed(1) }}%</p>
        </div>
      </div>
    </div>

    <div v-if="loading" class="loading-overlay">
      <div class="spinner"></div>
      <p>Выполняется упаковка...</p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'

interface PaperSize {
  id: string
  name: string
  width: number
  height: number
}

interface Part {
  id: number
  width: number
  height: number
}

interface NestPart {
  id: number
  x: number
  y: number
  width: number
  height: number
  rotation: number
}

interface NestSheet {
  width: number
  height: number
  parts: NestPart[]
  utilization: number
}

interface NestMetrics {
  totalSheets: number
  totalParts: number
  utilization: number
  totalPartsArea: number
  totalSheetsArea: number
}

// Данные
const paperSizes = ref<PaperSize[]>([
  { id: 'a4', name: 'A4', width: 210, height: 297 },
  { id: 'a3', name: 'A3', width: 297, height: 420 },
  { id: 'letter', name: 'Letter', width: 215.9, height: 279.4 },
  { id: 'custom', name: 'Произвольный', width: 0, height: 0 }
])
const selectedPaperSize = ref('a4')
const customWidth = ref(300)
const customHeight = ref(400)
const margin = ref(5)
const gap = ref(2)
const parts = ref<Part[]>([
  { id: 1, width: 50, height: 30 },
  { id: 2, width: 40, height: 25 },
  { id: 3, width: 60, height: 35 },
  { id: 4, width: 30, height: 30 },
  { id: 5, width: 70, height: 20 },
])
const sheets = ref<NestSheet[]>([])
const metrics = ref<NestMetrics | null>(null)
const loading = ref(false)

// Вычисляемые свойства
const currentPaperSize = computed(() => {
  const size = paperSizes.value.find(s => s.id === selectedPaperSize.value)
  if (size && size.id === 'custom') {
    return { width: customWidth.value, height: customHeight.value }
  }
  const fallback = size || paperSizes.value[0]!
  return { width: fallback.width, height: fallback.height }
})

// Методы
const onPaperSizeChange = () => {
  // При изменении размера бумаги можно обновить что-то
}

const loadParts = async () => {
  // Загрузка частей из проекта (заглушка)
  // В реальности нужно получить развертки из store
  console.log('Загрузка частей')
}

const runNesting = async () => {
  loading.value = true
  try {
    const paperWidth = currentPaperSize.value.width
    const paperHeight = currentPaperSize.value.height
    const partsJson = JSON.stringify(parts.value.map(p => ({ width: p.width, height: p.height })))

    // Вызов Tauri команды
    const resultJson = await invoke<string>('nest_parts', {
      partsJson: partsJson,
      paperSize: [paperWidth, paperHeight],
      margin: margin.value,
      gap: gap.value
    })

    // Парсинг результата (пока заглушка)
    // В реальности результат должен соответствовать структуре NestResult из Rust
    console.log('Result:', resultJson)
    // Для демо создадим фиктивные листы
    generateMockSheets()
  } catch (error) {
    console.error('Ошибка упаковки:', error)
    alert('Ошибка упаковки: ' + error)
  } finally {
    loading.value = false
  }
}

const generateMockSheets = () => {
  // Генерация моковых данных для демонстрации
  const sheetWidth = currentPaperSize.value.width
  const sheetHeight = currentPaperSize.value.height
  const mockSheets: NestSheet[] = []
  let x = margin.value
  let y = margin.value
  let sheetIndex = 0
  let partsPlaced = 0

  for (const part of parts.value) {
    if (x + part.width + margin.value > sheetWidth) {
      x = margin.value
      y += part.height + gap.value
    }
    if (y + part.height + margin.value > sheetHeight) {
      // Новый лист
      sheetIndex++
      x = margin.value
      y = margin.value
    }
    let sheet = mockSheets[sheetIndex]
    if (!sheet) {
      sheet = {
        width: sheetWidth,
        height: sheetHeight,
        parts: [],
        utilization: 0
      }
      mockSheets[sheetIndex] = sheet
    }
    sheet.parts.push({
      id: part.id,
      x,
      y,
      width: part.width,
      height: part.height,
      rotation: 0
    })
    partsPlaced++
    x += part.width + gap.value
  }

  // Вычисляем утилизацию для каждого листа
  mockSheets.forEach(sheet => {
    const totalArea = sheet.width * sheet.height
    const partsArea = sheet.parts.reduce((sum, p) => sum + p.width * p.height, 0)
    sheet.utilization = (partsArea / totalArea) * 100
  })

  // Вычисляем общие метрики
  const totalSheets = mockSheets.length
  const totalParts = partsPlaced
  const totalPartsArea = parts.value.reduce((sum, p) => sum + p.width * p.height, 0)
  const totalSheetsArea = totalSheets * sheetWidth * sheetHeight
  const utilization = (totalPartsArea / totalSheetsArea) * 100

  metrics.value = {
    totalSheets,
    totalParts,
    utilization,
    totalPartsArea,
    totalSheetsArea
  }

  sheets.value = mockSheets
}

const exportSVG = () => {
  // Экспорт SVG (заглушка)
  alert('Экспорт SVG пока не реализован')
}

const sheetStyle = (sheet: NestSheet) => {
  const scale = 0.8 // Масштаб для отображения
  return {
    width: `${sheet.width * scale}px`,
    height: `${sheet.height * scale}px`
  }
}

// Инициализация
onMounted(() => {
  // При монтировании можно загрузить части из store
})
</script>

<style scoped>
.paper-nester {
  padding: 20px;
  display: flex;
  flex-direction: column;
  gap: 20px;
  height: 100%;
  position: relative;
}

.controls {
  display: flex;
  flex-wrap: wrap;
  gap: 20px;
  padding: 15px;
  background-color: #f5f5f5;
  border-radius: 8px;
  border: 1px solid #ddd;
}

.paper-size-selector,
.parameters,
.actions {
  display: flex;
  flex-direction: column;
  gap: 10px;
}

label {
  font-weight: bold;
  font-size: 14px;
}

select, input[type="number"] {
  padding: 8px;
  border: 1px solid #ccc;
  border-radius: 4px;
  font-size: 14px;
}

.custom-size {
  display: flex;
  align-items: center;
  gap: 5px;
}

.actions button {
  padding: 10px 15px;
  background-color: #007bff;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 14px;
  transition: background-color 0.2s;
}

.actions button:hover:not(:disabled) {
  background-color: #0056b3;
}

.actions button:disabled {
  background-color: #ccc;
  cursor: not-allowed;
}

.results {
  display: flex;
  gap: 30px;
}

.metrics {
  flex: 0 0 250px;
  padding: 15px;
  background-color: #e9f7fe;
  border-radius: 8px;
  border: 1px solid #b3e0ff;
}

.metrics h3 {
  margin-top: 0;
  color: #0066cc;
}

.sheets-preview {
  flex: 1;
  display: flex;
  flex-wrap: wrap;
  gap: 20px;
  overflow-y: auto;
  max-height: 500px;
}

.sheet-container {
  border: 1px solid #ddd;
  border-radius: 8px;
  padding: 10px;
  background-color: white;
  box-shadow: 0 2px 5px rgba(0,0,0,0.1);
}

.sheet-container h4 {
  margin: 0 0 10px 0;
  text-align: center;
}

.sheet-svg-wrapper {
  border: 1px solid #aaa;
  background-color: #f9f9f9;
  overflow: hidden;
}

.sheet-svg {
  display: block;
}

.sheet-utilization {
  text-align: center;
  font-size: 12px;
  color: #666;
  margin: 5px 0 0;
}

.loading-overlay {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: rgba(255, 255, 255, 0.8);
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  z-index: 1000;
}

.spinner {
  width: 40px;
  height: 40px;
  border: 4px solid #f3f3f3;
  border-top: 4px solid #007bff;
  border-radius: 50%;
  animation: spin 1s linear infinite;
}

@keyframes spin {
  0% { transform: rotate(0deg); }
  100% { transform: rotate(360deg); }
}
</style>