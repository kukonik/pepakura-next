
<template>
  <div class="paper-settings">
    <h3>Параметры оптимизации</h3>
    
    <div class="settings-group">
      <h4>Параметры бумаги</h4>
      <div class="form-group">
        <label for="sheetWidth">Ширина листа (мм):</label>
        <input 
          id="sheetWidth" 
          type="number" 
          v-model="localSettings.sheetWidth" 
          min="100" 
          max="500"
          @input="emitUpdate"
        />
      </div>
      
      <div class="form-group">
        <label for="sheetHeight">Высота листа (мм):</label>
        <input 
          id="sheetHeight" 
          type="number" 
          v-model="localSettings.sheetHeight" 
          min="100" 
          max="500"
          @input="emitUpdate"
        />
      </div>
      
      <div class="form-group">
        <label for="minGap">Минимальный зазор (мм):</label>
        <input 
          id="minGap" 
          type="number" 
          v-model="localSettings.minGap" 
          min="0" 
          max="10" 
          step="0.1"
          @input="emitUpdate"
        />
      </div>
    </div>
    
    <div class="settings-group">
      <h4>Параметры вкладышей</h4>
      <div class="form-group">
        <label for="minTabWidth">Минимальная ширина вкладыша (мм):</label>
        <input 
          id="minTabWidth" 
          type="number" 
          v-model="localSettings.minTabWidth" 
          min="1" 
          max="20"
          @input="emitUpdate"
        />
      </div>
      
      <div class="form-group">
        <label for="maxAutoTabAngle">Максимальный угол для автовкладышей (градусы):</label>
        <input 
          id="maxAutoTabAngle" 
          type="number" 
          v-model="localSettings.maxAutoTabAngle" 
          min="0" 
          max="180"
          @input="emitUpdate"
        />
      </div>
    </div>
    
    <div class="settings-group">
      <h4>Поля для печати</h4>
      <div class="form-group checkbox-group">
        <input 
          id="addPrintMargins" 
          type="checkbox" 
          v-model="localSettings.addPrintMargins"
          @change="emitUpdate"
        />
        <label for="addPrintMargins">Добавить поля для печати</label>
      </div>
      
      <div class="form-group" v-if="localSettings.addPrintMargins">
        <label for="marginSize">Размер полей (мм):</label>
        <input 
          id="marginSize" 
          type="number" 
          v-model="localSettings.marginSize" 
          min="0" 
          max="20"
          @input="emitUpdate"
        />
      </div>
    </div>
    
    <div class="actions">
      <button @click="resetToDefaults" class="btn-secondary">
        Сбросить по умолчанию
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, watch } from 'vue'

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

// Входные параметры
const props = defineProps<{
  settings: PaperSettings
}>()

// События
const emit = defineEmits<{
  (e: 'update:settings', settings: PaperSettings): void
  (e: 'reset'): void
}>()

// Локальное состояние
const localSettings = reactive({ ...props.settings })

// Наблюдатель за изменениями входных параметров
watch(() => props.settings, (newSettings) => {
  Object.assign(localSettings, newSettings)
}, { deep: true })

// Функция для отправки обновлений
function emitUpdate() {
  emit('update:settings', { ...localSettings })
}

// Сброс параметров по умолчанию
function resetToDefaults() {
  emit('reset')
}
</script>

<style scoped>
.paper-settings {
  padding: 20px;
  background-color: #f8f9fa;
  border-radius: 8px;
}

.paper-settings h3 {
  margin-top: 0;
  color: #333;
  border-bottom: 1px solid #eee;
  padding-bottom: 10px;
}

.settings-group {
  margin-bottom: 20px;
}

.settings-group h4 {
  margin-top: 0;
  color: #333;
  border-bottom: 1px solid #eee;
  padding-bottom: 5px;
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
  justify-content: flex-end;
  margin-top: 20px;
}

.btn-secondary {
  padding: 8px 16px;
  border: 1px solid #6c757d;
  background-color: #6c757d;
  color: white;
  border-radius: 4px;
  cursor: pointer;
  font-size: 14px;
  font-weight: 500;
  transition: background-color 0.2s;
}

.btn-secondary:hover {
  background-color: #545b62;
}
</style>
</file>
<line_count>165</line_count>
</write_to_file>