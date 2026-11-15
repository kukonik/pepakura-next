<template>
  <div class="svg-exporter">
    <h3>Экспорт в SVG</h3>
    <div class="export-controls">
      <div class="control-group">
        <label for="export-type">Тип экспорта:</label>
        <select id="export-type" v-model="exportType" :disabled="isProcessing">
          <option value="nest_result">Результат вложения</option>
          <option value="sheet">Отдельный лист</option>
        </select>
      </div>
      
      <div class="control-group" v-if="exportType === 'sheet'">
        <label for="sheet-index">Номер листа:</label>
        <input 
          id="sheet-index" 
          type="number" 
          v-model.number="sheetIndex" 
          min="0" 
          :max="maxSheetIndex"
          :disabled="isProcessing"
        />
      </div>
      
      <div class="control-group">
        <label for="scale">Масштаб (мм в пикселях):</label>
        <input 
          id="scale" 
          type="number" 
          v-model.number="scale" 
          min="0.1" 
          max="10" 
          step="0.1"
          :disabled="isProcessing"
        />
      </div>
      
      <div class="control-group checkbox-group">
        <input 
          id="include-margins" 
          type="checkbox" 
          v-model="includeMargins" 
          :disabled="isProcessing"
        />
        <label for="include-margins">Включить поля</label>
      </div>
      
      <div class="control-group checkbox-group">
        <input 
          id="include-labels" 
          type="checkbox" 
          v-model="includeLabels" 
          :disabled="isProcessing"
        />
        <label for="include-labels">Включить метки</label>
      </div>
      
      <button 
        @click="exportToSVG" 
        :disabled="isProcessing || !canExport"
        class="export-button"
      >
        {{ isProcessing ? 'Экспорт...' : 'Экспортировать в SVG' }}
      </button>
    </div>
    
    <div v-if="error" class="error-message">
      {{ error }}
    </div>
    
    <div v-if="successMessage" class="success-message">
      {{ successMessage }}
      <div v-if="exportPath" class="export-path">
        Путь к файлу: {{ exportPath }}
      </div>
      <button @click="openExportedFile" class="open-file-button" v-if="exportPath">
        Открыть файл
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { save } from '@tauri-apps/plugin-dialog'
import { writeTextFile } from '@tauri-apps/plugin-fs'
import { useProjectStore } from '@/stores/projectStore'

type ExportType = 'nest_result' | 'sheet'

const projectStore = useProjectStore()
const isProcessing = ref(false)
const error = ref<string | null>(null)
const successMessage = ref<string | null>(null)

const exportType = ref<ExportType>('nest_result')
const sheetIndex = ref<number>(0)
const scale = ref<number>(1.0)
const includeMargins = ref<boolean>(true)
const includeLabels = ref<boolean>(true)

// Проверяем, можно ли выполнить экспорт
const canExport = computed(() => {
  return !!projectStore.currentProject &&
         (exportType.value === 'nest_result' ? !!projectStore.nestResult : true)
})

// Сброс сообщений при изменении параметров
watch([exportType, sheetIndex, scale, includeMargins, includeLabels], () => {
  error.value = null
  successMessage.value = null
})

const exportToSVG = async () => {
  if (isProcessing.value) return
  if (!projectStore.currentProject) {
    error.value = 'Нет загруженного проекта'
    return
  }
  
  isProcessing.value = true
  error.value = null
  successMessage.value = null
  
  try {
    let svgs: string[] = []
    
    if (exportType.value === 'nest_result') {
      // Экспорт результата вложения
      if (!projectStore.nestResult) {
        throw new Error('Нет результата вложения для экспорта')
      }
      
      svgs = await invoke<string[]>('export_nest_result_to_svg', {
        project: projectStore.currentProject,
        params: {
          quality: 'medium',
          marginMm: 2.0,
          rotationStep: 15,
          allowFlipping: true
        }
      })
    } else {
      // Экспорт отдельного листа
      svgs = [await invoke<string>('export_sheet_to_svg', {
        project: projectStore.currentProject,
        sheetIndex: sheetIndex.value
      })]
    }
    
    // Сохраняем SVG файлы
    if (svgs.length > 0) {
      for (let i = 0; i < svgs.length; i++) {
        const defaultName = exportType.value === 'nest_result'
          ? `nest_result_${i + 1}.svg`
          : `sheet_${sheetIndex.value + 1}.svg`
          
        const filePath = await save({
          defaultPath: defaultName,
          filters: [{
            name: 'SVG Files',
            extensions: ['svg']
          }]
        })
        
        if (filePath) {
          await writeTextFile(filePath, svgs[i])
        }
      }
      
      successMessage.value = `Экспорт в SVG успешно выполнен! Сохранено файлов: ${svgs.length}`
    } else {
      throw new Error('Нет данных для экспорта')
    }
  } catch (err: any) {
    error.value = err.message || 'Ошибка при экспорте в SVG'
    console.error('Failed to export to SVG:', err)
  } finally {
    isProcessing.value = false
  }
}
</script>

<style scoped>
.svg-exporter {
  padding: 20px;
  background-color: var(--bg-secondary);
  border-radius: 8px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.control-group {
  margin-bottom: 16px;
}

.control-group label {
  display: block;
  margin-bottom: 4px;
  font-weight: 500;
  color: var(--text-primary);
}

.control-group input,
.control-group select {
  width: 100%;
  padding: 8px 12px;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  background-color: var(--bg-input);
  color: var(--text-primary);
  font-size: 1rem;
}

.control-group input:disabled,
.control-group select:disabled {
  background-color: var(--bg-disabled);
  cursor: not-allowed;
}

.checkbox-group {
  display: flex;
  align-items: center;
}

.checkbox-group input {
  width: auto;
  margin-right: 8px;
}

.export-button {
  width: 100%;
  padding: 12px;
  background-color: var(--primary-color);
  color: white;
  border: none;
  border-radius: 4px;
  font-size: 1rem;
  font-weight: 500;
  cursor: pointer;
  transition: background-color 0.2s;
}

.export-button:hover:not(:disabled) {
  background-color: var(--primary-color-dark);
}

.export-button:disabled {
  background-color: var(--bg-disabled);
  cursor: not-allowed;
}

.error-message {
  padding: 12px;
  background-color: var(--error-bg);
  border: 1px solid var(--error-border);
  border-radius: 4px;
  color: var(--error-text);
  margin-top: 16px;
}

.success-message {
  padding: 12px;
  background-color: var(--success-bg);
  border: 1px solid var(--success-border);
  border-radius: 4px;
  color: var(--success-text);
  margin-top: 16px;
}

.export-path {
  margin-top: 8px;
  font-size: 0.9rem;
  word-break: break-all;
}

.open-file-button {
  margin-top: 12px;
  padding: 8px 16px;
  background-color: var(--secondary-color);
  color: white;
  border: none;
  border-radius: 4px;
  font-size: 0.9rem;
  cursor: pointer;
  transition: background-color 0.2s;
}

.open-file-button:hover {
  background-color: var(--secondary-color-dark);
}
</style>