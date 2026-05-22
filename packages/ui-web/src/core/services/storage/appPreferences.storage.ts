// src/core/services/storage/appPreferences.storage.ts
import { useAppStore } from '../../../stores/app.store'

export class AppPreferencesStorage {
  private static instance: AppPreferencesStorage
  private appStore = useAppStore()

  private constructor() {
    this.loadPreferences()
  }

  static getInstance(): AppPreferencesStorage {
    if (!AppPreferencesStorage.instance) {
      AppPreferencesStorage.instance = new AppPreferencesStorage()
    }
    return AppPreferencesStorage.instance
  }

  private loadPreferences(): void {
    try {
      const saved = localStorage.getItem('appPreferences')
      if (saved) {
        const preferences = JSON.parse(saved)
        this.appStore.setPreferences(preferences)
      }
    } catch (error) {
      console.error('Failed to load preferences:', error)
    }
  }

  savePreferences(preferences: any): void {
    try {
      localStorage.setItem('appPreferences', JSON.stringify(preferences))
      this.appStore.setPreferences(preferences)
    } catch (error) {
      console.error('Failed to save preferences:', error)
    }
  }
}

export const appPreferencesStorage = AppPreferencesStorage.getInstance()
