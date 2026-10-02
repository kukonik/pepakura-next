<template>
  <div class="instruction-export">
    <div class="export-header">
      <h3>Экспорт инструкции</h3>
      <button @click="$emit('close')" class="close-btn">✕</button>
    </div>

    <div class="export-content">
      <!-- Превью инструкции -->
      <div class="instruction-preview">
        <h4>{{ instruction.model_name }}</h4>
        <div class="instruction-meta">
          <span class="difficulty" :class="instruction.difficulty.toLowerCase()">
            {{ instruction.difficulty }}
          </span>
          <span class="time">⏱ {{ instruction.total_time_minutes }} мин</span>
        </div>

        <div class="steps-preview">
          <div 
            v-for="step in instruction.steps" 
            :key="step.step_number"
            class="step-card"
          >
            <div class="step-number">{{ step.step_number }}</div>
            <div class="step-description">{{ step.description }}</div>
            <div class="step-time">{{ step.estimated_time_minutes }} мин</div>
          </div>
        </div>

        <div v-if="instruction.tips.length" class="tips-section">
          <h5>💡 Советы</h5>
          <ul>
            <li v-for="(tip, index) in instruction.tips" :key="index">
              {{ tip }}
            </li>
          </ul>
        </div>
      </div>

      <!-- Настройки экспорта -->
      <div class="export-settings">
        <h4>Настройки PDF</h4>
        
        <div class="setting">
          <label>Формат страницы</label>
          <select v-model="settings.pageSize">
            <option value="A4">A4 (210 × 297 мм)</option>
            <option value="A3">A3 (297 × 420 мм)</option>
            <option value="Letter">Letter (8.5 × 11")</option>
          </select>
        </div>

        <div class="setting">
          <label>Ориентация</label>
          <select v-model="settings.orientation">
            <option value="portrait">Портретная</option>
            <option value="landscape">Альбомная</option>
          </select>
        </div>

        <div class="setting">
          <label>
            <input type="checkbox" v-model="settings.includeDiagrams" />
            Включить диаграммы
          </label>
        </div>

        <div class="setting">
          <label>
            <input type="checkbox" v-model="settings.includeTips" />
            Включить советы
          </label>
        </div>

        <div class="setting">
          <label>
            <input type="checkbox" v-model="settings.color" />
            Цветной PDF
          </label>
        </div>
      </div>
    </div>

    <div class="export-actions">
      <button @click="exportPdf" :disabled="isExporting" class="primary-btn">
        {{ isExporting ? 'Экспорт...' : 'Экспорт в PDF' }}
      </button>
      <button @click="$emit('close')" class="secondary-btn">
        Отмена
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive } from 'vue'

export interface AssemblyInstruction {
  model_name: string
  difficulty: string
  total_time_minutes: number
  steps: AssemblyStep[]
  tips: string[]
}

export interface AssemblyStep {
  step_number: number
  description: string
  part_ids: number[]
  estimated_time_minutes: number
}

export interface PdfSettings {
  pageSize: 'A4' | 'A3' | 'Letter'
  orientation: 'portrait' | 'landscape'
  includeDiagrams: boolean
  includeTips: boolean
  color: boolean
}

const props = defineProps<{
  instruction: AssemblyInstruction
}>()

const emit = defineEmits<{
  close: []
  export: [settings: PdfSettings]
}>()

const isExporting = ref(false)

const settings = reactive<PdfSettings>({
  pageSize: 'A4',
  orientation: 'portrait',
  includeDiagrams: true,
  includeTips: true,
  color: false
})

async function exportPdf() {
  isExporting.value = true
  
  try {
    // Генерация PDF через браузерный print
    const printWindow = window.open('', '_blank')
    if (!printWindow) {
      throw new Error('Не удалось открыть окно печати')
    }

    const html = generatePdfHtml(props.instruction, settings)
    printWindow.document.write(html)
    printWindow.document.close()
    
    // Печать
    printWindow.focus()
    setTimeout(() => {
      printWindow.print()
      printWindow.close()
    }, 250)
    
    emit('export', settings)
  } catch (error) {
    console.error('Ошибка экспорта PDF:', error)
    alert(`Ошибка экспорта: ${error}`)
  } finally {
    isExporting.value = false
  }
}

function generatePdfHtml(instruction: AssemblyInstruction, settings: PdfSettings): string {
  return `
<!DOCTYPE html>
<html>
<head>
  <title>Инструкция: ${instruction.model_name}</title>
  <style>
    @page {
      size: ${settings.pageSize} ${settings.orientation};
      margin: 20mm;
    }
    body {
      font-family: Arial, sans-serif;
      line-height: 1.6;
      color: #333;
    }
    h1 {
      color: ${settings.color ? '#1976d2' : '#000'};
      border-bottom: 2px solid ${settings.color ? '#1976d2' : '#000'};
      padding-bottom: 10px;
    }
    .meta {
      margin: 20px 0;
      padding: 10px;
      background: #f5f5f5;
      border-radius: 4px;
    }
    .difficulty {
      display: inline-block;
      padding: 4px 12px;
      border-radius: 4px;
      font-weight: bold;
      margin-right: 10px;
    }
    .difficulty.easy { background: #4caf50; color: white; }
    .difficulty.medium { background: #ff9800; color: white; }
    .difficulty.hard { background: #f44336; color: white; }
    .difficulty.expert { background: #9c27b0; color: white; }
    .step {
      margin: 20px 0;
      padding: 15px;
      border: 1px solid #ddd;
      border-radius: 8px;
      page-break-inside: avoid;
    }
    .step-number {
      display: inline-block;
      width: 30px;
      height: 30px;
      line-height: 30px;
      text-align: center;
      background: ${settings.color ? '#1976d2' : '#333'};
      color: white;
      border-radius: 50%;
      margin-right: 10px;
      font-weight: bold;
    }
    .tips {
      margin-top: 30px;
      padding: 15px;
      background: #fff3cd;
      border-left: 4px solid #ffc107;
    }
    .tips h3 {
      margin-top: 0;
    }
    @media print {
      body { print-color-adjust: exact; -webkit-print-color-adjust: exact; }
    }
  </style>
</head>
<body>
  <h1>📐 Инструкция по сборке</h1>
  <h2>${instruction.model_name}</h2>
  
  <div class="meta">
    <span class="difficulty ${instruction.difficulty.toLowerCase()}">
      ${instruction.difficulty}
    </span>
    <span>⏱ ${instruction.total_time_minutes} мин</span>
    <span>📦 ${instruction.steps.length} шагов</span>
  </div>

  <div class="steps">
    ${instruction.steps.map(step => `
      <div class="step">
        <span class="step-number">${step.step_number}</span>
        <strong>${step.description}</strong>
        ${step.part_ids.length > 0 ? `<br><small>Детали: ${step.part_ids.join(', ')}</small>` : ''}
        <br><small>⏱ ~${step.estimated_time_minutes} мин</small>
      </div>
    `).join('')}
  </div>

  ${settings.includeTips && instruction.tips.length ? `
    <div class="tips">
      <h3>💡 Советы</h3>
      <ul>
        ${instruction.tips.map(tip => `<li>${tip}</li>`).join('')}
      </ul>
    </div>
  ` : ''}
</body>
</html>
  `
}
</script>

<style scoped>
.instruction-export {
  display: flex;
  flex-direction: column;
  height: 100%;
}

.export-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 16px;
  border-bottom: 1px solid #ddd;
}

.export-header h3 {
  margin: 0;
}

.close-btn {
  background: none;
  border: none;
  font-size: 24px;
  cursor: pointer;
  color: #666;
}

.export-content {
  flex: 1;
  overflow-y: auto;
  padding: 16px;
  display: grid;
  grid-template-columns: 1fr 300px;
  gap: 20px;
}

.instruction-preview {
  background: #f9f9f9;
  padding: 20px;
  border-radius: 8px;
}

.instruction-preview h4 {
  margin-top: 0;
}

.instruction-meta {
  display: flex;
  gap: 10px;
  margin-bottom: 20px;
}

.difficulty {
  padding: 4px 12px;
  border-radius: 4px;
  font-weight: bold;
  font-size: 12px;
}

.difficulty.easy { background: #4caf50; color: white; }
.difficulty.medium { background: #ff9800; color: white; }
.difficulty.hard { background: #f44336; color: white; }
.difficulty.expert { background: #9c27b0; color: white; }

.steps-preview {
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.step-card {
  display: flex;
  gap: 10px;
  padding: 12px;
  background: white;
  border-radius: 6px;
  border: 1px solid #ddd;
}

.step-number {
  width: 28px;
  height: 28px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: #1976d2;
  color: white;
  border-radius: 50%;
  font-weight: bold;
  flex-shrink: 0;
}

.step-description {
  flex: 1;
}

.step-time {
  color: #666;
  font-size: 12px;
  white-space: nowrap;
}

.tips-section {
  margin-top: 20px;
  padding: 15px;
  background: #fff3cd;
  border-radius: 6px;
}

.tips-section h5 {
  margin-top: 0;
}

.tips-section ul {
  margin: 10px 0;
  padding-left: 20px;
}

.export-settings {
  background: #f5f5f5;
  padding: 16px;
  border-radius: 8px;
}

.export-settings h4 {
  margin-top: 0;
}

.setting {
  margin-bottom: 16px;
}

.setting label {
  display: block;
  margin-bottom: 4px;
  font-weight: 500;
}

.setting select {
  width: 100%;
  padding: 8px;
  border: 1px solid #ddd;
  border-radius: 4px;
}

.setting input[type="checkbox"] {
  margin-right: 8px;
}

.export-actions {
  display: flex;
  gap: 10px;
  padding: 16px;
  border-top: 1px solid #ddd;
}

.primary-btn,
.secondary-btn {
  padding: 10px 20px;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-weight: 500;
}

.primary-btn {
  background: #1976d2;
  color: white;
  flex: 1;
}

.primary-btn:disabled {
  background: #ccc;
  cursor: not-allowed;
}

.secondary-btn {
  background: #e0e0e0;
}
</style>
