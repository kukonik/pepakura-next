import { defineStore } from 'pinia';
import { ref } from 'vue';
import { usePlatform } from '@pepakura/shared/composables/usePlatform';
import type { AppSettings, UnfoldConfig, AiConfig } from '@/types';

const defaultSettings: AppSettings = {
  language: 'ru',
  theme: 'system',
  defaultExportPath: '',
  unfoldConfig: {
    preserveDetail: true,
    maxIterations: 100,
    tolerance: 1e-6,
  },
  aiConfig: {
    provider: 'ollama',
    model: 'llama3.2',
    apiKey: '',
  },
};

export const useSettingsStore = defineStore('settings', () => {
  const settings = ref<AppSettings>(defaultSettings);
  const { invoke } = usePlatform();

  /**
   * Загрузить настройки из платформенного хранилища
   */
  const load = async () => {
    try {
      const loaded = await invoke<AppSettings>('get_settings');
      settings.value = { ...defaultSettings, ...loaded };
    } catch (error) {
      console.error('Failed to load settings:', error);
      // Оставляем настройки по умолчанию
    }
  };

  /**
   * Сохранить настройки в платформенное хранилище
   */
  const save = async () => {
    try {
      await invoke('save_settings', { settings: settings.value });
    } catch (error) {
      console.error('Failed to save settings:', error);
      throw error;
    }
  };

  /**
   * Сбросить настройки к значениям по умолчанию
   */
  const reset = () => {
    settings.value = { ...defaultSettings };
  };

  /**
   * Обновить настройки развёртки
   */
  const updateUnfoldConfig = (config: Partial<UnfoldConfig>) => {
    settings.value.unfoldConfig = { ...settings.value.unfoldConfig, ...config };
  };

  /**
   * Обновить AI конфигурацию
   */
  const updateAiConfig = (config: Partial<AiConfig>) => {
    settings.value.aiConfig = { ...settings.value.aiConfig, ...config };
  };

  return {
    settings,
    load,
    save,
    reset,
    updateUnfoldConfig,
    updateAiConfig,
  };
});