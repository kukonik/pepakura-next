import { createPinia, setActivePinia } from 'pinia'
import { useAutoSaveStore } from './autoSaveStore'
import { useProjectStore } from './project'
import { vi, beforeEach, describe, it, expect } from 'vitest'

// Мокаем localStorage
const localStorageMock = (() => {
  let store: Record<string, string> = {}
  return {
    getItem: (key: string) => store[key] || null,
    setItem: (key: string, value: string) => {
      store[key] = value.toString()
    },
    removeItem: (key: string) => {
      delete store[key]
    },
    clear: () => {
      store = {}
    }
  }
})()

Object.defineProperty(window, 'localStorage', {
  value: localStorageMock
})

describe('autoSaveStore', () => {
  beforeEach(() => {
    // Создаем новую инстанцию Pinia перед каждым тестом
    setActivePinia(createPinia())
    
    // Очищаем localStorage
    window.localStorage.clear()
  })

  describe('saveProject', () => {
    it('should save project state to localStorage', () => {
      const autoSaveStore = useAutoSaveStore()
      const projectStore = useProjectStore()
      
      // Устанавливаем тестовые данные проекта
      projectStore.project = {
        id: 'test-project-id',
        name: 'Test Project',
        createdAt: new Date().toISOString(),
        lastModified: new Date().toISOString(),
        settings: {
          aiBackend: 'ollama',
          aiModel: 'llama2',
          exportFormat: 'svg'
        },
        model: {
          status: 'ready',
          data: null,
          previews: []
        },
        unfold: {
          status: 'idle',
          data: null,
          estimatedPatches: null,
          estimatedSheets: null
        }
      }
      
      // Сохраняем проект
      autoSaveStore.saveProject()
      
      // Проверяем, что данные сохранены в localStorage
      const savedData = window.localStorage.getItem('pepakura-project-test-project-id')
      expect(savedData).not.toBeNull()
      
      const parsedData = JSON.parse(savedData!)
      expect(parsedData.project.id).toBe('test-project-id')
      expect(parsedData.project.name).toBe('Test Project')
    })

    it('should save project with timestamp', () => {
      const autoSaveStore = useAutoSaveStore()
      const projectStore = useProjectStore()
      
      // Устанавливаем тестовые данные проекта
      projectStore.project = {
        id: 'test-project-id',
        name: 'Test Project',
        createdAt: new Date().toISOString(),
        lastModified: new Date().toISOString(),
        settings: {
          aiBackend: 'ollama',
          aiModel: 'llama2',
          exportFormat: 'svg'
        },
        model: {
          status: 'ready',
          data: null,
          previews: []
        },
        unfold: {
          status: 'idle',
          data: null,
          estimatedPatches: null,
          estimatedSheets: null
        }
      }
      
      // Сохраняем проект
      const timestamp = autoSaveStore.saveProject()
      
      // Проверяем, что возвращен timestamp
      expect(timestamp).toBeGreaterThan(0)
      
      // Проверяем, что данные сохранены в localStorage
      const savedData = window.localStorage.getItem('pepakura-project-test-project-id')
      expect(savedData).not.toBeNull()
      
      const parsedData = JSON.parse(savedData!)
      expect(parsedData.timestamp).toBe(timestamp)
    })
  })

  describe('loadProject', () => {
    it('should load project state from localStorage', () => {
      const autoSaveStore = useAutoSaveStore()
      const projectStore = useProjectStore()
      
      // Создаем тестовые данные проекта
      const testData = {
        timestamp: Date.now(),
        project: {
          id: 'test-project-id',
          name: 'Test Project',
          createdAt: new Date().toISOString(),
          lastModified: new Date().toISOString(),
          settings: {
            aiBackend: 'ollama',
            aiModel: 'llama2',
            exportFormat: 'svg'
          },
          model: {
            status: 'ready',
            data: null,
            previews: []
          },
          unfold: {
            status: 'idle',
            data: null,
            estimatedPatches: null,
            estimatedSheets: null
          }
        }
      }
      
      // Сохраняем тестовые данные в localStorage
      window.localStorage.setItem('pepakura-project-test-project-id', JSON.stringify(testData))
      
      // Загружаем проект
      const loaded = autoSaveStore.loadProject('test-project-id')
      
      // Проверяем, что проект загружен успешно
      expect(loaded).toBe(true)
      expect(projectStore.project.id).toBe('test-project-id')
      expect(projectStore.project.name).toBe('Test Project')
    })

    it('should return false when project not found', () => {
      const autoSaveStore = useAutoSaveStore()
      
      // Пытаемся загрузить несуществующий проект
      const loaded = autoSaveStore.loadProject('non-existent-project-id')
      
      // Проверяем, что загрузка не удалась
      expect(loaded).toBe(false)
    })
  })

  describe('getSavedVersions', () => {
    it('should return list of saved versions', () => {
      const autoSaveStore = useAutoSaveStore()
      
      // Создаем тестовые данные версий
      const testVersions = [
        { id: 'version-1', timestamp: Date.now() - 1000, description: 'First version' },
        { id: 'version-2', timestamp: Date.now(), description: 'Second version' }
      ]
      
      // Сохраняем тестовые данные в localStorage
      window.localStorage.setItem(
        'pepakura-project-test-project-id-versions', 
        JSON.stringify(testVersions)
      )
      
      // Получаем список версий
      const versions = autoSaveStore.getSavedVersions('test-project-id')
      
      // Проверяем, что версии загружены правильно
      expect(versions).toHaveLength(2)
      expect(versions[0].id).toBe('version-1')
      expect(versions[1].id).toBe('version-2')
    })

    it('should return empty array when no versions found', () => {
      const autoSaveStore = useAutoSaveStore()
      
      // Получаем список версий для проекта без сохраненных версий
      const versions = autoSaveStore.getSavedVersions('non-existent-project-id')
      
      // Проверяем, что возвращен пустой массив
      expect(versions).toHaveLength(0)
    })
  })

  describe('autoSave', () => {
    it('should automatically save project when enabled', async () => {
      const autoSaveStore = useAutoSaveStore()
      const projectStore = useProjectStore()
      
      // Устанавливаем тестовые данные проекта
      projectStore.project = {
        id: 'test-project-id',
        name: 'Test Project',
        createdAt: new Date().toISOString(),
        lastModified: new Date().toISOString(),
        settings: {
          aiBackend: 'ollama',
          aiModel: 'llama2',
          exportFormat: 'svg'
        },
        model: {
          status: 'ready',
          data: null,
          previews: []
        },
        unfold: {
          status: 'idle',
          data: null,
          estimatedPatches: null,
          estimatedSheets: null
        }
      }
      
      // Включаем автосохранение
      autoSaveStore.setEnabled(true)
      autoSaveStore.setInterval(100) // Устанавливаем короткий интервал для теста
      
      // Ждем немного больше интервала
      await new Promise(resolve => setTimeout(resolve, 150))
      
      // Проверяем, что проект был сохранен
      const savedData = window.localStorage.getItem('pepakura-project-test-project-id')
      expect(savedData).not.toBeNull()
    })

    it('should not save project when disabled', async () => {
      const autoSaveStore = useAutoSaveStore()
      const projectStore = useProjectStore()
      
      // Устанавливаем тестовые данные проекта
      projectStore.project = {
        id: 'test-project-id',
        name: 'Test Project',
        createdAt: new Date().toISOString(),
        lastModified: new Date().toISOString(),
        settings: {
          aiBackend: 'ollama',
          aiModel: 'llama2',
          exportFormat: 'svg'
        },
        model: {
          status: 'ready',
          data: null,
          previews: []
        },
        unfold: {
          status: 'idle',
          data: null,
          estimatedPatches: null,
          estimatedSheets: null
        }
      }
      
      // Отключаем автосохранение
      autoSaveStore.setEnabled(false)
      autoSaveStore.setInterval(100) // Устанавливаем короткий интервал для теста
      
      // Ждем немного больше интервала
      await new Promise(resolve => setTimeout(resolve, 150))
      
      // Проверяем, что проект не был сохранен
      const savedData = window.localStorage.getItem('pepakura-project-test-project-id')
      expect(savedData).toBeNull()
    })
  })
})