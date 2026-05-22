<template>
  <div class="sheet-preview-wrapper">
    <div v-if="isLoading" class="loading">Загрузка...</div>
    <div v-else-if="error" class="error">Ошибка: {{ error }}</div>
    <div v-else-if="svgContent" class="svg-container" v-html="svgContent"></div>
    <div v-else class="no-content">Нет данных для отображения</div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, watch } from 'vue'
import { useProjectStore } from '@/stores/projectStore'

const props = defineProps<{
  sheetIndex: number
}>()

const projectStore = useProjectStore()
const svgContent = ref<string | null>(null)
const isLoading = ref(false)
const error = ref<string | null>(null)

const loadSheetSvg = async () => {
  if (props.sheetIndex < 0) return
  
  isLoading.value = true
  error.value = null
  
  try {
    const svg = await projectStore.getSheetSvg(props.sheetIndex)
    svgContent.value = svg
  } catch (err: any) {
    error.value = err.message || 'Не удалось загрузить SVG'
    console.error('Failed to load sheet SVG:', err)
  } finally {
    isLoading.value = false
  }
}

// Загружаем SVG при монтировании и при изменении индекса листа
onMounted(() => {
  loadSheetSvg()
})

watch(() => props.sheetIndex, () => {
  loadSheetSvg()
})
</script>

<style scoped>
.sheet-preview-wrapper {
  width: 100%;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
}

.loading, .error, .no-content {
  padding: 20px;
  text-align: center;
}

.svg-container {
  width: 100%;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
}

.svg-container svg {
  max-width: 100%;
  max-height: 70vh;
  border: 1px solid var(--border-color, #334155);
  border-radius: 4px;
}
</style>