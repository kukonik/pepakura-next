<template>
  <div class="auto-seams-view">
    <div class="header">
      <h2>Автоматическое создание швов</h2>
      <p>Используйте AI для автоматического определения оптимальных швов на модели</p>
    </div>
    
    <div class="content">
      <div class="settings-panel">
        <div class="setting-group">
          <label for="ai-model">AI Модель:</label>
          <select id="ai-model" v-model="selectedModel">
            <option value="llama3">Llama 3</option>
            <option value="gpt-4">GPT-4</option>
            <option value="custom">Пользовательская</option>
          </select>
        </div>
        
        <div class="setting-group">
          <label for="complexity">Сложность швов:</label>
          <select id="complexity" v-model="complexity">
            <option value="low">Низкая</option>
            <option value="medium">Средняя</option>
            <option value="high">Высокая</option>
          </select>
        </div>
        
        <div class="setting-group">
          <label for="optimization">Оптимизация:</label>
          <select id="optimization" v-model="optimization">
            <option value="speed">Скорость</option>
            <option value="quality">Качество</option>
            <option value="balance">Баланс</option>
          </select>
        </div>
        
        <button @click="generateSeams" class="generate-btn">
          Сгенерировать швы
        </button>
      </div>
      
      <div class="visualization-panel">
        <div class="model-viewer">
          <!-- 3D визуализация модели с швами -->
          <div class="viewer-placeholder">
            <p>3D визуализация модели</p>
          </div>
        </div>
        
        <div class="seam-info">
          <h3>Информация о швах</h3>
          <div class="seam-stats">
            <p>Количество швов: {{ seamCount }}</p>
            <p>Общая длина: {{ totalLength }} мм</p>
            <p>Оценка качества: {{ qualityScore }}/10</p>
          </div>
          
          <div class="actions">
            <button @click="applySeams" class="apply-btn">
              Применить швы
            </button>
            <button @click="resetSeams" class="reset-btn">
              Сбросить
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { useSeamsStore } from '../stores/seams.store';

// Хранилище швов
const seamsStore = useSeamsStore();

// Настройки AI
const selectedModel = ref('llama3');
const complexity = ref('medium');
const optimization = ref('balance');

// Статистика швов
const seamCount = ref(0);
const totalLength = ref(0);
const qualityScore = ref(0);

/**
 * Генерация швов с помощью AI
 */
async function generateSeams() {
  try {
    // TODO: Вызов AI сервиса для генерации швов
    console.log('Генерация швов с помощью AI...');
    
    // Имитация получения результатов
    seamCount.value = Math.floor(Math.random() * 50) + 10;
    totalLength.value = Math.floor(Math.random() * 1000) + 100;
    qualityScore.value = Math.floor(Math.random() * 4) + 7;
  } catch (error) {
    console.error('Ошибка при генерации швов:', error);
  }
}

/**
 * Применение швов
 */
function applySeams() {
  // TODO: Применение сгенерированных швов к модели
  console.log('Применение швов...');
}

/**
 * Сброс швов
 */
function resetSeams() {
  seamCount.value = 0;
  totalLength.value = 0;
  qualityScore.value = 0;
}
</script>

<style scoped>
.auto-seams-view {
  display: flex;
  flex-direction: column;
  height: 100%;
  padding: 20px;
  background-color: var(--bg-primary);
}

.header {
  margin-bottom: 20px;
}

.header h2 {
  margin: 0 0 10px 0;
  font-size: 24px;
  font-weight: 600;
}

.header p {
  margin: 0;
  color: var(--text-secondary);
}

.content {
  display: flex;
  flex: 1;
  gap: 20px;
}

.settings-panel {
  width: 300px;
  padding: 20px;
  background-color: var(--bg-secondary);
  border-radius: 8px;
  border: 1px solid var(--border-color);
}

.setting-group {
  margin-bottom: 20px;
}

.setting-group label {
  display: block;
  margin-bottom: 5px;
  font-weight: 500;
}

.setting-group select {
  width: 100%;
  padding: 8px;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  background-color: var(--bg-primary);
}

.generate-btn {
  width: 100%;
  padding: 12px;
  background-color: var(--primary-color);
  color: white;
  border: none;
  border-radius: 4px;
  font-size: 16px;
  font-weight: 500;
  cursor: pointer;
}

.generate-btn:hover {
  background-color: var(--primary-hover);
}

.visualization-panel {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 20px;
}

.model-viewer {
  flex: 1;
  background-color: var(--bg-secondary);
  border-radius: 8px;
  border: 1px solid var(--border-color);
  overflow: hidden;
}

.viewer-placeholder {
  display: flex;
  align-items: center;
  justify-content: center;
  height: 100%;
  color: var(--text-secondary);
}

.seam-info {
  padding: 20px;
  background-color: var(--bg-secondary);
  border-radius: 8px;
  border: 1px solid var(--border-color);
}

.seam-info h3 {
  margin: 0 0 15px 0;
  font-size: 18px;
  font-weight: 600;
}

.seam-stats p {
  margin: 8px 0;
  font-size: 14px;
}

.actions {
  display: flex;
  gap: 10px;
  margin-top: 20px;
}

.apply-btn, .reset-btn {
  flex: 1;
  padding: 10px;
  border: none;
  border-radius: 4px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
}

.apply-btn {
  background-color: var(--primary-color);
  color: white;
}

.apply-btn:hover {
  background-color: var(--primary-hover);
}

.reset-btn {
  background-color: var(--btn-secondary-bg);
  color: var(--btn-secondary-text);
  border: 1px solid var(--btn-secondary-border);
}

.reset-btn:hover {
  background-color: var(--btn-secondary-hover);
}
</style>