import { defineStore } from 'pinia'
import { ref, computed } from 'vue'

export const useUiStore = defineStore('ui', () => {
  // Theme state
  const isDarkMode = ref(false)
  
  // Sidebar state
  const isSidebarOpen = ref(true)
  
  // Loading states
  const isLoading = ref(false)
  const loadingMessage = ref('')
  
  // Notifications
  const notifications = ref<Array<{
    id: string
    type: 'success' | 'error' | 'warning' | 'info'
    message: string
    timestamp: Date
  }>>([])

  // Computed
  const themeClass = computed(() => isDarkMode.value ? 'dark-theme' : 'light-theme')

  // Actions
  const toggleDarkMode = () => {
    isDarkMode.value = !isDarkMode.value
    // Save to localStorage
    localStorage.setItem('pepakura-theme', isDarkMode.value ? 'dark' : 'light')
  }

  const toggleSidebar = () => {
    isSidebarOpen.value = !isSidebarOpen.value
  }

  const showLoading = (message = 'Загрузка...') => {
    isLoading.value = true
    loadingMessage.value = message
  }

  const hideLoading = () => {
    isLoading.value = false
    loadingMessage.value = ''
  }

  const addNotification = (notification: {
    type: 'success' | 'error' | 'warning' | 'info'
    message: string
  }) => {
    const newNotification = {
      id: Math.random().toString(36).substr(2, 9),
      type: notification.type,
      message: notification.message,
      timestamp: new Date()
    }
    
    notifications.value.push(newNotification)
    
    // Auto-remove after 5 seconds
    setTimeout(() => {
      removeNotification(newNotification.id)
    }, 5000)
  }

  const removeNotification = (id: string) => {
    const index = notifications.value.findIndex(n => n.id === id)
    if (index !== -1) {
      notifications.value.splice(index, 1)
    }
  }

  const clearNotifications = () => {
    notifications.value = []
  }

  return {
    // State
    isDarkMode,
    isSidebarOpen,
    isLoading,
    loadingMessage,
    notifications,
    
    // Computed
    themeClass,
    
    // Actions
    toggleDarkMode,
    toggleSidebar,
    showLoading,
    hideLoading,
    addNotification,
    removeNotification,
    clearNotifications
  }
})