<script setup lang="ts">
import { ref, onUnmounted } from 'vue'
import { usePlatform } from '@pepakura/shared/composables/usePlatform'

const { getBridge } = usePlatform()

const progress = ref<number>(0)
const message = ref<string>('')
const isProcessing = ref<boolean>(false)
const result = ref<string | null>(null)
const error = ref<string | null>(null)

let unlisten: (() => void) | null = null

const handleUnfold = async () => {
  isProcessing.value = true
  progress.value = 0
  message.value = 'Запуск...'
  result.value = null
  error.value = null

  try {
    const bridge = getBridge()
    
    // Подписываемся на события прогресса
    unlisten = await bridge.onUnfoldProgress((percent, msg) => {
      progress.value = percent
      message.value = msg
    })

    // Запускаем тяжелую задачу (передаем 5000 граней для теста)
    const res = await bridge.startMockUnfold(5000)
    
    result.value = res
    message.value = 'Завершено!'
    progress.value = 100
  } catch (err: unknown) {
    // Tauri v2 часто возвращает ошибку как строку, а не как объект Error
    if (typeof err === 'string') {
      error.value = err
    } else if (err instanceof Error) {
      error.value = err.message
    } else {
      error.value = JSON.stringify(err)
    }
  } finally {
    isProcessing.value = false
    if (unlisten) {
      unlisten()
      unlisten = null
    }
  }
}

// Очистка при размонтировании компонента
onUnmounted(() => {
  if (unlisten) {
    unlisten()
  }
})
</script>

<template>
  <div class="p-6 max-w-md mx-auto">
    <h2 class="text-xl font-bold mb-4 text-gray-100">Асинхронный пайплайн вычислений (Unfold Skeleton)</h2>
    <p class="text-gray-400 mb-6 text-sm">
      Имитация тяжелых вычислений раскладки. Процесс выполняется в фоновом потоке и не блокирует UI.
    </p>

    <button 
      @click="handleUnfold" 
      :disabled="isProcessing" 
      class="bg-blue-600 hover:bg-blue-700 disabled:bg-gray-600 disabled:cursor-not-allowed text-white font-bold py-2 px-4 rounded transition-colors"
    >
      {{ isProcessing ? 'Вычисление...' : 'Start Mock Unfold' }}
    </button>

    <!-- Прогресс-бар -->
    <div v-if="isProcessing || progress > 0" class="mt-6">
      <div class="w-full bg-gray-700 rounded-full h-4 overflow-hidden">
        <div 
          class="bg-blue-500 h-full rounded-full transition-all duration-300 ease-linear"
          :style="{ width: progress + '%' }"
        ></div>
      </div>
      <p class="text-gray-400 text-sm mt-2">{{ message }}</p>
    </div>

    <!-- Успешный результат -->
    <div v-if="result" class="mt-4 p-4 bg-green-900/50 border border-green-500 rounded text-green-300 text-sm">
      <strong>Успех:</strong> {{ result }}
    </div>

    <!-- Ошибка -->
    <div v-if="error" class="mt-4 p-4 bg-red-900/50 border border-red-500 rounded text-red-300 text-sm">
      <strong>Error</strong><br>{{ error }}
    </div>

    <!-- Подсказка для тестирования -->
    <div v-if="!isProcessing && !result && !error" class="mt-6 p-3 bg-gray-800 rounded border border-gray-700 text-xs text-gray-500">
      Проверка: Пока идет процесс (около 5 секунд), попробуйте перетащить окно приложения за заголовок мышкой. Оно не должно быть замороженным.
    </div>
  </div>
</template>

<style scoped>
/* Стили реализованы через Tailwind утилиты прямо в шаблоне для минимизации кода */
</style>