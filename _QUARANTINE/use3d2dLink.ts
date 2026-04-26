/**
 * Composable для привязки 3D ↔ 2D.
 * 
 * Предоставляет:
 * - Cross-highlighting (выделение в 3D → подсветка в 2D)
 * - Выделение в 2D → подсветка в 3D
 * - Синхронизацию камер
 * - Общие события
 */

import { ref, watch, onMounted, onUnmounted } from 'vue'

export interface FaceHighlight {
  faceIndex: number
  highlighted: boolean
  source: '3d' | '2d'
}

export interface CameraSync {
  view3d: {
    position: number[]
    target: number[]
    zoom: number
  }
  view2d: {
    x: number
    y: number
    scale: number
  }
}

export function use3d2dLink() {
  const linkedFaceIndex = ref<number | null>(null)
  const isSyncEnabled = ref(true)
  const highlightHistory = ref<FaceHighlight[]>([])

  // 3D → 2D: Выделение грани в 3D подсвечивает в 2D
  const highlightFaceIn2D = (faceIndex: number) => {
    linkedFaceIndex.value = faceIndex

    // Эмитим событие для 2D компонента
    window.dispatchEvent(new CustomEvent('highlight-face-2d', {
      detail: { faceIndex, highlighted: true },
    }))

    // Добавляем в историю
    highlightHistory.value.push({
      faceIndex,
      highlighted: true,
      source: '3d',
    })
  }

  // 2D → 3D: Выделение в 2D подсвечивает в 3D
  const highlightFaceIn3D = (faceIndex: number) => {
    linkedFaceIndex.value = faceIndex

    // Эмитим событие для 3D компонента
    window.dispatchEvent(new CustomEvent('highlight-face-3d', {
      detail: { faceIndex, highlighted: true },
    }))

    // Добавляем в историю
    highlightHistory.value.push({
      faceIndex,
      highlighted: true,
      source: '2d',
    })
  }

  // Сброс выделения
  const clearHighlight = () => {
    linkedFaceIndex.value = null

    // Эмитим события для сброса
    window.dispatchEvent(new CustomEvent('highlight-face-2d', {
      detail: { faceIndex: null, highlighted: false },
    }))

    window.dispatchEvent(new CustomEvent('highlight-face-3d', {
      detail: { faceIndex: null, highlighted: false },
    }))
  }

  // Синхронизация камеры 3D с 2D видом
  const syncCamera3dTo2d = (
    camera3d: { position: number[], target: number[] },
    camera2d: { x: number, y: number, scale: number }
  ) => {
    if (!isSyncEnabled.value) return

    // Проекция 3D камеры на 2D плоскость
    const projection = projectCameraTo2D(camera3d)

    // Эмитим событие для обновления 2D камеры
    window.dispatchEvent(new CustomEvent('sync-camera-2d', {
      detail: {
        x: projection.x,
        y: projection.y,
        scale: projection.scale,
      },
    }))
  }

  // Проекция 3D камеры на 2D
  const projectCameraTo2D = (camera3d: { position: number[], target: number[] }) => {
    const [x, y, z] = camera3d.position
    const [tx, ty, tz] = camera3d.target

    // Вычисляем угол обзора
    const dx = x - tx
    const dy = y - ty
    const dz = z - tz

    // Расстояние до цели
    const distance = Math.sqrt(dx * dx + dy * dy + dz * dz)

    // Масштаб обратно пропорционален расстоянию
    const scale = Math.max(0.1, Math.min(5, 10 / distance))

    // 2D позиция на основе направления
    const view2d = {
      x: dx * 0.5,
      y: dy * 0.5,
      scale,
    }

    return view2d
  }

  // Получение текущей выделенной грани
  const getLinkedFaceIndex = () => linkedFaceIndex.value

  // Включение/выключение синхронизации
  const setSyncEnabled = (enabled: boolean) => {
    isSyncEnabled.value = enabled
  }

  // Получение истории выделения
  const getHighlightHistory = () => highlightHistory.value

  // Очистка истории
  const clearHistory = () => {
    highlightHistory.value = []
  }

  // Обработчик событий от 3D
  const handle3dFaceSelected = (event: Event) => {
    const customEvent = event as CustomEvent<{ faceIndex: number }>
    highlightFaceIn2D(customEvent.detail.faceIndex)
  }

  // Обработчик событий от 2D
  const handle2dFaceSelected = (event: Event) => {
    const customEvent = event as CustomEvent<{ faceIndex: number }>
    highlightFaceIn3D(customEvent.detail.faceIndex)
  }

  // Подписка на события
  onMounted(() => {
    window.addEventListener('face-selected-3d', handle3dFaceSelected)
    window.addEventListener('face-selected-2d', handle2dFaceSelected)
  })

  // Отписка от событий
  onUnmounted(() => {
    window.removeEventListener('face-selected-3d', handle3dFaceSelected)
    window.removeEventListener('face-selected-2d', handle2dFaceSelected)
  })

  return {
    // State
    linkedFaceIndex,
    isSyncEnabled,
    highlightHistory,

    // Actions
    highlightFaceIn2D,
    highlightFaceIn3D,
    clearHighlight,
    syncCamera3dTo2d,
    projectCameraTo2D,
    getLinkedFaceIndex,
    setSyncEnabled,
    getHighlightHistory,
    clearHistory,
  }
}
