<template>
  <div class="auto-save-indicator" :class="{ 'saving': isSaving, 'error': hasError }">
    <div class="indicator-content">
      <div class="status-icon">
        <div v-if="isSaving" class="spinner"></div>
        <div v-else-if="hasError" class="error-icon">⚠️</div>
        <div v-else-if="isEnabled" class="success-icon">💾</div>
        <div v-else class="disabled-icon">🚫</div>
      </div>
      
      <div class="status-text">
        <div class="status-message">
          <span v-if="isSaving">Сохранение...</span>
          <span v-else-if="hasError">Ошибка сохранения</span>
          <span v-else-if="isEnabled">
            Автосохранение включено
            <span v-if="lastSaveTime" class="last-save">
              (последнее: {{ formatTime(lastSaveTime) }})
            </span>
          </span>
          <span v-else>Автосохранение отключено</span>
        </div>
        
        <div v-if="hasError" class="error-details">
          {{ errorMessage }}
        </div>
      </div>
    </div>
    
    <div class="indicator-actions">
      <button 
        @click="toggleAutoSave"
        class="toggle-button"
        :class="{ 'enabled': isEnabled }"
      >
        {{ isEnabled ? 'Выключить' : 'Включить' }}
      </button>
      
      <button 
        @click="saveNow"
        :disabled="isSaving"
        class="save-now-button"
      >
        Сохранить сейчас
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useAutoSave } from '@/composables/useAutoSave'

// Получаем состояние автосохранения
const { 
  autoSaveState, 
  startAutoSave, 
  stopAutoSave, 
  saveNow: triggerSaveNow,
  setAutoSaveEnabled,
  setAutoSaveInterval
} = useAutoSave()

// Вычисляемые свойства для удобства использования
const isEnabled = computed(() => autoSaveState.value.isEnabled)
const isSaving = computed(() => autoSaveState.value.isSaving)
const hasError = computed(() => !!autoSaveState.value.error)
const errorMessage = computed(() => autoSaveState.value.error)
const lastSaveTime = computed(() => autoSaveState.value.lastSaveTime)

// Функция для переключения автосохранения
const toggleAutoSave = () => {
  if (isEnabled.value) {
    stopAutoSave()
    setAutoSaveEnabled(false)
  } else {
    startAutoSave()
    setAutoSaveEnabled(true)
  }
}

// Функция для немедленного сохранения
const saveNow = async () => {
  await triggerSaveNow()
}

// Функция для форматирования времени
const formatTime = (date: Date | null): string => {
  if (!date) return 'никогда'
  
  const now = new Date()
  const diffMs = now.getTime() - date.getTime()
  const diffMins = Math.floor(diffMs / 60000)
  
  if (diffMins < 1) {
    return 'только что'
  } else if (diffMins < 60) {
    return `${diffMins} мин. назад`
  } else {
    const diffHours = Math.floor(diffMins / 60)
    return `${diffHours} ч. назад`
  }
}
</script>

<style scoped>
.auto-save-indicator {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 12px 16px;
  background-color: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: 8px;
  font-size: 0.9rem;
  transition: all 0.3s ease;
}

.auto-save-indicator.saving {
  background-color: var(--primary-color-light);
  border-color: var(--primary-color);
}

.auto-save-indicator.error {
  background-color: var(--error-bg);
  border-color: var(--error-border);
}

.indicator-content {
  display: flex;
  align-items: center;
  gap: 12px;
}

.status-icon {
  font-size: 1.2rem;
  width: 24px;
  height: 24px;
  display: flex;
  align-items: center;
  justify-content: center;
}

.spinner {
  width: 16px;
  height: 16px;
  border: 2px solid var(--text-secondary);
  border-top: 2px solid var(--primary-color);
  border-radius: 50%;
  animation: spin 1s linear infinite;
}

@keyframes spin {
  0% { transform: rotate(0deg); }
  100% { transform: rotate(360deg); }
}

.status-text {
  display: flex;
  flex-direction: column;
}

.status-message {
  font-weight: 500;
  color: var(--text-primary);
}

.last-save {
  font-weight: normal;
  color: var(--text-secondary);
  font-size: 0.85rem;
}

.error-details {
  color: var(--error-text);
  font-size: 0.85rem;
  margin-top: 2px;
}

.indicator-actions {
  display: flex;
  gap: 8px;
}

.toggle-button {
  padding: 6px 12px;
  background-color: var(--bg-input);
  color: var(--text-primary);
  border: 1px solid var(--border-color);
  border-radius: 4px;
  font-size: 0.85rem;
  cursor: pointer;
  transition: all 0.2s;
}

.toggle-button:hover {
  background-color: var(--bg-hover);
}

.toggle-button.enabled {
  background-color: var(--primary-color);
  color: white;
  border-color: var(--primary-color);
}

.toggle-button.enabled:hover {
  background-color: var(--primary-color-dark);
}

.save-now-button {
  padding: 6px 12px;
  background-color: var(--bg-secondary);
  color: var(--text-primary);
  border: 1px solid var(--border-color);
  border-radius: 4px;
  font-size: 0.85rem;
  cursor: pointer;
  transition: all 0.2s;
}

.save-now-button:hover:not(:disabled) {
  background-color: var(--bg-hover);
}

.save-now-button:disabled {
  background-color: var(--bg-disabled);
  cursor: not-allowed;
}

@media (max-width: 768px) {
  .auto-save-indicator {
    flex-direction: column;
    align-items: flex-start;
    gap: 12px;
  }
  
  .indicator-actions {
    align-self: flex-end;
  }
}
</style>