<template>
  <div class="print-nesting-panel">
    <h3>Печать и размещение</h3>
    
    <div class="export-info">
      <span class="vector-badge">Точный вектор для печати</span>
    </div>
    
    <div class="settings-section">
      <h4>Настройки бумаги</h4>
      <div class="form-group">
        <label for="paperFormat">Формат бумаги:</label>
        <select id="paperFormat" v-model="paperFormat">
          <option value="A4">A4 (210×297 мм)</option>
          <option value="A3">A3 (297×420 мм)</option>
          <option value="Letter">Letter (215.9×279.4 мм)</option>
        </select>
      </div>
      
      <div class="form-group">
        <label for="marginMm">Поля (мм):</label>
        <input 
          id="marginMm" 
          type="number" 
          v-model.number="marginMm" 
          min="0" 
          max="50"
        />
      </div>
      
      <div class="form-group">
        <label for="scale">Масштаб:</label>
        <input 
          id="scale" 
          type="number" 
          v-model.number="scale" 
          min="0.1" 
          max="10" 
          step="0.1"
        />
      </div>
    </div>
    
    <div class="settings-section">
      <h4>Настройки размещения</h4>
      <div class="form-group">
        <label for="maxSheets">Максимум листов:</label>
        <input
          id="maxSheets"
          type="number"
          v-model.number="maxSheets"
          min="1"
          max="100"
        />
      </div>
      
      <div class="form-group">
        <label for="rotationStep">Шаг вращения (градусы):</label>
        <input
          id="rotationStep"
          type="number"
          v-model.number="rotationStepDeg"
          min="0"
          max="90"
          step="5"
        />
        <small>0 для отключения вращения</small>
      </div>
    </div>
    
    <div class="actions">
      <button 
        @click="nestProject" 
        :disabled="isNesting || !hasProject"
        class="primary-button"
      >
        {{ isNesting ? 'Размещение...' : 'Разместить развертки' }}
      </button>
      
      <button
        @click="exportToSvg"
        :disabled="!hasNestingResult || isExporting"
        class="secondary-button export-button"
      >
        <span v-if="isExporting && exportTotal">
          Сохранение {{ exportProgress }}/{{ exportTotal }} (Лист {{ exportSheetIndex }}/{{ exportSheetTotal }})
        </span>
        <span v-else>
          {{ isExporting ? 'Экспорт...' : 'Экспорт в SVG' }}
        </span>
      </button>
      
      <!-- Опция экспорта в один файл -->
      <div class="form-group export-option">
        <label>
          <input
            type="checkbox"
            v-model="exportToOneFile"
            :disabled="!hasNestingResult || isExporting"
          />
          Экспорт в один файл
        </label>
      </div>
      
      <button
        @click="printProject"
        :disabled="!hasNestingResult"
        class="secondary-button"
      >
        Печать
      </button>
      
      <!-- Кнопка для перехода в редактор размещения -->
      <button
        v-if="hasNestingResult && !projectStore.hasPartOverrides()"
        @click="enterEditingMode"
        class="secondary-button"
      >
        Редактировать размещение
      </button>
      
      <!-- Кнопка для сброса ручных изменений -->
      <button
        v-if="projectStore.hasPartOverrides()"
        @click="resetOverrides"
        class="secondary-button"
      >
        Сбросить правки
      </button>
      
      <!-- Индикатор автосохранения -->
      <div v-if="projectStore.isDirty" class="auto-save-indicator">
        Автосохранение...
      </div>
    </div>
    
    <div v-if="nestError" class="error-message">
      Ошибка: {{ nestError }}
    </div>
    
    <div v-if="exportError" class="error-message">
      Ошибка экспорта: {{ exportError }}
    </div>
    
    <div v-if="exportSheetError" class="error-message">
      Ошибка экспорта листа: {{ exportSheetError }}
    </div>
    
    <div v-if="hasNestingResult" class="nesting-results">
      <h4>Результаты размещения</h4>
      <div class="metrics-summary">
        <h5>Метрики качества</h5>
        <p>Листов использовано: {{ nestResult.metrics.totalSheets }}</p>
        <p>Частей размещено: {{ nestResult.metrics.totalParts }}</p>
        <p>Средняя заполненность: {{ nestResult.metrics.avgFillRate.toFixed(2) }}%</p>
        <p>Общая площадь частей: {{ nestResult.metrics.totalPartsArea.toFixed(2) }} мм²</p>
      </div>
      
      <div class="params-summary">
        <h5>Использованные настройки</h5>
        <p>Формат бумаги: {{ nestResult.paramsSnapshot.paper.format }}</p>
        <p>Размеры: {{ nestResult.paramsSnapshot.paper.width_mm }} × {{ nestResult.paramsSnapshot.paper.height_mm }} мм</p>
        <p>Поля: {{ nestResult.paramsSnapshot.paper.margin_mm }} мм</p>
        <p>Масштаб: {{ nestResult.paramsSnapshot.scale }}</p>
        <p>Максимум листов: {{ nestResult.paramsSnapshot.max_sheets }}</p>
        <p>Всего листов для экспорта: {{ nestResult.sheets.length }}</p>
      </div>
      
      <div class="sheets-summary">
        <h5>Листы</h5>
        <div class="sheet-preview-first" v-if="firstSheetSvg || isFirstSheetSvgLoading">
          <h6>Предпросмотр первого листа</h6>
          <div class="svg-preview-container">
            <div v-if="isFirstSheetSvgLoading" class="loading-indicator">
              Загрузка предпросмотра...
            </div>
            <div v-else v-html="firstSheetSvg"></div>
          </div>
        </div>
        <div class="sheet-thumbnails">
          <div
            v-for="sheet in sheetPreviews"
            :key="sheet.index"
            class="sheet-thumbnail"
            @click="openPreviewModal(sheet.index)"
          >
            <div class="thumbnail-placeholder">
              <span class="thumbnail-text">Лист {{ sheet.sheetNumber }}</span>
              <span class="parts-count">{{ sheet.partsCount }} частей</span>
            </div>
            <div class="sheet-info">
              <p>Размер: {{ sheet.width }} × {{ sheet.height }} мм</p>
            </div>
          </div>
        </div>
      </div>
    </div>
    
    <!-- Модальное окно предпросмотра -->
    <div v-if="isPreviewModalOpen" class="modal-overlay" @click="closePreviewModal">
      <div class="modal-content" @click.stop>
        <div class="modal-header">
          <h3>Предпросмотр листа {{ previewSheetIndex + 1 }}</h3>
          <button class="close-button" @click="closePreviewModal">×</button>
        </div>
        <div class="modal-body">
          <div v-if="isSvgLoading" class="loading-indicator">
            Загрузка SVG...
          </div>
          <div v-else class="svg-preview-container">
            <SheetPreview :sheetIndex="previewSheetIndex" />
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import { useProjectStore } from '@/stores/projectStore'
import { PaperSettings, NestParams } from '@/../../shared/src/types/nesting'
import { save } from '@tauri-apps/plugin-dialog'
import { writeTextFile } from '@tauri-apps/plugin-fs'
import SheetPreview from './SheetPreview.vue'

// Хранилище проекта
const projectStore = useProjectStore()

// Состояние компонента
const paperFormat = ref('A4')
const marginMm = ref(5)
const scale = ref(1.0)
const maxSheets = ref(4)
const rotationStepDeg = ref(15)
const exportToOneFile = ref(false) // Опция экспорта в один файл

// Состояние модального окна
const isPreviewModalOpen = ref(false)
const previewSheetIndex = ref(0)

// Состояние для SVG первого листа
const firstSheetSvg = ref<string | null>(null)
const isFirstSheetSvgLoading = ref(false)

// Вычисляемые свойства
const hasProject = computed(() => projectStore.hasProject)
const isNesting = computed(() => projectStore.isNesting)
const nestError = computed(() => projectStore.nestError)
const isExporting = computed(() => projectStore.isExporting)
const exportError = computed(() => projectStore.exportError)
const exportSheetError = computed(() => projectStore.exportSheetError)
const exportProgress = computed(() => projectStore.exportProgress)
const exportTotal = computed(() => projectStore.exportTotal)
const exportSheetIndex = computed(() => projectStore.exportSheetIndex)
const exportSheetTotal = computed(() => projectStore.exportSheetTotal)
const nestResult = computed(() => projectStore.nestResult)
const hasNestingResult = computed(() => !!nestResult.value)
const isSvgLoading = computed(() => projectStore.isSvgLoading)

// Вычисляемые свойства для листов
const sheetPreviews = computed(() => {
  if (!nestResult.value) return []
  
  return nestResult.value.sheets.map((sheet, index) => ({
    index,
    sheetNumber: index + 1,
    partsCount: sheet.parts.length,
    width: sheet.width_mm,
    height: sheet.height_mm
  }))
})

// Вычисляемое свойство для первого листа SVG
const firstSheetSvg = computed(() => {
  if (!nestResult.value || nestResult.value.sheets.length === 0) return null
  // В реальной реализации здесь будет вызов projectStore.getSheetSvg(0)
  // Пока возвращаем заглушку
  return `<svg xmlns="http://www.w3.org/2000/svg" width="200" height="200" viewBox="0 0 200 200">
    <rect x="0" y="0" width="200" height="200" fill="white" stroke="black" stroke-width="1"/>
    <text x="100" y="100" text-anchor="middle" dominant-baseline="middle" font-family="Arial" font-size="12">
      Предпросмотр листа 1
    </text>
  </svg>`
})

// Функции для работы с предпросмотром
const openPreviewModal = (sheetIndex: number) => {
  previewSheetIndex.value = sheetIndex
  isPreviewModalOpen.value = true
}

const closePreviewModal = () => {
  isPreviewModalOpen.value = false
}

const getSheetSvgPreview = async (sheetIndex: number) => {
  try {
    return await projectStore.getSheetSvg(sheetIndex)
  } catch (error) {
    console.error(`Failed to get SVG for sheet ${sheetIndex}:`, error)
    return null
  }
}

// Функция для загрузки SVG первого листа
const loadFirstSheetSvg = async () => {
  if (!nestResult.value || nestResult.value.sheets.length === 0) {
    firstSheetSvg.value = null
    return
  }

  isFirstSheetSvgLoading.value = true
  try {
    firstSheetSvg.value = await projectStore.getSheetSvg(0)
  } catch (error) {
    console.error('Failed to load first sheet SVG:', error)
    firstSheetSvg.value = null
  } finally {
    isFirstSheetSvgLoading.value = false
  }
}

// Загружаем SVG первого листа при изменении результата размещения
watch(nestResult, () => {
  loadFirstSheetSvg()
}, { immediate: true })

// Вычисляемое свойство для первого листа SVG
const firstSheetSvg = computed(async () => {
  if (!nestResult.value || nestResult.value.sheets.length === 0) return null
  // Получаем SVG первого листа через store
  try {
    return await projectStore.getSheetSvg(0)
  } catch (error) {
    console.error('Failed to get first sheet SVG:', error)
    return null
  }
})

// Функции
const nestProject = async () => {
  if (!projectStore.currentProject) return
  
  // Создаем настройки бумаги
  const paperSettings: PaperSettings = {
    format: paperFormat.value,
    width_mm: getPaperWidth(paperFormat.value),
    height_mm: getPaperHeight(paperFormat.value),
    margin_mm: marginMm.value
  }
  
  // Создаем параметры размещения
  const nestParams: NestParams = {
    paper: paperSettings,
    max_sheets: maxSheets.value,
    scale: scale.value,
    rotationStepDeg: rotationStepDeg.value
  }
  
  // Выполняем размещение
  await projectStore.nestProject(nestParams)
}

const printProject = () => {
  // TODO: Реализовать печать
  alert('Печать еще не реализована')
}

const exportToSvg = async () => {
  if (!projectStore.currentProject || !nestResult.value) return
  
  try {
    // Создаем настройки бумаги
    const paperSettings: PaperSettings = {
      format: paperFormat.value,
      width_mm: getPaperWidth(paperFormat.value),
      height_mm: getPaperHeight(paperFormat.value),
      margin_mm: marginMm.value
    }
    
    // Создаем параметры размещения
    const nestParams: NestParams = {
      paper: paperSettings,
      max_sheets: maxSheets.value,
      scale: scale.value,
      rotationStepDeg: rotationStepDeg.value
    }
    
    // Открываем диалог сохранения
    const saveResult = await save({
      filters: [{
        name: 'SVG Files',
        extensions: ['svg']
      }]
    })
    
    if (saveResult) {
      // Извлекаем имя файла без расширения
      const baseName = saveResult.replace(/\.[^/.]+$/, "")
      
      if (exportToOneFile.value) {
        // Экспортируем в один файл
        await exportToSingleSvg(nestParams, baseName)
      } else {
        // Экспортируем в SVG с сохранением файлов
        await projectStore.exportNestResultToSvg(nestParams, baseName)
      }
      
      // Показываем уведомление об успешном экспорте
      alert(`Экспортировано ${nestResult.value.sheets.length} SVG файлов`)
    }
  } catch (error) {
    console.error('Failed to export to SVG:', error)
    // Ошибка будет отображена через computed свойство exportError
  }
}

// Функция для экспорта в один SVG файл
const exportToSingleSvg = async (params: NestParams, baseFileName: string) => {
  if (!projectStore.currentProject || !nestResult.value) return
  
  projectStore.isExporting = true
  projectStore.exportError = null
  projectStore.exportSheetError = null
  projectStore.exportProgress = 0
  projectStore.exportTotal = 1
  projectStore.exportSheetIndex = 0
  projectStore.exportSheetTotal = nestResult.value.sheets.length
  
  try {
    // Получаем SVG для всех листов
    const sheetSvgs: string[] = []
    for (let i = 0; i < nestResult.value.sheets.length; i++) {
      try {
        projectStore.exportSheetIndex = i + 1
        projectStore.exportSheetError = null
        const svg = await projectStore.getSheetSvg(i)
        sheetSvgs.push(svg)
      } catch (sheetError: any) {
        // Обработка ошибки конкретного листа
        projectStore.exportSheetError = sheetError.toString()
        console.error(`Failed to export sheet ${i}:`, sheetError)
        // Продолжаем экспорт остальных листов
      }
    }
    
    // Объединяем все SVG в один файл
    const combinedSvg = combineSheetSvgs(sheetSvgs)
    
    // Сохраняем файл
    await writeTextFile(`${baseFileName}.svg`, combinedSvg)
    
    projectStore.exportProgress = 1
  } catch (error: any) {
    projectStore.exportError = error.toString()
    console.error('Failed to export to single SVG:', error)
    throw new Error(`Failed to export to single SVG: ${error}`)
  } finally {
    projectStore.isExporting = false
    projectStore.exportProgress = null
    projectStore.exportTotal = null
    projectStore.exportSheetIndex = null
    projectStore.exportSheetTotal = null
  }
}

// Функция для объединения SVG листов
const combineSheetSvgs = (svgs: string[]): string => {
  if (svgs.length === 0) return ''
  
  // Извлекаем размеры из первого SVG
  const firstSvg = svgs[0]
  const widthMatch = firstSvg.match(/width="([^"]+)"/)
  const heightMatch = firstSvg.match(/height="([^"]+)"/)
  
  const width = widthMatch ? widthMatch[1] : '210mm'
  const height = heightMatch ? heightMatch[1] : '297mm'
  
  // Вычисляем общую высоту для всех листов
  const totalHeight = svgs.length * 300 // Примерное значение, можно уточнить
  
  // Создаем объединенный SVG
  let combinedSvg = `<svg xmlns="http://www.w3.org/2000/svg" width="${width}" height="${totalHeight}mm" viewBox="0 0 210 ${totalHeight}">\n`
  
  // Добавляем все листы с отступами
  svgs.forEach((svg, index) => {
    // Извлекаем содержимое SVG (без тега svg)
    const content = svg.replace(/<svg[^>]*>|<\/svg>/g, '').trim()
    combinedSvg += `  <!-- Sheet ${index + 1} -->\n`
    combinedSvg += `  <g transform="translate(0, ${index * 300})">\n`
    combinedSvg += `    ${content}\n`
    combinedSvg += `  </g>\n`
  })
  
  combinedSvg += '</svg>'
  
  return combinedSvg
}

// Вспомогательные функции
const getPaperWidth = (format: string): number => {
  switch (format) {
    case 'A4': return 210
    case 'A3': return 297
    case 'Letter': return 215.9
    default: return 210
  }
}

const getPaperHeight = (format: string): number => {
  switch (format) {
    case 'A4': return 297
    case 'A3': return 420
    case 'Letter': return 279.4
    default: return 297
  }
}

// Функции для работы с редактором размещения
const enterEditingMode = () => {
  // Переход на страницу редактора размещения
  window.location.hash = '#/nesting-editor'
}

const resetOverrides = () => {
  projectStore.resetPartOverrides()
}
</script>

<style scoped>
.print-nesting-panel {
  padding: 20px;
  background-color: var(--panel-bg, #1e293b);
  border-radius: 8px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.settings-section {
  margin-bottom: 20px;
}

.settings-section h4 {
  margin-bottom: 10px;
  color: var(--text-primary, #f8fafc);
}

.form-group {
  margin-bottom: 15px;
}

.form-group label {
  display: block;
  margin-bottom: 5px;
  font-weight: 500;
  color: var(--text-secondary, #cbd5e1);
}

.form-group input,
.form-group select {
  width: 100%;
  padding: 8px 12px;
  border: 1px solid var(--border-color, #334155);
  border-radius: 4px;
  background-color: var(--input-bg, #0f172a);
  color: var(--text-primary, #f8fafc);
  font-size: 14px;
}

.form-group input:focus,
.form-group select:focus {
  outline: none;
  border-color: var(--primary-color, #6366f1);
  box-shadow: 0 0 0 2px rgba(99, 102, 241, 0.25);
}

.actions {
  display: flex;
  gap: 10px;
  margin-bottom: 20px;
  flex-wrap: wrap;
}

.primary-button,
.secondary-button {
  padding: 10px 20px;
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

.primary-button:disabled {
  background-color: var(--button-disabled-bg, #64748b);
  cursor: not-allowed;
}

.secondary-button {
  background-color: var(--secondary-button-bg, #334155);
  color: var(--text-primary, #f8fafc);
}

.secondary-button:hover {
  background-color: var(--secondary-button-hover-bg, #475569);
}

.error-message {
  padding: 10px;
  background-color: var(--error-bg, #7f1d1d);
  color: var(--error-text, #fecaca);
  border-radius: 4px;
  margin-bottom: 20px;
}

.nesting-results {
  margin-top: 20px;
  padding: 15px;
  background-color: var(--result-bg, #1e293b);
  border-radius: 4px;
}

.nesting-results h4 {
  margin-top: 0;
  color: var(--text-primary, #f8fafc);
}

.sheet-preview {
  padding: 10px;
  margin: 10px 0;
  background-color: var(--sheet-preview-bg, #334155);
  border-radius: 4px;
}

.sheet-preview h5 {
  margin: 0 0 5px 0;
  color: var(--text-primary, #f8fafc);
}

.sheet-preview p {
  margin: 2px 0;
  color: var(--text-secondary, #cbd5e1);
  font-size: 13px;
}

.sheet-preview-first {
  margin-bottom: 20px;
  padding: 15px;
  background-color: var(--preview-bg, #1e293b);
  border-radius: 4px;
}

.sheet-preview-first h6 {
  margin-top: 0;
  color: var(--text-primary, #f8fafc);
}

.svg-preview-container {
  text-align: center;
  margin-top: 10px;
}

.svg-preview-container svg {
  max-width: 100%;
  max-height: 200px;
  border: 1px solid var(--border-color, #334155);
  border-radius: 4px;
}

.sheet-thumbnails {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(120px, 1fr));
  gap: 15px;
  margin-top: 10px;
}

.sheet-thumbnail {
  cursor: pointer;
  border: 1px solid var(--border-color, #334155);
  border-radius: 4px;
  padding: 10px;
  text-align: center;
  transition: border-color 0.2s;
}

.sheet-thumbnail:hover {
  border-color: var(--primary-color, #6366f1);
}

.thumbnail-placeholder {
  background-color: var(--thumbnail-bg, #475569);
  border-radius: 4px;
  padding: 10px;
  margin-bottom: 8px;
}

.thumbnail-text {
  display: block;
  font-weight: 500;
  margin-bottom: 5px;
}

.parts-count {
  display: block;
  font-size: 12px;
  color: var(--text-secondary, #cbd5e1);
}

.sheet-info p {
  margin: 2px 0;
  font-size: 12px;
  color: var(--text-secondary, #cbd5e1);
}

.export-info {
  margin-bottom: 15px;
}

.vector-badge {
  background-color: var(--success-color, #10b981);
  color: white;
  padding: 4px 8px;
  border-radius: 12px;
  font-size: 12px;
  font-weight: 500;
}

.export-button {
  min-width: 150px;
}

.export-option {
  margin-top: 10px;
}

.export-option label {
  display: flex;
  align-items: center;
  cursor: pointer;
}

.export-option input {
  margin-right: 8px;
}

.svg-preview {
  margin-top: 20px;
  padding: 15px;
  background-color: var(--preview-bg, #1e293b);
  border-radius: 4px;
}

.preview-container {
  text-align: center;
  margin-top: 10px;
}

.svg-preview-image {
  max-width: 100%;
  max-height: 200px;
  border: 1px solid var(--border-color, #334155);
  border-radius: 4px;
}

/* Модальное окно */
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: rgba(0, 0, 0, 0.5);
  display: flex;
  justify-content: center;
  align-items: center;
  z-index: 1000;
}

.modal-content {
  background-color: var(--panel-bg, #1e293b);
  border-radius: 8px;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.3);
  max-width: 90vw;
  max-height: 90vh;
  overflow: hidden;
  display: flex;
  flex-direction: column;
}

.modal-header {
  padding: 15px 20px;
  border-bottom: 1px solid var(--border-color, #334155);
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.modal-header h3 {
  margin: 0;
  color: var(--text-primary, #f8fafc);
}

.close-button {
  background: none;
  border: none;
  font-size: 24px;
  cursor: pointer;
  color: var(--text-secondary, #cbd5e1);
  padding: 0;
  width: 30px;
  height: 30px;
  display: flex;
  align-items: center;
  justify-content: center;
}

.close-button:hover {
  color: var(--text-primary, #f8fafc);
}

.modal-body {
  padding: 20px;
  overflow: auto;
  flex: 1;
}

.loading-indicator {
  text-align: center;
  padding: 40px 20px;
  color: var(--text-secondary, #cbd5e1);
}

.svg-preview-container {
  text-align: center;
}

.svg-preview-container svg {
  max-width: 100%;
  max-height: 70vh;
  border: 1px solid var(--border-color, #334155);
  border-radius: 4px;
}

.auto-save-indicator {
  padding: 5px 10px;
  background-color: var(--warning-color, #f59e0b);
  color: white;
  border-radius: 4px;
  font-size: 12px;
  animation: pulse 1s infinite;
}

@keyframes pulse {
  0% { opacity: 1; }
  50% { opacity: 0.7; }
  100% { opacity: 1; }
}
</style>