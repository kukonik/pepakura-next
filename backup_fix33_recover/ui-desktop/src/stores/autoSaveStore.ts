import { defineStore } from 'pinia'
import { ref, watch } from 'vue'
import { useProjectStore } from './project'

export const useAutoSaveStore = defineStore('autoSave', () => {
  const isEnabled = ref(true)
  const interval = ref(60000) // 1 минута
  const lastSave = ref<Date | null>(null)
  const versions = ref<Array<{ id: string; timestamp: number; description?: string }>>([])

  let autoSaveTimer: NodeJS.Timeout | null = null

  const projectStore = useProjectStore()

  // Сохранить проект в localStorage
  const saveProject = (): number => {
    if (!projectStore.project) {
      return 0
    }

    const timestamp = Date.now()
    const data = {
      timestamp,
      project: projectStore.project,
    }

    const key = `pepakura-project-${projectStore.project.id}`
    window.localStorage.setItem(key, JSON.stringify(data))

    // Добавить версию
    const version = {
      id: `version-${timestamp}`,
      timestamp,
      description: `Auto-save at ${new Date(timestamp).toISOString()}`,
    }
    const versionsKey = `pepakura-project-${projectStore.project.id}-versions`
    const existingVersions = JSON.parse(window.localStorage.getItem(versionsKey) || '[]')
    existingVersions.push(version)
    window.localStorage.setItem(versionsKey, JSON.stringify(existingVersions))

    lastSave.value = new Date(timestamp)
    return timestamp
  }

  // Загрузить проект из localStorage
  const loadProject = (projectId: string): boolean => {
    const key = `pepakura-project-${projectId}`
    const data = window.localStorage.getItem(key)
    if (!data) {
      return false
    }

    try {
      const parsed = JSON.parse(data)
      projectStore.project = parsed.project
      return true
    } catch {
      return false
    }
  }

  // Получить список сохранённых версий проекта
  const getSavedVersions = (projectId: string) => {
    const key = `pepakura-project-${projectId}-versions`
    const data = window.localStorage.getItem(key)
    if (!data) {
      return []
    }
    return JSON.parse(data)
  }

  // Включить/выключить автосохранение
  const setEnabled = (enabled: boolean) => {
    isEnabled.value = enabled
    if (enabled) {
      startAutoSave()
    } else {
      stopAutoSave()
    }
  }

  // Установить интервал автосохранения
  const setInterval = (ms: number) => {
    interval.value = ms
    if (isEnabled.value) {
      stopAutoSave()
      startAutoSave()
    }
  }

  const startAutoSave = () => {
    if (autoSaveTimer) {
      clearInterval(autoSaveTimer)
    }
    autoSaveTimer = setInterval(() => {
      saveProject()
    }, interval.value)
  }

  const stopAutoSave = () => {
    if (autoSaveTimer) {
      clearInterval(autoSaveTimer)
      autoSaveTimer = null
    }
  }

  // Инициализация
  if (isEnabled.value) {
    startAutoSave()
  }

  return {
    isEnabled,
    interval,
    lastSave,
    versions,
    saveProject,
    loadProject,
    getSavedVersions,
    setEnabled,
    setInterval,
  }
})