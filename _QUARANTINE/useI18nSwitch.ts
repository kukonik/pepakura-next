import { useI18n } from 'vue-i18n'
import { useSettingsStore } from '@/stores/settings.store'
import { usePlatform } from '@shared/composables/usePlatform'

export function useI18nSwitch() {
  const { locale } = useI18n()
  const settingsStore = useSettingsStore()
  const { invoke } = usePlatform()

  const switchLanguage = async (lang: 'ru' | 'en') => {
    // Обновляем локаль Vue I18n
    locale.value = lang

    // Сохраняем в настройки Pinia store
    settingsStore.settings.language = lang
    await settingsStore.save()

    // Сохраняем через Platform Bridge
    try {
      await invoke('save_settings', { settings: settingsStore.settings })
    } catch (error) {
      console.warn('Failed to save settings via platform:', error)
    }
  }

  const toggleLanguage = () => {
    const newLang = locale.value === 'ru' ? 'en' : 'ru'
    switchLanguage(newLang)
  }

  const currentLanguage = () => locale.value as 'ru' | 'en'

  return {
    switchLanguage,
    toggleLanguage,
    currentLanguage,
  }
}