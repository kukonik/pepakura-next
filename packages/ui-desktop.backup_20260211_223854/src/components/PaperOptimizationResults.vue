
<template>
  <div class="optimization-results">
    <h3>Результаты оптимизации</h3>
    
    <div class="results-grid">
      <div class="result-card">
        <div class="result-value">{{ paperUsage.sheetCount }}</div>
        <div class="result-label">Листов бумаги</div>
      </div>
      
      <div class="result-card">
        <div class="result-value">{{ paperUsage.usagePercentage.toFixed(1) }}%</div>
        <div class="result-label">Использование бумаги</div>
      </div>
      
      <div class="result-card">
        <div class="result-value">{{ (paperUsage.modelArea / 1000000).toFixed(2) }}</div>
        <div class="result-label">Площадь модели (м²)</div>
      </div>
    </div>
    
    <div class="tips-section" v-if="assemblyTips.length > 0">
      <h4>Рекомендации по сборке</h4>
      <ul class="tips-list">
        <li v-for="(tip, index) in assemblyTips" :key="index">
          {{ tip }}
        </li>
      </ul>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'

// Определение типов
interface PaperUsage {
  sheetCount: number
  usagePercentage: number
  modelArea: number
  usedArea: number
}

// Входные параметры
const props = defineProps<{
  paperUsage: PaperUsage
  assemblyTips: string[]
}>()

// Вычисляемые свойства
const formattedModelArea = computed(() => {
  return (props.paperUsage.modelArea / 1000000).toFixed(2) // Преобразуем мм² в м²
})
</script>

<style scoped>
.optimization-results {
  padding: 20px;
  background-color: #f8f9fa;
  border-radius: 8px;
  margin-top: 20px;
}

.optimization-results h3 {
  margin-top: 0;
  color: #333;
  border-bottom: 1px solid #eee;
  padding-bottom: 10px;
}

.results-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(150px, 1fr));
  gap: 15px;
  margin: 20px 0;
}

.result-card {
  background-color: white;
  border-radius: 6px;
  padding: 15px;
  text-align: center;
  box-shadow: 0 2px 4px rgba(0,0,0,0.1);
}

.result-value {
  font-size: 24px;
  font-weight: bold;
  color: #007bff;
  margin-bottom: 5px;
}

.result-label {
  font-size: 14px;
  color: #666;
}

.tips-section {
  margin-top: 20px;
}

.tips-section h4 {
  margin-top: 0;
  color: #333;
}

.tips-list {
  padding-left: 20px;
  margin: 0;
}

.tips-list li {
  margin-bottom: 8px;
  color: #666;
}
</style>
</file>
<line_count>85</line_count>
</write_to_file>