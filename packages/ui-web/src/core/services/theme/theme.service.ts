// src/core/services/theme/theme.service.ts
import { useAppStore } from '../../../stores/app.store'

export class ThemeService {
  private static instance: ThemeService
  private appStore = useAppStore()

  private constructor() {
    this.initializeTheme()
  }

  static getInstance(): ThemeService {
    if (!ThemeService.instance) {
      ThemeService.instance = new ThemeService()
    }
    return ThemeService.instance
  }

  private initializeTheme(): void {
    const savedTheme = localStorage.getItem('theme')
    if (savedTheme === 'dark' || savedTheme === 'light') {
      this.setTheme(savedTheme)
    } else {
      // Определяем системную тему
      const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches
      this.setTheme(prefersDark ? 'dark' : 'light')
    }
  }

  setTheme(theme: 'light' | 'dark'): void {
    this.appStore.setTheme(theme)
    localStorage.setItem('theme', theme)
    
    if (theme === 'dark') {
      document.documentElement.classList.add('dark')
    } else {
      document.documentElement.classList.remove('dark')
    }
  }

  toggleTheme(): void {
    const newTheme = this.appStore.theme === 'light' ? 'dark' : 'light'
    this.setTheme(newTheme)
  }
}

export const themeService = ThemeService.getInstance()
