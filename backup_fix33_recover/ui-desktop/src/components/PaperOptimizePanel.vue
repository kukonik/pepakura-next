
<template>
  <div class="paper-optimize-panel">
    <h2>Оптимизация для бумаги</h2>
    
    <PaperOptimizeSettings
      :settings="paperSettings"
      @update:settings="updateSettings"
      @reset="resetToDefaults"
    />
    
    <div class="actions">
      <button @click="optimizeModel" class="btn-primary">
        Оптимизировать модель
      </button>
    </div>
    
    <div v-if="optimizationResult" class="results-section">
      <PaperOptimizationResults
        :paper-usage="{
          sheetCount: optimizationResult.sheetCount,
          usagePercentage: optimizationResult.usagePercentage,
          modelArea: optimizationResult.modelArea,
          usedArea: optimizationResult.usedArea
        }"
        :assembly-tips="optimizationResult.assemblyTips"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue'
import { listen } from '@tauri-apps/api/event'
import { invoke } from '@tauri-apps/api/tauri'
import PaperOptimizationResults from './PaperOptimizationResults.vue'

// Определение типов
interface PaperSettings {
  sheetWidth: number
  sheetHeight: number
  minGap: number
  minTabWidth: number
  maxAutoTabAngle: number
  addPrintMargins: boolean
  marginSize: number
}

interface OptimizationResult {
  sheetCount: number
  usagePercentage: number
  modelArea: number
  usedArea: number
  assemblyTips: string[]
}

// Реактивные данные
const paperSettings = reactive<PaperSettings>({
  sheetWidth: 210,
  sheetHeight: 297,
  minGap: 2,
  minTabWidth: 5,
  maxAutoTabAngle: 60,
  addPrintMargins: true,
  marginSize: 5
})

const optimizationResult = ref<OptimizationResult | null>(null)

// Загрузка параметров по умолчанию при монтировании
onMounted(async () => {
  try {
    const defaultParams: any = await invoke('get_default_paper_optimize_params')
    paperSettings.sheetWidth = defaultParams.sheet_width
    paperSettings.sheetHeight = defaultParams.sheet_height
    paperSettings.minGap = defaultParams.min_gap
    paperSettings.minTabWidth = defaultParams.min_tab_width
    paperSettings.maxAutoTabAngle = defaultParams.max_auto_tab_angle
    paperSettings.addPrintMargins = defaultParams.add_print_margins
    paperSettings.marginSize = defaultParams.margin_size
  } catch (error) {
    console.error('Failed to load default paper optimize params:', error)
  }
})

// Функция оптимизации модели
async function optimizeModel() {
  try {
    // Здесь должна быть логика получения текущей модели
    // Пока используем пустую модель для демонстрации
    const model = {
      vertices: [],
      faces: []
    }
    
    // Подготавливаем параметры для передачи в бэкенд
    const params = {
      sheet_width: paperSettings.sheetWidth,
      sheet_height: paperSettings.sheetHeight,
      min_gap: paperSettings.minGap,
      min_tab_width: paperSettings.minTabWidth,
      max_auto_tab_angle: paperSettings.maxAutoTabAngle,
      add_print_margins: paperSettings.addPrintMargins,
      margin_size: paperSettings.marginSize
    }
    
    // Вызываем команду оптимизации
    const result: any = await invoke('optimize_model_for_paper', { model, params })
    
    // Обновляем результаты оптимизации
    optimizationResult.value = {
      sheetCount: result.paper_usage.sheet_count,
      usagePercentage: result.paper_usage.usage_percentage,
      modelArea: result.paper_usage.model_area,
      usedArea: result.paper_usage.used_area,
      assemblyTips: result.assembly_tips
    }
  } catch (error) {
    console.error('Failed to optimize model:', error)
    // Здесь можно показать уведомление об ошибке
  }
}

// Сброс параметров по умолчанию
async function resetToDefaults() {
  try {
    const defaultParams: any = await invoke('get_default_paper_optimize_params')
    paperSettings.sheetWidth = defaultParams.sheet_width
    paperSettings.sheetHeight = defaultParams.sheet_height
    paperSettings.minGap = defaultParams.min_gap
    paperSettings.minTabWidth = defaultParams.min_tab_width
    paperSettings.maxAutoTabAngle = defaultParams.max_auto_tab_angle
    paperSettings.addPrintMargins = defaultParams.add_print_margins
    paperSettings.marginSize = defaultParams.margin_size
  } catch (error) {
    console.error('Failed to reset to defaults:', error)
  }
}
</script>

<style scoped>
.paper-optimize-panel {
  padding: 20px;
  background-color: #f5f5f5;
  border-radius: 8px;
  max-width: 600px;
  margin: 0 auto;
}

.settings-section {
  margin-bottom: 20px;
  padding: 15px;
  background-color: white;
  border-radius: 6px;
  box-shadow: 0 2px 4px rgba(0,0,0,0.1);
}

.settings-section h3 {
  margin-top: 0;
  color: #333;
  border-bottom: 1px solid #eee;
  padding-bottom: 8px;
}

.form-group {
  margin-bottom: 15px;
}

.form-group label {
  display: block;
  margin-bottom: 5px;
  font-weight: 500;
  color: #555;
}

.form-group input[type="number"] {
  width: 100%;
  padding: 8px;
  border: 1px solid #ddd;
  border-radius: 4px;
  font-size: 14px;
}

.checkbox-group {
  display: flex;
  align-items: center;
}

.checkbox-group input[type="checkbox"] {
  margin-right: 8px;
}

.actions {
  display: flex;
  gap: 10px;
  margin: 20px 0;
}

.btn-primary, .btn-secondary {
  padding: 10px 20px;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 14px;
  font-weight: 500;
  transition: background-color 0.2s;
}

.btn-primary {
  background-color: #007bff;
  color: white;
}

.btn-primary:hover {
  background-color: #0056b3;
}

.btn-secondary {
  background-color: #6c757d;
  color: white;
}

.btn-secondary:hover {
  background-color: #545b62;
}

.results-section {
  margin-top: 20px;
  padding: 15px;
  background-color: white;
  border-radius: 6px;
  box-shadow: 0 2px 4px rgba(0,0,0,0.1);
}

.results-section h3 {
  margin-top: 0;
  color: #333;
}

.result-item {
  display: flex;
  justify-content: space-between;
  padding: 8px 0;
  border-bottom: 1px solid #eee;
}

.result-item:last-child {
  border-bottom: none;
}

.result-item span {
  color: #666;
}

.result-item strong {
  color: #333;
  font-weight: 500;
}

.tips-section {
  margin-top: 15px;
}

.tips-section h4 {
  margin-top: 0;
  color: #333;
}

.tips-section ul {
  padding-left: 20px;
  margin: 0;
}

.tips-section li {
  margin-bottom: 5px;
  color: #666;
}
</style>
</file>
<line_count>280</line_count>
</write_to_file>