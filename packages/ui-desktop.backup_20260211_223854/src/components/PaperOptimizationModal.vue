<template>
  <div class="modal-overlay" @click="closeModal">
    <div class="modal-content" @click.stop>
      <div class="modal-header">
        <h2>Оптимизация для бумаги</h2>
        <button class="close-btn" @click="closeModal">×</button>
      </div>
      
      <div class="modal-body">
        <PaperOptimizeSettings
          :settings="paperSettings"
          @update:settings="updateSettings"
          @reset="resetToDefaults"
        />
        
        <div class="actions">
          <button @click="optimizeModel" class="btn-primary" :disabled="isOptimizing">
            {{ isOptimizing ? 'Оптимизация...' : 'Оптимизировать модель' }}
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
        
        <div v-if="error" class="error-message">
          {{ error }}
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'
import PaperOptimizeSettings from './PaperOptimizeSettings.vue'
import PaperOptimizationResults from './PaperOptimizationResults.vue'
import { useProjectStore } from '@/stores/project'

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

// Типы для результатов оптимизации
interface PaperUsage {
  sheetCount: number
  usagePercentage: number
  modelArea: number
  usedArea: number
}

interface OptimizationResult {
  sheetCount: number
  usagePercentage: number
  modelArea: number
  usedArea: number
  assemblyTips: string[]
}

interface PaperUsage {
  sheetCount: number
  usagePercentage: number
  modelArea: number
  usedArea: number
}

interface OptimizationResult {
  sheetCount: number
  usagePercentage: number
  modelArea: number
  usedArea: number
  assemblyTips: string[]
}

// События
const emit = defineEmits<{
  (e: 'close'): void
}>()

// Хранилище проекта
const projectStore = useProjectStore()

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
const isOptimizing = ref(false)
const error = ref<string | null>(null)

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
  } catch (err) {
    console.error('Failed to load default paper optimize params:', err)
    error.value = 'Не удалось загрузить параметры по умолчанию'
  }
})

// Функция обновления настроек
function updateSettings(newSettings: PaperSettings) {
  Object.assign(paperSettings, newSettings)
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
    error.value = null
  } catch (err) {
    console.error('Failed to reset to defaults:', err)
    error.value = 'Не удалось сбросить параметры по умолчанию'
  }
}

// Функция оптимизации модели
async function optimizeModel() {
  if (isOptimizing.value) return
  
  isOptimizing.value = true
  error.value = null
  
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
  } catch (err) {
    console.error('Failed to optimize model:', err)
    error.value = 'Не удалось оптимизировать модель'
  } finally {
    isOptimizing.value = false
  }
}

// Закрытие модального окна
function closeModal() {
  emit('close')
}
</script>

<style scoped>
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: rgba(0, 0, 0, 0.7);
  display: flex;
  justify-content: center;
  align-items: center;
  z-index: 1000;
}

.modal-content {
  background: #1e293b;
  border-radius: 0.75rem;
  border: 1px solid rgba(148, 163, 184, 0.7);
  width: 90%;
  max-width: 600px;
  max-height: 90vh;
  overflow: hidden;
  display: flex;
  flex-direction: column;
}

.modal-header {
  padding: 1rem;
  border-bottom: 1px solid rgba(148, 163, 184, 0.5);
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.modal-header h2 {
  margin: 0;
  font-size: 1.25rem;
  font-weight: 500;
  color: #f1f5f9;
}

.close-btn {
  background: none;
  border: none;
  font-size: 1.5rem;
  color: #94a3b8;
  cursor: pointer;
  padding: 0;
  width: 32px;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 50%;
}

.close-btn:hover {
  background-color: rgba(148, 163, 184, 0.2);
  color: #f1f5f9;
}

.modal-body {
  padding: 1rem;
  overflow-y: auto;
  flex: 1;
}

.actions {
  display: flex;
  justify-content: center;
  margin: 1.5rem 0;
}

.btn-primary {
  border-radius: 999px;
  border: 1px solid transparent;
  padding: 0.5rem 1.5rem;
  font-size: 0.875rem;
  cursor: pointer;
  background: linear-gradient(135deg, #3b82f6, #6366f1);
  color: #f9fafb;
  font-weight: 500;
  transition: all 0.2s;
}

.btn-primary:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.btn-primary:hover:not(:disabled) {
  transform: translateY(-1px);
  box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
}

.results-section {
  margin-top: 1.5rem;
}

.error-message {
  color: #f87171;
  background-color: rgba(239, 68, 68, 0.1);
  border: 1px solid rgba(239, 68, 68, 0.3);
  border-radius: 0.375rem;
  padding: 0.75rem;
  margin-top: 1rem;
  font-size: 0.875rem;
}
</style>