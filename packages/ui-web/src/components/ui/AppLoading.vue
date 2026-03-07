<template>
  <div class="app-loading" :class="{ 'fullscreen': fullscreen }">
    <div class="loading-content">
      <div class="logo-animation">
        <svg class="logo-icon" viewBox="0 0 100 100">
          <polygon 
            points="50,10 90,30 90,70 50,90 10,70 10,30" 
            fill="none" 
            stroke="var(--color-primary)" 
            stroke-width="3"
          />
          <path 
            d="M50,10 L50,90 M10,30 L90,70 M90,30 L10,70" 
            stroke="var(--color-secondary)" 
            stroke-width="2" 
            stroke-dasharray="5,5"
          />
        </svg>
      </div>
      
      <div v-if="progress >= 0" class="progress-container">
        <div class="progress-bar">
          <div 
            class="progress-fill" 
            :style="{ width: `${progress}%` }"
          ></div>
        </div>
        <span class="progress-text">{{ progress }}%</span>
      </div>
      
      <div v-if="message" class="loading-message">
        {{ message }}{{ dots }}
      </div>
      
      <div v-else class="spinner">
        <div class="spinner-sector spinner-sector-primary"></div>
        <div class="spinner-sector spinner-sector-secondary"></div>
        <div class="spinner-sector spinner-sector-accent"></div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'

interface Props {
  fullscreen?: boolean
  progress?: number
  message?: string
}

const props = withDefaults(defineProps<Props>(), {
  fullscreen: true,
  progress: -1,
  message: ''
})

const dots = ref('')
const intervalId = ref<number | undefined>()

onMounted(() => {
  intervalId.value = window.setInterval(() => {
    dots.value = dots.value.length >= 3 ? '' : dots.value + '.'
  }, 500)
})

onUnmounted(() => {
  if (intervalId.value !== undefined) {
    clearInterval(intervalId.value)
  }
})
</script>

<style scoped>
.app-loading {
  display: flex;
  align-items: center;
  justify-content: center;
  background-color: var(--color-background);
  transition: background-color 0.3s ease;
}

.app-loading.fullscreen {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  z-index: 9999;
}

.loading-content {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 2rem;
  padding: 3rem;
  border-radius: var(--radius-xl);
  background-color: var(--color-surface);
  box-shadow: var(--shadow-xl);
  max-width: 400px;
  width: 90%;
}

.logo-animation {
  width: 120px;
  height: 120px;
  animation: float 3s ease-in-out infinite;
}

.logo-icon {
  width: 100%;
  height: 100%;
  animation: rotate 20s linear infinite;
}

@keyframes float {
  0%, 100% {
    transform: translateY(0);
  }
  50% {
    transform: translateY(-20px);
  }
}

@keyframes rotate {
  from {
    transform: rotate(0deg);
  }
  to {
    transform: rotate(360deg);
  }
}

.progress-container {
  width: 100%;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 0.5rem;
}

.progress-bar {
  width: 100%;
  height: 8px;
  background-color: var(--color-background);
  border-radius: 4px;
  overflow: hidden;
}

.progress-fill {
  height: 100%;
  background: linear-gradient(
    90deg,
    var(--color-primary),
    var(--color-secondary)
  );
  border-radius: 4px;
  transition: width 0.3s ease;
}

.progress-text {
  font-size: 0.875rem;
  color: var(--color-text-secondary);
  font-weight: 500;
}

.loading-message {
  font-size: 1rem;
  color: var(--color-text);
  text-align: center;
  min-height: 1.5em;
}

/* Спиннер */
.spinner {
  position: relative;
  width: 60px;
  height: 60px;
}

.spinner-sector {
  position: absolute;
  width: 100%;
  height: 100%;
  border-radius: 50%;
  border: 4px solid transparent;
  animation: spin 1.2s cubic-bezier(0.5, 0, 0.5, 1) infinite;
}

.spinner-sector-primary {
  border-top-color: var(--color-primary);
  animation-delay: -0.45s;
}

.spinner-sector-secondary {
  border-top-color: var(--color-secondary);
  animation-delay: -0.3s;
}

.spinner-sector-accent {
  border-top-color: var(--color-info);
  animation-delay: -0.15s;
}

@keyframes spin {
  0% {
    transform: rotate(0deg);
  }
  100% {
    transform: rotate(360deg);
  }
}
</style>
