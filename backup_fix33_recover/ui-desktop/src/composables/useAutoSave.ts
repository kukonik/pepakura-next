import { ref, watch, onMounted, onUnmounted } from 'vue'
import { useProjectStore } from '@/stores/project.store'
import { GeneratedModel } from '@/types/project'

// Типы для автосохранения
interface AutoSaveState {
  isEnabled: boolean
  interval: number  // в минутах
  lastSaveTime: Date | null
  isSaving: boolean
  error: string | null
}

interface AutoSaveOptions {
  interval?: number  // в минутах
  onSave?: () => Promise<void>
}

// Глобальное состояние автосохранения
const autoSaveState = ref<AutoSaveState>({
  isEnabled: true,
  interval: 5,  // 5 минут по умолчанию
  lastSaveTime: null,
  isSaving: false,
  error: null
})

// Таймер автосохранения
let autoSaveTimer: number | null = null

// Функция для запуска автосохранения
const startAutoSave = (options?: AutoSaveOptions) => {
  // Останавливаем предыдущий таймер, если он есть
  if (autoSaveTimer) {
    clearInterval(autoSaveTimer)
    autoSaveTimer = null
  }
  
  // Обновляем настройки
  if (options?.interval) {
    autoSaveState.value.interval = options.interval
  }
  
  // Запускаем таймер
  autoSaveState.value.isEnabled = true
  autoSaveTimer = window.setInterval(() => {
    triggerAutoSave(options?.onSave)
  }, autoSaveState.value.interval * 60 * 1000)
}

// Функция для остановки автосохранения
const stopAutoSave = () => {
  if (autoSaveTimer) {
    clearInterval(autoSaveTimer)
    autoSaveTimer = null
  }
  autoSaveState.value.isEnabled = false
}

// Функция для выполнения автосохранения
const triggerAutoSave = async (onSave?: () => Promise<void>) => {
  if (autoSaveState.value.isSaving) {
    return
  }
  
  try {
    autoSaveState.value.isSaving = true
    autoSaveState.value.error = null
    
    // Выполняем пользовательскую функцию сохранения, если она предоставлена
    if (onSave) {
      await onSave()
    }
    
    // Обновляем время последнего сохранения
    autoSaveState.value.lastSaveTime = new Date()
  } catch (error) {
    autoSaveState.value.error = error instanceof Error ? error.message : 'Неизвестная ошибка при автосохранении'
    console.error('Ошибка автосохранения:', error)
  } finally {
    autoSaveState.value.isSaving = false
  }
}

// Функция для немедленного сохранения
const saveNow = async (onSave?: () => Promise<void>) => {
  await triggerAutoSave(onSave)
}

// Инициализация автосохранения при монтировании
onMounted(() => {
  // Автосохранение запускается при необходимости в компонентах
})

// Очистка при размонтировании
onUnmounted(() => {
  if (autoSaveTimer) {
    clearInterval(autoSaveTimer)
  }
})

// Экспортируем функции и состояние
export const useAutoSave = (options?: AutoSaveOptions) => {
  const projectStore = useProjectStore()
  
  // Следим за изменениями настроек автосохранения в хранилище
  watch(
    () => projectStore.autoSaveEnabled,
    (isEnabled) => {
      if (isEnabled) {
        startAutoSave(options)
      } else {
        stopAutoSave()
      }
    },
    { immediate: true }
  )
  
  watch(
    () => projectStore.autoSaveInterval,
    (interval) => {
      if (projectStore.autoSaveEnabled) {
        stopAutoSave()
        startAutoSave({ ...options, interval })
      }
    }
  )
  
  return {
    // Состояние
    autoSaveState: autoSaveState,
    
    // Методы управления
    startAutoSave,
    stopAutoSave,
    saveNow,
    
    // Управление настройками
    setAutoSaveEnabled: projectStore.setAutoSaveEnabled,
    setAutoSaveInterval: projectStore.setAutoSaveInterval,
  }
}

// Экспортируем типы
export type { AutoSaveState, AutoSaveOptions, GeneratedModel }