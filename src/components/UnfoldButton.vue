<template>
  <div class="unfold-button-container">
    <!-- Кнопка запуска развёртки -->
    <button
      @click="handleUnfold"
      :disabled="isProcessing || !hasModel"
      class="unfold-button"
      :class="{ 'processing': isProcessing }"
    >
      <span v-if="isProcessing" class="spinner"></span>
      <span v-else>📐 Развернуть модель</span>
    </button>

    <!-- Индикатор прогресса -->
    <div v-if="isProcessing" class="progress-container">
      <div class="progress-bar">
        <div 
          class="progress-fill" 
          :style="{ width: `${progress}%` }"
        ></div>
      </div>
      <span class="progress-text">{{ progressMessage }}</span>
      <span class="progress-percent">{{ progress }}%</span>
    </div>

    <!-- Кнопка отмены -->
    <button
      v-if="isProcessing"
      @click="handleCancel"
      class="cancel-button"
    >
      ✕ Отмена
    </button>

    <!-- Сообщение об ошибке -->
    <div v-if="error" class="error-message">
      {{ error }}
    </div>

    <!-- Сообщение об успехе -->
    <div v-if="isReady" class="success-message">
      ✓ Развёртка завершена успешно
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, onBeforeUnmount } from 'vue'
import { useAsyncUnfold } from '@frontend/composables/useAsyncUnfold'
import { useProjectStore } from '@/stores/projectStore'

const projectStore = useProjectStore()

// Используем composable для асинхронной развёртки
const {
  isProcessing,
  progress,
  progressMessage,
  result,
  error,
  isReady,
  runUnfold,
  cancel,
  dispose,
} = useAsyncUnfold()

// Вычисляемое: есть ли загруженная модель
const hasModel = computed(() => projectStore.hasModel)

/**
 * Обработчик клика по кнопке развёртки.
 */
const handleUnfold = async () => {
  if (!projectStore.currentMesh) {
    projectStore.setError('Модель не загружена')
    return
  }

  try {
    // Конвертируем меш в формат для воркера
    const meshData = {
      name: projectStore.currentMesh.name || 'model',
      vertices: projectStore.currentMesh.vertices.map((v, i) => ({
        id: i,
        position: Array.isArray(v) ? Array.from(v) : [v.x, v.y, v.z],
      })),
      faces: projectStore.currentMesh.faces.map((f) => ({
        vertices: Array.isArray(f) ? Array.from(f) : [f.a, f.b, f.c],
      })),
    }

    const config = {
      algorithm: 'mds' as const,
      max_iterations: 100,
      tolerance: 1e-6,
      preserve_detail: true,
    }

    await runUnfold(meshData, config)

    // При успехе обновляем проект
    if (result.value) {
      projectStore.setUnfoldedResult(result.value)
    }
  } catch (err) {
    const errorMessage = err instanceof Error ? err.message : String(err)
    if (errorMessage !== 'Unfold cancelled by user') {
      projectStore.setError(errorMessage)
    }
  }
}

/**
 * Обработчик отмены развёртки.
 */
const handleCancel = () => {
  cancel()
}

// Очищаем ресурсы при уничтожении компонента
onBeforeUnmount(() => {
  dispose()
})
</script>

<style scoped>
.unfold-button-container {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 12px;
  padding: 20px;
  width: 100%;
  max-width: 400px;
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
  display: flex;
  align-items: center;
  gap: 8px;
  min-width: 200px;
  justify-content: center;
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

.unfold-button.processing {
  background: linear-gradient(135deg, #f093fb 0%, #f5576c 100%);
  animation: pulse 1.5s ease-in-out infinite;
}

@keyframes pulse {
  0%, 100% {
    box-shadow: 0 2px 10px rgba(245, 87, 108, 0.3);
  }
  50% {
    box-shadow: 0 2px 20px rgba(245, 87, 108, 0.6);
  }
}

.spinner {
  display: inline-block;
  width: 18px;
  height: 18px;
  border: 2px solid rgba(255, 255, 255, 0.3);
  border-radius: 50%;
  border-top-color: #ffffff;
  animation: spin 1s linear infinite;
}

@keyframes spin {
  to {
    transform: rotate(360deg);
  }
}

/* Индикатор прогресса */
.progress-container {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 8px;
  width: 100%;
  padding: 12px 16px;
  background: #f8f9fa;
  border-radius: 8px;
  border: 1px solid #e9ecef;
}

.progress-bar {
  width: 100%;
  height: 8px;
  background: #e9ecef;
  border-radius: 4px;
  overflow: hidden;
  position: relative;
}

.progress-fill {
  height: 100%;
  background: linear-gradient(90deg, #667eea 0%, #764ba2 100%);
  transition: width 0.3s ease;
  border-radius: 4px;
  position: relative;
  overflow: hidden;
}

.progress-fill::after {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: linear-gradient(
    90deg,
    transparent,
    rgba(255, 255, 255, 0.3),
    transparent
  );
  animation: shimmer 1.5s infinite;
}

@keyframes shimmer {
  0% {
    transform: translateX(-100%);
  }
  100% {
    transform: translateX(100%);
  }
}

.progress-text {
  font-size: 13px;
  color: #495057;
  font-weight: 500;
}

.progress-percent {
  font-size: 14px;
  font-weight: 600;
  color: #667eea;
}

/* Кнопка отмены */
.cancel-button {
  background: #f8f9fa;
  border: 1px solid #dee2e6;
  color: #dc3545;
  padding: 8px 16px;
  border-radius: 6px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
  display: flex;
  align-items: center;
  gap: 6px;
}

.cancel-button:hover {
  background: #dc3545;
  color: white;
  border-color: #dc3545;
}

/* Сообщение об ошибке */
.error-message {
  color: #721c24;
  background: #f8d7da;
  border: 1px solid #f5c6cb;
  border-radius: 6px;
  padding: 12px 16px;
  font-size: 14px;
  width: 100%;
  text-align: center;
  animation: shake 0.5s ease-in-out;
}

@keyframes shake {
  0%, 100% {
    transform: translateX(0);
  }
  25% {
    transform: translateX(-5px);
  }
  75% {
    transform: translateX(5px);
  }
}

/* Сообщение об успехе */
.success-message {
  color: #155724;
  background: #d4edda;
  border: 1px solid #c3e6cb;
  border-radius: 6px;
  padding: 12px 16px;
  font-size: 14px;
  width: 100%;
  text-align: center;
  animation: fadeIn 0.3s ease-in-out;
}

@keyframes fadeIn {
  from {
    opacity: 0;
    transform: translateY(-10px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}
</style>
