import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import type { Project } from '../types/project'

export const useProjectsStore = defineStore('projects', () => {
  // State
  const projects = ref<Project[]>([])
  const currentProjectId = ref<string | null>(null)
  const isLoading = ref(false)
  const error = ref<string | null>(null)

  // Computed
  const currentProject = computed(() => {
    if (!currentProjectId.value) return null
    return projects.value.find(p => p.id === currentProjectId.value) || null
  })

  const recentProjects = computed(() => {
    return [...projects.value]
      .sort((a, b) => 
        new Date(b.lastOpened).getTime() - new Date(a.lastOpened).getTime()
      )
      .slice(0, 5)
  })

  // Actions
  const loadProjects = async () => {
    isLoading.value = true
    error.value = null
    
    try {
      // In a real app, this would call the Tauri API
      // For now, we'll use mock data
      const mockProjects: Project[] = [
        {
          id: '1',
          name: 'Моя первая модель',
          path: '/projects/first-model',
          createdAt: new Date('2024-01-01'),
          lastOpened: new Date('2024-01-15'),
          thumbnail: null
        },
        {
          id: '2',
          name: 'Коробка для телефона',
          path: '/projects/phone-box',
          createdAt: new Date('2024-01-10'),
          lastOpened: new Date('2024-01-12'),
          thumbnail: null
        }
      ]
      
      projects.value = mockProjects
    } catch (err) {
      error.value = 'Не удалось загрузить проекты'
      console.error('Failed to load projects:', err)
    } finally {
      isLoading.value = false
    }
  }

  const createProject = async (name: string, path: string) => {
    try {
      const newProject: Project = {
        id: Math.random().toString(36).substr(2, 9),
        name,
        path,
        createdAt: new Date(),
        lastOpened: new Date()
      }
      
      projects.value.push(newProject)
      currentProjectId.value = newProject.id
      
      return newProject
    } catch (err) {
      error.value = 'Не удалось создать проект'
      console.error('Failed to create project:', err)
      throw err
    }
  }

  const openProject = async (projectId: string) => {
    try {
      const project = projects.value.find(p => p.id === projectId)
      if (!project) {
        throw new Error('Проект не найден')
      }
      
      // Update last opened time
      project.lastOpened = new Date()
      currentProjectId.value = projectId
      
      // In a real app, this would load the project data
      return project
    } catch (err) {
      error.value = 'Не удалось открыть проект'
      console.error('Failed to open project:', err)
      throw err
    }
  }

  const closeProject = () => {
    currentProjectId.value = null
  }

  const deleteProject = async (projectId: string) => {
    try {
      const index = projects.value.findIndex(p => p.id === projectId)
      if (index === -1) {
        throw new Error('Проект не найден')
      }
      
      projects.value.splice(index, 1)
      
      if (currentProjectId.value === projectId) {
        currentProjectId.value = null
      }
    } catch (err) {
      error.value = 'Не удалось удалить проект'
      console.error('Failed to delete project:', err)
      throw err
    }
  }

  return {
    // State
    projects: projects.value,
    currentProjectId: currentProjectId.value,
    isLoading: isLoading.value,
    error: error.value,
    
    // Computed
    currentProject,
    recentProjects,
    
    // Actions
    loadProjects,
    createProject,
    openProject,
    closeProject,
    deleteProject
  }
})