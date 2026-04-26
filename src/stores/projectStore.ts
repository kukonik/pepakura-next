import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'

export interface Vertex3D {
  x: number
  y: number
  z: number
}

export interface Face3D {
  a: number
  b: number
  c: number
}

export interface MeshData {
  name: string
  vertices: Vertex3D[]
  faces: Face3D[]
}

export interface UnfoldedResult {
  vertices_2d: number[]
  faces: any[]
  metadata: string
}

export interface ProjectState {
  modelPath: string | null
  modelData: string | null
  currentMesh: MeshData | null
  unfoldedResult: UnfoldedResult | null
  unfoldedSvg: string | null
  isProcessing: boolean
  error: string | null
}

export const useProjectStore = defineStore('project', () => {
  // Состояние проекта
  const modelPath = ref<string | null>(null)
  const modelData = ref<string | null>(null)
  const currentMesh = ref<MeshData | null>(null)
  const unfoldedResult = ref<UnfoldedResult | null>(null)
  const unfoldedSvg = ref<string | null>(null)
  const isProcessing = ref<boolean>(false)
  const error = ref<string | null>(null)

  // Вычисляемые свойства
  const hasModel = computed(() => modelPath.value !== null || currentMesh.value !== null)
  const hasUnfoldedSvg = computed(() => unfoldedSvg.value !== null)
  const hasUnfoldedResult = computed(() => unfoldedResult.value !== null)

  // Загрузка 3D модели
  const loadModel = (path: string, data: string) => {
    modelPath.value = path
    modelData.value = data
    unfoldedSvg.value = null
    unfoldedResult.value = null
    error.value = null
  }

  // Установка текущего меша (для Web версии)
  const setCurrentMesh = (mesh: MeshData) => {
    currentMesh.value = mesh
    error.value = null
  }

  // Установка результата развёртки
  const setUnfoldedResult = (result: UnfoldedResult) => {
    unfoldedResult.value = result
    isProcessing.value = false
    error.value = null
  }

  // Установка ошибки
  const setError = (message: string) => {
    error.value = message
    isProcessing.value = false
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
    currentMesh.value = null
    unfoldedResult.value = null
    unfoldedSvg.value = null
    isProcessing.value = false
    error.value = null
  }

  return {
    // Состояние
    modelPath,
    modelData,
    currentMesh,
    unfoldedResult,
    unfoldedSvg,
    isProcessing,
    error,

    // Вычисляемые свойства
    hasModel,
    hasUnfoldedSvg,
    hasUnfoldedResult,

    // Методы
    loadModel,
    setCurrentMesh,
    setUnfoldedResult,
    setError,
    unfoldModel,
    reset
  }
})