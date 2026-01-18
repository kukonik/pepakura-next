<template>
  <div class="test-loading-view">
    <h1>Тестирование компонента AppLoading</h1>
    
    <div class="test-cases">
      <div class="test-case">
        <h2>1. Полноэкранная загрузка</h2>
        <button @click="testFullscreenLoading">Запустить полноэкранную загрузку</button>
        
        <AppLoading 
          v-if="fullscreenLoading.active"
          :fullscreen="true"
          :message="fullscreenLoading.message"
        />
      </div>
      
      <div class="test-case">
        <h2>2. Прогресс бар</h2>
        <button @click="startProgress">Запустить прогресс</button>
        <button @click="resetProgress">Сбросить</button>
        
        <div v-if="progress.active" class="progress-container">
          <AppLoading 
            :progress="progress.value"
            :message="`Загрузка: ${progress.value}%`"
          />
        </div>
      </div>
      
      <div class="test-case">
        <h2>3. Спиннер с сообщением</h2>
        <AppLoading :message="'Ожидание завершения операции...'" />
      </div>
      
      <div class="test-case">
        <h2>4. Спиннер без сообщения</h2>
        <AppLoading />
      </div>
      
      <div class="test-case">
        <h2>5. Кастомное сообщение</h2>
        <input v-model="customMessage" placeholder="Введите сообщение..." />
        <AppLoading :message="customMessage" />
      </div>
    </div>
    
    <div class="navigation">
      <router-link to="/">Вернуться на главную</router-link>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive } from 'vue'
import AppLoading from '@components/ui/AppLoading.vue'

// Тест 1: Полноэкранная загрузка
const fullscreenLoading = reactive({
  active: false,
  message: 'Идет загрузка данных...'
})

const testFullscreenLoading = () => {
  fullscreenLoading.active = true
  fullscreenLoading.message = 'Загрузка тестовых данных...'
  
  setTimeout(() => {
    fullscreenLoading.active = false
  }, 2500)
}

// Тест 2: Прогресс бар
const progress = reactive({
  active: false,
  value: 0
})

const startProgress = () => {
  progress.active = true
  progress.value = 0
  
  const interval = setInterval(() => {
    progress.value += 5
    if (progress.value >= 100) {
      clearInterval(interval)
      setTimeout(() => {
        progress.active = false
      }, 1000)
    }
  }, 200)
}

const resetProgress = () => {
  progress.active = false
  progress.value = 0
}

// Тест 5: Кастомное сообщение
const customMessage = ref('Мое кастомное сообщение загрузки...')
</script>

<style scoped>
.test-loading-view {
  padding: 2rem;
  max-width: 1200px;
  margin: 0 auto;
}

h1 {
  color: var(--color-primary);
  text-align: center;
  margin-bottom: 2rem;
}

.test-cases {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
  gap: 2rem;
  margin-bottom: 3rem;
}

.test-case {
  padding: 1.5rem;
  background: var(--color-surface);
  border-radius: 12px;
  box-shadow: var(--shadow-md);
  border: 1px solid var(--color-border);
}

.test-case h2 {
  color: var(--color-text);
  font-size: 1.25rem;
  margin-bottom: 1rem;
}

button {
  padding: 0.5rem 1rem;
  background: var(--color-primary);
  color: white;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  margin-right: 0.5rem;
  margin-bottom: 1rem;
  font-size: 0.9rem;
}

button:hover {
  background: color-mix(in srgb, var(--color-primary) 90%, black);
}

button:active {
  transform: translateY(1px);
}

input {
  width: 100%;
  padding: 0.5rem;
  border: 1px solid var(--color-border);
  border-radius: 6px;
  margin-bottom: 1rem;
  font-size: 1rem;
}

.progress-container {
  margin-top: 1rem;
  padding: 1rem;
  background: rgba(0, 0, 0, 0.05);
  border-radius: 8px;
}

.navigation {
  text-align: center;
  margin-top: 2rem;
  padding-top: 2rem;
  border-top: 1px solid var(--color-border);
}

.navigation a {
  color: var(--color-primary);
  text-decoration: none;
  font-size: 1.1rem;
  padding: 0.75rem 1.5rem;
  border: 2px solid var(--color-primary);
  border-radius: 8px;
  display: inline-block;
}

.navigation a:hover {
  background: var(--color-primary);
  color: white;
}
</style>
