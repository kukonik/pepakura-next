import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'

export interface ProjectState {
  modelPath: string | null
  modelData: string | null
  unfoldedSvg: string | null
  isProcessing: boolean
  error: string | null
}

export const useProjectStore = defineStore('project', () => {
  // Состояние проекта
  const modelPath = ref<string | null>(null)
  const modelData = ref<string | null>(null)
  const unfoldedSvg = ref<string | null>(null)
  const isProcessing = ref<boolean>(false)
  const error = ref<string | null>(null)

  // Вычисляемые свойства
  const hasModel = computed(() => modelPath.value !== null)
  const hasUnfoldedSvg = computed(() => unfoldedSvg.value !== null)

  // Загрузка 3D модели
  const loadModel = (path: string, data: string) => {
    modelPath.value = path
    modelData.value = data
    unfoldedSvg.value = null
    error.value = null
  }

  // Выполнение развёртки 3D модели
  const unfoldModel = async () => {
    if (!modelPath.value) {
      error.value = 'Нет загруженной 3D модели'
      return
    }

    isProcessing.value = true
    error.value = null

    try {
      // Вызываем команду Rust для развёртки
      const svgContent = await invoke<string>('unfold_3d_model', {
        objPath: modelPath.value
      })
      
      unfoldedSvg.value = svgContent
    } catch (err) {
      error.value = err instanceof Error ? err.message : String(err)
      console.error('Ошибка при выполнении развёртки:', err)
    } finally {
      isProcessing.value = false
    }
  }

  // Очистка состояния
  const reset = () => {
    modelPath.value = null
    modelData.value = null
    unfoldedSvg.value = null
    isProcessing.value = false
    error.value = null
  }

  return {
    // Состояние
    modelPath,
    modelData,
    unfoldedSvg,
    isProcessing,
    error,
    
    // Вычисляемые свойства
    hasModel,
    hasUnfoldedSvg,
    
    // Методы
    loadModel,
    unfoldModel,
    reset
  }
})