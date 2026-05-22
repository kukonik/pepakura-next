<template>
  <div class="nesting-controller">
    <h3>Вложение (Nesting)</h3>
    <div class="nesting-controls">
      <div class="control-group">
        <label for="nesting-quality">Качество вложения:</label>
        <select id="nesting-quality" v-model="nestingParams.quality" :disabled="isProcessing">
          <option value="low">Низкое</option>
          <option value="medium">Среднее</option>
          <option value="high">Высокое</option>
        </select>
      </div>
      
      <div class="control-group">
        <label for="nesting-margin">Отступ между деталями (мм):</label>
        <input 
          id="nesting-margin" 
          type="number" 
          v-model.number="nestingParams.margin" 
          min="0" 
          max="50" 
          step="0.1"
          :disabled="isProcessing"
        />
      </div>
      
      <div class="control-group">
        <label for="nesting-rotation">Шаг поворота (градусы):</label>
        <input 
          id="nesting-rotation" 
          type="number" 
          v-model.number="nestingParams.rotationStep" 
          min="1" 
          max="90" 
          step="1"
          :disabled="isProcessing"
        />
      </div>
      
      <div class="control-group checkbox-group">
        <input 
          id="allow-flipping" 
          type="checkbox" 
          v-model="nestingParams.allowFlipping" 
          :disabled="isProcessing"
        />
        <label for="allow-flipping">Разрешить переворот деталей</label>
      </div>
      
      <button 
        @click="startNesting" 
        :disabled="isProcessing || !hasUnfolds"
        class="nest-button"
      >
        {{ isProcessing ? 'Обработка...' : 'Выполнить вложение' }}
      </button>
    </div>
    
    <div v-if="error" class="error-message">
      {{ error }}
    </div>
    
    <div v-if="successMessage" class="success-message">
      {{ successMessage }}
    </div>
    
    <div class="nesting-info" v-if="nestingResult">
      <h4>Результаты вложения:</h4>
      <ul>
        <li>Использовано листов: {{ nestingResult.sheetCount }}</li>
        <li>Общее количество деталей: {{ nestingResult.totalParts }}</li>
        <li>Процент заполнения: {{ (nestingResult.utilization * 100).toFixed(2) }}%</li>
        <li>Время обработки: {{ nestingResult.processingTime }} мс</li>
      </ul>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, reactive } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { useProjectStore } from '@/stores/projectStore'
import type { NestResult } from '@/../../../shared/src/types/nesting'

interface NestingParams {
  quality: 'low' | 'medium' | 'high'
  margin: number
  rotationStep: number
  allowFlipping: boolean
}

const projectStore = useProjectStore()
const isProcessing = ref(false)
const error = ref<string | null>(null)
const successMessage = ref<string | null>(null)

const nestingParams = reactive<NestingParams>({
  quality: 'medium',
  margin: 2.0,
  rotationStep: 15,
  allowFlipping: true
})

// Проверяем, есть ли развертки для вложения
const hasUnfolds = computed(() => {
  return !!projectStore.currentProject
})

const startNesting = async () => {
  if (isProcessing.value) return
  if (!projectStore.currentProject) {
    error.value = 'Нет загруженного проекта'
    return
  }
  
  isProcessing.value = true
  error.value = null
  successMessage.value = null
  
  try {
    // Вызываем Tauri команду для вложения
    const result = await invoke<NestResult>('nest_project', {
      project: projectStore.currentProject,
      params: {
        quality: nestingParams.quality,
        marginMm: nestingParams.margin,
        rotationStep: nestingParams.rotationStep,
        allowFlipping: nestingParams.allowFlipping
      }
    })
    
    successMessage.value = 'Вложение успешно выполнено!'
    // Обновляем состояние проекта с результатами вложения
    projectStore.nestResult = result
    console.log('Nesting result:', result)
  } catch (err: any) {
    error.value = err.message || 'Ошибка при выполнении вложения'
    console.error('Failed to perform nesting:', err)
  } finally {
    isProcessing.value = false
  }
}
</script>

<style scoped>
.nesting-controller {
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

.nest-button {
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

.nest-button:hover:not(:disabled) {
  background-color: var(--primary-color-dark);
}

.nest-button:disabled {
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

.nesting-info {
  margin-top: 20px;
  padding: 16px;
  background-color: var(--bg-tertiary);
  border-radius: 4px;
}

.nesting-info h4 {
  margin-top: 0;
  margin-bottom: 12px;
  color: var(--text-primary);
}

.nesting-info ul {
  margin: 0;
  padding-left: 20px;
}

.nesting-info li {
  margin-bottom: 8px;
  color: var(--text-secondary);
}
</style>