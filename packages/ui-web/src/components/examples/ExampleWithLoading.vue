<template>
  <div class="example-with-loading">
    <h2>Пример использования AppLoading</h2>
    
    <div class="controls">
      <button @click="toggleLoading">Переключить загрузку</button>
      <button @click="startProgress">Запустить прогресс</button>
    </div>
    
    <!-- Пример 1: Полноэкранная загрузка -->
    <AppLoading 
      v-if="showFullscreenLoading"
      :fullscreen="true"
      :message="fullscreenMessage"
    />
    
    <!-- Пример 2: Прогресс бар -->
    <div v-else-if="showProgress" class="progress-example">
      <h3>Прогресс загрузки: {{ progress }}%</h3>
      <AppLoading 
        :progress="progress"
        :message="'Загрузка данных...'"
      />
    </div>
    
    <!-- Пример 3: Простой спиннер -->
    <div v-else class="spinner-example">
      <h3>Спиннер без прогресса</h3>
      <AppLoading :message="'Ожидание...'" />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import AppLoading from '@components/ui/AppLoading.vue'

const showFullscreenLoading = ref(false)
const showProgress = ref(false)
const progress = ref(0)
const fullscreenMessage = ref('Загрузка данных...')

const toggleLoading = () => {
  showFullscreenLoading.value = !showFullscreenLoading.value
  showProgress.value = false
  
  if (showFullscreenLoading.value) {
    fullscreenMessage.value = 'Идет загрузка контента...'
    // Симулируем загрузку
    setTimeout(() => {
      showFullscreenLoading.value = false
    }, 3000)
  }
}

const startProgress = () => {
  showFullscreenLoading.value = false
  showProgress.value = true
  progress.value = 0
  
  const interval = setInterval(() => {
    progress.value += 10
    if (progress.value >= 100) {
      clearInterval(interval)
      setTimeout(() => {
        showProgress.value = false
      }, 1000)
    }
  }, 500)
}
</script>

<style scoped>
.example-with-loading {
  padding: 2rem;
  border: 2px dashed var(--color-primary);
  border-radius: 12px;
  background: var(--color-surface);
  margin: 2rem auto;
  max-width: 600px;
}

.controls {
  display: flex;
  gap: 1rem;
  margin: 1rem 0;
}

button {
  padding: 0.5rem 1rem;
  background: var(--color-primary);
  color: white;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  font-size: 1rem;
}

button:hover {
  background: color-mix(in srgb, var(--color-primary) 90%, black);
}

.progress-example,
.spinner-example {
  margin-top: 2rem;
  padding: 1rem;
  background: rgba(0, 0, 0, 0.05);
  border-radius: 8px;
}

h2, h3 {
  color: var(--color-text);
  margin-bottom: 1rem;
}
</style>
