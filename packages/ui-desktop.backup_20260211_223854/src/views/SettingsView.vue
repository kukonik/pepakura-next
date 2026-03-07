<script setup lang='ts'>
import { ref, computed } from 'vue'
import { useAiStore } from '../stores/aiStore'

const aiStore = useAiStore()

// Методы для сохранения настроек
const saveSettings = () => {
  console.log('Настройки сохранены')
  // Здесь будет реальная логика сохранения
}

// Методы для проверки валидации
const isFormValid = computed(() => {
  const config = aiStore.currentProviderConfig
  if (aiStore.activeProvider === 'ollama' || aiStore.activeProvider === 'openai') {
    return config.endpoint && config.model
  }
  return config.endpoint && config.model && config.apiKey
})
</script>

<template>
  <div class="settings-view">
    <h2>Настройки AI</h2>
    
    <div class="provider-selection">
      <h3>Выбор провайдера</h3>
      <div class="provider-buttons">
        <button 
          v-for="provider in ['ollama', 'openai', 'custom']" 
          :key="provider"
          :class="{ active: aiStore.activeProvider === provider }"
          @click="aiStore.setActiveProvider(provider as any)"
          type="button"
        >
          {{ provider === 'ollama' ? 'Ollama' : provider === 'openai' ? 'OpenAI' : 'Пользовательский' }}
        </button>
      </div>
    </div>

    <div class="provider-config">
      <h3>Настройки {{ aiStore.activeProvider === 'ollama' ? 'Ollama' : aiStore.activeProvider === 'openai' ? 'OpenAI' : 'Пользовательский' }}</h3>
      
      <div class="form-group">
        <label for="endpoint">Endpoint:</label>
        <input 
          id="endpoint"
          v-model="aiStore.currentProviderConfig.endpoint" 
          type="text" 
          placeholder="URL API провайдера"
        />
      </div>

      <div class="form-group">
        <label for="model">Модель:</label>
        <input 
          id="model"
          v-model="aiStore.currentProviderConfig.model" 
          type="text" 
          placeholder="Название модели"
        />
      </div>

      <div class="form-group">
        <label for="temperature">Температура:</label>
        <input 
          id="temperature"
          v-model.number="aiStore.currentProviderConfig.temperature" 
          type="number" 
          min="0" 
          max="1" 
          step="0.1"
        />
        <span class="help-text">0.0 - детерминированные ответы, 1.0 - креативные</span>
      </div>

      <div class="form-group">
        <label for="maxTokens">Максимальное количество токенов:</label>
        <input 
          id="maxTokens"
          v-model.number="aiStore.currentProviderConfig.maxTokens" 
          type="number" 
          min="1"
        />
      </div>

      <div v-if="aiStore.activeProvider !== 'ollama'" class="form-group">
        <label for="apiKey">API Key:</label>
        <input 
          id="apiKey"
          v-model="aiStore.currentProviderConfig.apiKey" 
          type="password" 
          placeholder="Введите API ключ"
        />
      </div>
    </div>

    <div class="actions">
      <button @click="saveSettings" :disabled="!isFormValid" class="btn-primary">Сохранить настройки</button>
    </div>
  </div>
</template>

<style scoped>
.settings-view {
  padding: 20px;
  max-width: 800px;
  margin: 0 auto;
}

h2 {
  color: #333;
  margin-bottom: 20px;
}

.provider-selection {
  margin-bottom: 30px;
  padding: 20px;
  border-radius: 8px;
  background-color: #f5f5f5;
}

.provider-buttons {
  display: flex;
  gap: 10px;
  margin-bottom: 15px;
}

.provider-buttons button {
  padding: 10px 15px;
  border: 1px solid #ddd;
  background: white;
  cursor: pointer;
  border-radius: 4px;
  transition: all 0.2s;
}

.provider-buttons button:hover {
  background-color: #e9ecef;
}

.provider-buttons button.active {
  background-color: #007bff;
  color: white;
  border-color: #007bff;
}

.provider-config {
  margin-bottom: 30px;
  padding: 20px;
  border-radius: 8px;
  background-color: #f8f9fa;
}

.form-group {
  margin-bottom: 20px;
}

.form-group label {
  display: block;
  margin-bottom: 5px;
  font-weight: bold;
  color: #333;
}

.form-group input {
  width: 100%;
  padding: 10px;
  border: 1px solid #ddd;
  border-radius: 4px;
  box-sizing: border-box;
}

.help-text {
  display: block;
  margin-top: 5px;
  font-size: 0.9em;
  color: #666;
}

.actions {
  text-align: right;
}

.btn-primary {
  padding: 10px 20px;
  background-color: #007bff;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
}

.btn-primary:disabled {
  background-color: #6c757d;
  cursor: not-allowed;
}

.btn-primary:hover:not(:disabled) {
  background-color: #0056b3;
}
</style>
