import { defineStore } from 'pinia'
import { ref } from 'vue'

export const useAutoSaveStore = defineStore('autoSave', () => {
  const isEnabled = ref(true)
  const intervalSeconds = ref(30)
  const lastSaveAt = ref<Date | null>(null)

  let timer: ReturnType<typeof setInterval> | null = null

  const start = (save: () => Promise<void>) => {
    if (timer || !isEnabled.value) return
    timer = setInterval(async () => {
      try {
        await save()
        lastSaveAt.value = new Date()
      } catch (e) {
        console.error('[AutoSave] error', e)
      }
    }, intervalSeconds.value * 1000)
  }

  const stop = () => {
    if (!timer) return
    clearInterval(timer)
    timer = null
  }

  return { isEnabled, intervalSeconds, lastSaveAt, start, stop }
})