<template>
  <div class="unfold-button-container">
    <button 
      @click="handleUnfold" 
      :disabled="projectStore.isProcessing || !projectStore.hasModel"
      class="unfold-button"
    >
      <span v-if="projectStore.isProcessing" class="spinner"></span>
      <span v-else>📐 Развернуть модель</span>
    </button>
    
    <div v-if="projectStore.error" class="error-message">
      {{ projectStore.error }}
    </div>
  </div>
</template>

<script setup lang="ts">
import { useProjectStore } from '@/stores/projectStore'

const projectStore = useProjectStore()

const handleUnfold = async () => {
  await projectStore.unfoldModel()
}
</script>

<style scoped>
.unfold-button-container {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 10px;
  padding: 20px;
}

.unfold-button {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  border: none;
  color: white;
  padding: 12px 24px;
  border-radius: 6px;
  font-size: 16px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.3s ease;
  box-shadow: 0 2px 10px rgba(0, 0, 0, 0.1);
}

.unfold-button:hover:not(:disabled) {
  transform: translateY(-2px);
  box-shadow: 0 4px 15px rgba(0, 0, 0, 0.2);
}

.unfold-button:disabled {
  opacity: 0.6;
  cursor: not-allowed;
  transform: none;
}

.spinner {
  display: inline-block;
  width: 20px;
  height: 20px;
  border: 2px solid #ffffff;
  border-radius: 50%;
  border-top-color: transparent;
  animation: spin 1s linear infinite;
}

@keyframes spin {
  to {
    transform: rotate(360deg);
  }
}

.error-message {
  color: #e74c3c;
  background: #fdf2f2;
  border: 1px solid #f5c6cb;
  border-radius: 4px;
  padding: 10px 15px;
  font-size: 14px;
  max-width: 300px;
  text-align: center;
}
</style>