import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { Project, ProjectStats, GeneratedModel, GenerationSource } from '@/types/project'

interface State {
  projects: Project[]
  loading: boolean
  error: string | null
  stats: ProjectStats
  autoSaveEnabled: boolean
  autoSaveInterval: number  // в минутах
}

export const useProjectStore = defineStore('project', () => {
  // Состояние
  const state = ref<State>({
    projects: [],
    loading: false,
    error: null,
    stats: {
      total: 0,
      totalSize: 0,
      recentCount: 0,
      lastActive: null,
      avgSheetCount: 0,
      generatedModelsCount: 0
    },
    autoSaveEnabled: true,
    autoSaveInterval: 5  // 5 минут по умолчанию
  })

  // Вычисляемые свойства
  const projects = computed(() => state.value.projects)
  const loading = computed(() => state.value.loading)
  const error = computed(() => state.value.error)
  const stats = computed(() => state.value.stats)
  const autoSaveEnabled = computed(() => state.value.autoSaveEnabled)
  const autoSaveInterval = computed(() => state.value.autoSaveInterval)
  
  const recentProjects = computed(() => 
    [...state.value.projects]
      .sort((a, b) => b.lastModified.getTime() - a.lastModified.getTime())
      .slice(0, 10)
  )

  // Методы
  const loadProjects = async () => {
    try {
      state.value.loading = true
      state.value.error = null
      
      // Имитация загрузки данных (в реальном приложении здесь будут вызовы Tauri)
      await new Promise(resolve => setTimeout(resolve, 800))
      
      // Mock данные для демонстрации
      const mockProjects: Project[] = [
        {
          id: 'proj-001',
          name: 'Рыцарь',
          thumbnail: '/thumbnails/knight.jpg',
          lastModified: new Date('2026-01-15T14:30:00'),
          fileSize: 2.5,
          tags: ['фигурка', 'персонаж'],
          filePath: 'D:/models/knight.obj',
          sheetCount: 3,
          modelType: 'человек',
          generatedModels: [],
        },
        {
          id: 'proj-002',
          name: 'Танк',
          thumbnail: '/thumbnails/tank.jpg',
          lastModified: new Date('2026-01-14T09:15:00'),
          fileSize: 4.8,
          tags: ['техника', 'военная'],
          filePath: 'D:/models/tank.obj',
          sheetCount: 5,
          modelType: 'техника',
          generatedModels: [],
        },
        {
          id: 'proj-003',
          name: 'Дракон',
          thumbnail: '/thumbnails/dragon.jpg',
          lastModified: new Date('2026-01-12T16:45:00'),
          fileSize: 6.2,
          tags: ['фэнтези', 'животное'],
          filePath: 'D:/models/dragon.obj',
          sheetCount: 8,
          modelType: 'фэнтези',
          generatedModels: [],
        },
        {
          id: 'proj-004',
          name: 'Космический корабль',
          thumbnail: '/thumbnails/spaceship.jpg',
          lastModified: new Date('2026-01-10T11:20:00'),
          fileSize: 3.7,
          tags: ['космос', 'техника'],
          filePath: 'D:/models/spaceship.obj',
          sheetCount: 6,
          modelType: 'техника',
          generatedModels: [],
        }
      ]
      
      state.value.projects = mockProjects
      
      // Обновление статистики
      updateStats()
      
    } catch (err) {
      state.value.error = 'Ошибка загрузки проектов: ' + (err instanceof Error ? err.message : String(err))
      console.error('Error loading projects:', err)
    } finally {
      state.value.loading = false
    }
  }

  const createProject = async (name: string, file?: File): Promise<Project | null> => {
    try {
      state.value.loading = true
      state.value.error = null
      
      // Генерация уникального ID
      const id = `proj-${Date.now().toString(36)}`
      const now = new Date()
      
      // Создание нового проекта
      const newProject: Project = {
        id,
        name,
        lastModified: now,
        fileSize: file ? file.size / (1024 * 1024) : 0.5, // в МБ
        tags: ['новый'],
        sheetCount: 1,
        modelType: 'неизвестно',
        generatedModels: []
      }
      
      // Добавление в состояние
      state.value.projects.unshift(newProject)
      
      // Обновление статистики
      updateStats()
      
      return newProject
    } catch (err) {
      state.value.error = 'Ошибка создания проекта: ' + (err instanceof Error ? err.message : String(err))
      console.error('Error creating project:', err)
      return null
    } finally {
      state.value.loading = false
    }
  }

  const deleteProject = async (projectId: string): Promise<boolean> => {
    try {
      state.value.loading = true
      state.value.error = null
      
      // Фильтрация проектов
      const initialCount = state.value.projects.length
      state.value.projects = state.value.projects.filter(p => p.id !== projectId)
      
      // Проверка успешности удаления
      if (state.value.projects.length === initialCount) {
        throw new Error('Проект не найден')
      }
      
      // Обновление статистики
      updateStats()
      
      return true
    } catch (err) {
      state.value.error = 'Ошибка удаления проекта: ' + (err instanceof Error ? err.message : String(err))
      console.error('Error deleting project:', err)
      return false
    } finally {
      state.value.loading = false
    }
  }

  const updateStats = () => {
    const projects = state.value.projects
    const totalSize = projects.reduce((sum, p) => sum + p.fileSize, 0)
    const totalSheets = projects.reduce((sum, p) => sum + p.sheetCount, 0)
    const generatedModelsCount = projects.reduce((sum, p) => sum + p.generatedModels.length, 0)
    
    state.value.stats = {
      total: projects.length,
      totalSize: totalSize,
      recentCount: projects.filter(p => {
        const diffDays = (Date.now() - p.lastModified.getTime()) / (1000 * 60 * 60 * 24)
        return diffDays <= 7
      }).length,
      lastActive: projects.length > 0 ? 
        new Date(Math.max(...projects.map(p => p.lastModified.getTime()))) : 
        null,
      avgSheetCount: projects.length > 0 ? totalSheets / projects.length : 0,
      generatedModelsCount: generatedModelsCount
    }
  }

  const importProject = async (filePath: string): Promise<Project | null> => {
    try {
      state.value.loading = true
      state.value.error = null
      
      // Имитация импорта проекта
      await new Promise(resolve => setTimeout(resolve, 500))
      
      // Генерация данных для импортированного проекта
      const now = new Date()
      const fileName = filePath.split('/').pop()?.split('.')[0] || 'импортированный проект'
      
      const importedProject: Project = {
        id: `import-${Date.now().toString(36)}`,
        name: fileName,
        thumbnail: '/thumbnails/import.jpg',
        lastModified: now,
        fileSize: 1.8,
        tags: ['импорт', 'внешний'],
        filePath: filePath,
        sheetCount: 4,
        modelType: 'импорт',
        generatedModels: []
      }
      
      state.value.projects.unshift(importedProject)
      updateStats()
      
      return importedProject
    } catch (err) {
      state.value.error = 'Ошибка импорта проекта: ' + (err instanceof Error ? err.message : String(err))
      console.error('Error importing project:', err)
      return null
    } finally {
      state.value.loading = false
    }
  }

  // Методы для работы с автосохранением
  const setAutoSaveEnabled = (enabled: boolean) => {
    state.value.autoSaveEnabled = enabled
  }

  const setAutoSaveInterval = (interval: number) => {
    state.value.autoSaveInterval = interval
  }

  // Методы для работы с сгенерированными моделями
  const addGeneratedModel = (projectId: string, model: GeneratedModel) => {
    const project = state.value.projects.find(p => p.id === projectId)
    if (project) {
      project.generatedModels.push(model)
      project.lastModified = new Date()
      updateStats()
    }
  }

  const removeGeneratedModel = (projectId: string, modelId: string) => {
    const project = state.value.projects.find(p => p.id === projectId)
    if (project) {
      project.generatedModels = project.generatedModels.filter(m => m.id !== modelId)
      project.lastModified = new Date()
      updateStats()
    }
  }

  const setCurrentModel = (projectId: string, modelId: string) => {
    const project = state.value.projects.find(p => p.id === projectId)
    if (project) {
      project.currentModelId = modelId
      project.lastModified = new Date()
    }
  }

  // Инициализация при создании store
  loadProjects()
  
  return {
    projects,
    loading,
    error,
    stats,
    recentProjects,
    autoSaveEnabled,
    autoSaveInterval,
    loadProjects,
    createProject,
    deleteProject,
    importProject,
    setAutoSaveEnabled,
    setAutoSaveInterval,
    addGeneratedModel,
    removeGeneratedModel,
    setCurrentModel
  }
})