import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { usePersistence } from '@/composables/usePersistence'

export const useAutoSaveStore = defineStore('autoSave', () => {
  const { saveState, loadState, pushHistory, settings } = usePersistence()
  
  const isEnabled = ref(settings.value.auto_save_enabled)
  const intervalSeconds = ref(settings.value.auto_save_interval)
  const lastSaveAt = ref<Date | null>(null)
  const nextSaveAt = ref<Date | null>(null)
  const saveCount = ref(0)
  const failedSaves = ref(0)
  const isSaving = ref(false)
  
  let timer: ReturnType<typeof setInterval> | null = null

  // Адаптивный интервал (уменьшается при частых изменениях)
  const adaptiveInterval = computed(() => {
    if (saveCount.value < 5) return 15000 // 15 сек в начале
    if (failedSaves.value > 0) return 60000 // 1 мин при ошибках
    return intervalSeconds.value * 1000
  })
  
  const start = (getState: () => any, projectId: string | null) => {
    if (timer || !isEnabled.value || !projectId) return
    
    const tick = async () => {
      if (isSaving.value) return
      
      isSaving.value = true
      try {
        const state = getState()
        await saveState(`project:${projectId}`, state)
        
        await pushHistory(projectId, 'auto_save', {}, state)
        
        lastSaveAt.value = new Date()
        nextSaveAt.value = new Date(Date.now() + adaptiveInterval.value)
        saveCount.value++
        failedSaves.value = 0
      } catch (e) {
        console.error('[AutoSave] error', e)
        failedSaves.value++
      } finally {
        isSaving.value = false
      }
    }
    
    // Первое сохранение через 5 секунд
    setTimeout(() => {
      tick()
      timer = setInterval(tick, adaptiveInterval.value)
    }, 5000)
  }
  
  const stop = () => {
    if (!timer) return
    clearInterval(timer)
    timer = null
  }
  
  const forceSave = async (getState: () => any, projectId: string | null) => {
    if (isSaving.value || !projectId) return
    isSaving.value = true
    try {
      const state = getState()
      await saveState(`project:${projectId}`, state)
      lastSaveAt.value = new Date()
      saveCount.value++
    } catch (e) {
      console.error('[AutoSave] force save error', e)
      failedSaves.value++
    } finally {
      isSaving.value = false
    }
  }
  
  const updateSettings = (newSettings: { enabled?: boolean, interval?: number }) => {
    if (newSettings.enabled !== undefined) {
      isEnabled.value = newSettings.enabled
    }
    if (newSettings.interval !== undefined) {
      intervalSeconds.value = newSettings.interval
    }
  }

  return {
    isEnabled,
    intervalSeconds,
    lastSaveAt,
    nextSaveAt,
    saveCount,
    failedSaves,
    isSaving,
    start,
    stop,
    forceSave,
    updateSettings
  }
})