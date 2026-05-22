/**
 * Composable для связи 2D и 3D видов.
 * 
 * Синхронизирует выделение между 2D развёрткой и 3D моделью.
 */

import { ref, watch } from 'vue'

export interface MeshFace {
  vertices: [number, number, number]
}

export interface UnfoldedFace {
  vertices_2d: Array<[number, number]>
  faceIndex: number
}

export function useViewLinking() {
  // Выбранные грани
  const selectedFace2D = ref<number | null>(null)
  const selectedFace3D = ref<number | null>(null)
  
  // Hover состояния
  const hoveredFace2D = ref<number | null>(null)
  const hoveredFace3D = ref<number | null>(null)

  /**
   * Выделяет грань в 2D виде.
   */
  function selectFace2D(faceIndex: number | null) {
    selectedFace2D.value = faceIndex
    if (faceIndex !== null) {
      selectedFace3D.value = faceIndex
    }
  }

  /**
   * Выделяет грань в 3D виде.
   */
  function selectFace3D(faceIndex: number | null) {
    selectedFace3D.value = faceIndex
    if (faceIndex !== null) {
      selectedFace2D.value = faceIndex
    }
  }

  /**
   * Наводит на грань в 2D виде.
   */
  function hoverFace2D(faceIndex: number | null) {
    hoveredFace2D.value = faceIndex
    if (faceIndex !== null) {
      hoveredFace3D.value = faceIndex
    }
  }

  /**
   * Наводит на грань в 3D виде.
   */
  function hoverFace3D(faceIndex: number | null) {
    hoveredFace3D.value = faceIndex
    if (faceIndex !== null) {
      hoveredFace2D.value = faceIndex
    }
  }

  /**
   * Сбрасывает выделение.
   */
  function clearSelection() {
    selectedFace2D.value = null
    selectedFace3D.value = null
    hoveredFace2D.value = null
    hoveredFace3D.value = null
  }

  /**
   * Проверяет, выделена ли грань.
   */
  function isFaceSelected(faceIndex: number): boolean {
    return selectedFace2D.value === faceIndex || selectedFace3D.value === faceIndex
  }

  /**
   * Проверяет, наведена ли грань.
   */
  function isFaceHovered(faceIndex: number): boolean {
    return hoveredFace2D.value === faceIndex || hoveredFace3D.value === faceIndex
  }

  // Синхронизация между 2D и 3D
  watch(selectedFace2D, (newVal) => {
    if (newVal !== null) {
      selectedFace3D.value = newVal
    }
  })

  watch(selectedFace3D, (newVal) => {
    if (newVal !== null) {
      selectedFace2D.value = newVal
    }
  })

  return {
    // State
    selectedFace2D,
    selectedFace3D,
    hoveredFace2D,
    hoveredFace3D,
    
    // Actions
    selectFace2D,
    selectFace3D,
    hoverFace2D,
    hoverFace3D,
    clearSelection,
    isFaceSelected,
    isFaceHovered,
  }
}
