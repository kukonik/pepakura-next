/**
 * Composable для работы с персистентностью и автосохранением.
 * 
 * Предоставляет:
 * - Автосохранение состояния проекта
 * - Загрузку сохранённых данных
 * - Восстановление после краша
 * - Историю действий (undo/redo)
 */

import { ref, computed, watch, type Ref } from 'vue'
import { listen } from '@tauri-apps/api/event'
import { usePlatform } from '@pepakura/shared/composables/usePlatform'

export interface AppSettings {
  theme: string
  language: string
  last_project_path: string | null
  auto_save_interval: number
  auto_save_enabled: boolean
}

export interface HistoryEntry {
  id: number
  project_id: string
  action: string
  state_before: any
  state_after: any
  timestamp: string
}

export interface RecentProject {
  path: string
  name: string
  last_opened: string
}

export interface StateEntry {
  key: string
  value: any
  updated_at: string
}

export function usePersistence() {
  const isSaving = ref(false)
  const isLoaded = ref(false)
  const lastSaveAt = ref<Date | null>(null)
  const error = ref<string | null>(null)
  const settings = ref<AppSettings>({
    theme: 'system',
    language: 'ru',
    last_project_path: null,
    auto_save_interval: 30,
    auto_save_enabled: true,
  })
  const recentProjects = ref<RecentProject[]>([])
  const history = ref<HistoryEntry[]>([])

  const { invoke } = usePlatform()

  // Загрузка настроек
  const loadSettings = async () => {
    try {
      const loaded = await invoke<AppSettings>('get_all_settings')
      settings.value = { ...settings.value, ...loaded }
      return loaded
    } catch (e) {
      error.value = e instanceof Error ? e.message : String(e)
      console.error('Failed to load settings:', error.value)
      return null
    }
  }

  // Сохранение настройки
  const saveSetting = async (key: string, value: string) => {
    try {
      await invoke('save_setting', { key, value })
      return true
    } catch (e) {
      error.value = e instanceof Error ? e.message : String(e)
      return false
    }
  }

  // Загрузка состояния по ключу
  const loadState = async <T>(key: string): Promise<T | null> => {
    try {
      const result = await invoke<T | null>('load_app_state', { key })
      return result
    } catch (e) {
      error.value = e instanceof Error ? e.message : String(e)
      return null
    }
  }

  // Сохранение состояния по ключу
  const saveState = async (key: string, value: any) => {
    isSaving.value = true
    try {
      await invoke('save_app_state', { key, value })
      lastSaveAt.value = new Date()
      return true
    } catch (e) {
      error.value = e instanceof Error ? e.message : String(e)
      return false
    } finally {
      isSaving.value = false
    }
  }

  // Добавление проекта в последние
  const addRecentProject = async (path: string, name: string) => {
    try {
      await invoke('add_recent_project', { path, name })
      await loadRecentProjects()
      return true
    } catch (e) {
      error.value = e instanceof Error ? e.message : String(e)
      return false
    }
  }

  // Загрузка последних проектов
  const loadRecentProjects = async () => {
    try {
      const projects = await invoke<RecentProject[]>('get_recent_projects')
      recentProjects.value = projects
      return projects
    } catch (e) {
      error.value = e instanceof Error ? e.message : String(e)
      return []
    }
  }

  // Добавление действия в историю
  const pushHistory = async (
    projectId: string,
    action: string,
    stateBefore: any,
    stateAfter: any
  ) => {
    try {
      const id = await invoke<number>('push_history', {
        projectId,
        action,
        stateBefore,
        stateAfter,
      })
      return id
    } catch (e) {
      error.value = e instanceof Error ? e.message : String(e)
      return null
    }
  }

  // Загрузка истории
  const loadHistory = async (projectId: string, limit = 50) => {
    try {
      const entries = await invoke<HistoryEntry[]>('get_history', {
        projectId,
        limit,
      })
      history.value = entries
      return entries
    } catch (e) {
      error.value = e instanceof Error ? e.message : String(e)
      return []
    }
  }

  // Получить последнее действие для undo
  const getLastUndo = async (projectId: string): Promise<HistoryEntry | null> => {
    try {
      const entry = await invoke<HistoryEntry | null>('get_last_undo', { projectId })
      return entry
    } catch (e) {
      error.value = e instanceof Error ? e.message : String(e)
      return null
    }
  }

  // Отмена последнего действия
  const undo = async (projectId: string): Promise<boolean> => {
    const entry = await getLastUndo(projectId)
    if (!entry) return false

    // Восстанавливаем предыдущее состояние
    await saveState(`project:${projectId}`, entry.state_before)
    return true
  }

  // Восстановление после краша
  const recoverFromCrash = async (): Promise<StateEntry[]> => {
    try {
      const entries = await invoke<StateEntry[]>('recover_from_crash')
      return entries
    } catch (e) {
      error.value = e instanceof Error ? e.message : String(e)
      return []
    }
  }

  // Настройка автосохранения
  const setupAutoSave = (
    getState: () => any,
    projectId: Ref<string | null>
  ) => {
    // Сохранение при изменении состояния
    const save = async () => {
      if (!projectId.value || !settings.value.auto_save_enabled) return
      
      const state = getState()
      await saveState(`project:${projectId.value}`, state)
      
      // Добавляем в историю
      await pushHistory(projectId.value, 'auto_save', {}, state)
    }

    // Автосохранение по таймеру
    let timer: ReturnType<typeof setInterval> | null = null
    
    const startAutoSave = () => {
      if (timer) return
      
      timer = setInterval(() => {
        save()
      }, settings.value.auto_save_interval * 1000)
    }

    const stopAutoSave = () => {
      if (timer) {
        clearInterval(timer)
        timer = null
      }
    }

    // Сохранение перед закрытием
    const handleBeforeUnload = () => {
      save()
    }

    if (typeof window !== 'undefined') {
      window.addEventListener('beforeunload', handleBeforeUnload)
    }

    return {
      startAutoSave,
      stopAutoSave,
      save,
    }
  }

  // Инициализация
  const init = async () => {
    await loadSettings()
    await loadRecentProjects()
    isLoaded.value = true
  }

  // Проверка наличия данных для восстановления
  const hasRecoveryData = async (): Promise<boolean> => {
    const entries = await recoverFromCrash()
    return entries.length > 0
  }

  return {
    // State
    isSaving,
    isLoaded,
    lastSaveAt,
    error,
    settings,
    recentProjects,
    history,

    // Settings
    loadSettings,
    saveSetting,

    // State persistence
    loadState,
    saveState,

    // Recent projects
    addRecentProject,
    loadRecentProjects,

    // History & Undo
    pushHistory,
    loadHistory,
    getLastUndo,
    undo,

    // Recovery
    recoverFromCrash,
    hasRecoveryData,

    // Auto-save
    setupAutoSave,

    // Init
    init,
  }
}