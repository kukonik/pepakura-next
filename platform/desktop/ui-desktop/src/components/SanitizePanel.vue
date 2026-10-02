<script setup lang="ts">
import { ref, computed } from 'vue'
import { usePlatform } from '@pepakura/shared/composables/usePlatform'
import type { SanitizeReport } from '@pepakura/shared/types/core'

const { getBridge } = usePlatform()

// State
const targetFaces = ref<number>(5000)
const report = ref<SanitizeReport | null>(null)
const isProcessing = ref(false)
const svgContent = ref<string | null>(null)

// Вычисления
const reductionPercent = computed(() => {
  if (!report.value) return 0
  const delta = report.value.original_faces - report.value.final_faces
  return Math.round((delta / report.value.original_faces) * 100)
})

// Actions
const handleOptimize = async () => {
  isProcessing.value = true
  report.value = null
  
  try {
    // Вызываем команду упрощения меша
    // Предполагаем, что есть команда 'sanitize_mesh' с параметром target_faces
    const result = await getBridge().invokeWithResult<SanitizeReport>('sanitize_mesh', {
      target_faces: targetFaces.value,
    })
    report.value = result
  } catch (e) {
    console.error('Ошибка оптимизации:', e)
    // В реальном приложении показать уведомление
  } finally {
    isProcessing.value = false
  }
}

const handleDownloadSvg = async () => {
  try {
    // Получаем SVG строку из Rust
    const svg = await getBridge().invokeWithResult<string>('export_svg', {
      // Можно передать опции, если нужно
    })
    
    // Создаем Blob
    const blob = new Blob([svg], { type: 'image/svg+xml' })
    const url = URL.createObjectURL(blob)
    
    // Триггерим скачивание
    const a = document.createElement('a')
    a.href = url
    a.download = 'unfold-model.svg'
    a.click()
    
    URL.revokeObjectURL(url)
  } catch (e) {
    console.error('Ошибка экспорта SVG:', e)
  }
}
</script>

<template>
  <div class="sanitize-panel border rounded p-4 bg-gray-900 text-gray-200">
    <h3 class="text-lg font-bold mb-4">🧹 Оптимизация & Экспорт</h3>

    <!-- Controls -->
    <div class="mb-4">
      <label class="block text-sm mb-2">Детализация (Граней): {{ targetFaces }}</label>
      <input 
        type="range" 
        v-model.number="targetFaces" 
        min="500" 
        max="20000" 
        step="100"
        class="w-full h-2 bg-gray-700 rounded-lg appearance-none cursor-pointer"
        :disabled="isProcessing"
      />
      <div class="flex justify-between text-xs text-gray-400 mt-1">
        <span>500</span>
        <span>10k</span>
        <span>20k</span>
      </div>
    </div>

    <div class="flex gap-2 mb-4">
      <button 
        @click="handleOptimize" 
        :disabled="isProcessing"
        class="flex-1 bg-blue-600 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded disabled:opacity-50 transition-colors"
      >
        {{ isProcessing ? 'Обработка...' : 'Оптимизировать' }}
      </button>
    </div>

    <!-- Stats -->
    <div v-if="report" class="bg-gray-800 p-3 rounded mb-4 text-sm border-l-4 border-green-500">
      <p class="mb-1">Граней: {{ report.original_faces }} → <strong>{{ report.final_faces }}</strong></p>
      <p class="text-green-400 font-semibold">Сокращено на {{ reductionPercent }}%</p>
      <p class="text-gray-400 text-xs mt-1">Время: {{ report.time_ms }} мс</p>
    </div>

    <!-- Download -->
    <button 
      @click="handleDownloadSvg"
      class="w-full border border-green-600 text-green-500 hover:bg-green-900/20 font-bold py-2 px-4 rounded flex items-center justify-center gap-2 transition-colors"
      :disabled="isProcessing"
    >
      <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
        <path fill-rule="evenodd" d="M3 17a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zm3.293-7.707a1 1 0 011.414 0L9 10.586V3a1 1 0 112 0v7.586l1.293-1.293a1 1 0 111.414 1.414l-3 3a1 1 0 01-1.414 0l-3-3a1 1 0 010-1.414z" clip-rule="evenodd" />
      </svg>
      📥 Скачать SVG
    </button>

    <p class="text-gray-500 text-xs mt-4 text-center">
      Автоматически упрощает модель до выбранного количества граней и экспортирует в векторный формат.
    </p>
  </div>
</template>

<style scoped>
.sanitize-panel {
  border-color: #374151;
}

input[type="range"]::-webkit-slider-thumb {
  appearance: none;
  height: 18px;
  width: 18px;
  border-radius: 50%;
  background: #3b82f6;
  cursor: pointer;
}

input[type="range"]::-moz-range-thumb {
  height: 18px;
  width: 18px;
  border-radius: 50%;
  background: #3b82f6;
  cursor: pointer;
  border: none;
}
</style>