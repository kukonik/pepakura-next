/**
 * Хранилище настроек развёртки
 */

import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import type { UnfoldPresetConfig } from './presets.store';

export interface UnfoldSettings {
  // Параметры алгоритма
  algorithm: 'mds' | 'lscm';
  maxIterations: number;
  tolerance: number;
  preserveDetail: boolean;
  
  // Параметры бумаги и масштаба
  paperSize: 'A4' | 'A3' | 'Letter' | 'Custom';
  scale: number;
  tabSize: number; // мм
  margin: number; // мм
  overlap: number; // мм
  
  // Дополнительные параметры
  quality: 'low' | 'medium' | 'high';
  orientation: 'portrait' | 'landscape';
  
  // Флаги
  autoArrange: boolean;
  showTabs: boolean;
  showSeams: boolean;
}

// Настройки по умолчанию (соответствуют пресету "Бумага А4")
const DEFAULT_SETTINGS: UnfoldSettings = {
  algorithm: 'lscm',
  maxIterations: 100,
  tolerance: 0.001,
  preserveDetail: true,
  
  paperSize: 'A4',
  scale: 1.0,
  tabSize: 10,
  margin: 5,
  overlap: 2,
  
  quality: 'medium',
  orientation: 'portrait',
  
  autoArrange: true,
  showTabs: true,
  showSeams: true,
};

const STORAGE_KEY = 'pepakura-unfold-settings';

export const useSettingsStore = defineStore('settings', () => {
  // Загружаем настройки из LocalStorage
  const loadStoredSettings = (): UnfoldSettings => {
    try {
      const stored = localStorage.getItem(STORAGE_KEY);
      if (stored) {
        const parsed = JSON.parse(stored);
        // Убедимся, что все поля присутствуют
        return { ...DEFAULT_SETTINGS, ...parsed };
      }
    } catch (error) {
      console.error('Ошибка загрузки настроек из LocalStorage:', error);
    }
    return { ...DEFAULT_SETTINGS };
  };

  // Реактивное состояние
  const settings = ref<UnfoldSettings>(loadStoredSettings());

  // Сохранение настроек в LocalStorage
  const saveSettings = () => {
    try {
      localStorage.setItem(STORAGE_KEY, JSON.stringify(settings.value));
    } catch (error) {
      console.error('Ошибка сохранения настроек в LocalStorage:', error);
    }
  };

  // Обновление настроек
  const updateSettings = (updates: Partial<UnfoldSettings>) => {
    settings.value = { ...settings.value, ...updates };
    saveSettings();
  };

  // Сброс к значениям по умолчанию
  const resetToDefaults = () => {
    settings.value = { ...DEFAULT_SETTINGS };
    saveSettings();
  };

  // Применение конфигурации из пресета
  const applyPresetConfig = (config: UnfoldPresetConfig) => {
    updateSettings({
      algorithm: config.algorithm ?? settings.value.algorithm,
      maxIterations: config.maxIterations ?? settings.value.maxIterations,
      tolerance: config.tolerance ?? settings.value.tolerance,
      preserveDetail: config.preserveDetail ?? settings.value.preserveDetail,
      paperSize: config.paperSize ?? settings.value.paperSize,
      scale: config.scale ?? settings.value.scale,
      tabSize: config.tabSize ?? settings.value.tabSize,
      margin: config.margin ?? settings.value.margin,
      overlap: config.overlap ?? settings.value.overlap,
      quality: config.quality ?? settings.value.quality,
      orientation: config.orientation ?? settings.value.orientation,
    });
  };

  // Геттеры для удобства
  const algorithm = computed(() => settings.value.algorithm);
  const paperSize = computed(() => settings.value.paperSize);
  const scale = computed(() => settings.value.scale);
  const tabSize = computed(() => settings.value.tabSize);
  const margin = computed(() => settings.value.margin);
  const overlap = computed(() => settings.value.overlap);
  const quality = computed(() => settings.value.quality);
  const orientation = computed(() => settings.value.orientation);
  const autoArrange = computed(() => settings.value.autoArrange);
  const showTabs = computed(() => settings.value.showTabs);
  const showSeams = computed(() => settings.value.showSeams);

  // Конфиг для unfold worker
  const unfoldConfig = computed(() => ({
    algorithm: settings.value.algorithm,
    maxIterations: settings.value.maxIterations,
    tolerance: settings.value.tolerance,
    preserveDetail: settings.value.preserveDetail,
  }));

  return {
    // State
    settings,
    
    // Getters
    algorithm,
    paperSize,
    scale,
    tabSize,
    margin,
    overlap,
    quality,
    orientation,
    autoArrange,
    showTabs,
    showSeams,
    unfoldConfig,
    
    // Actions
    updateSettings,
    resetToDefaults,
    applyPresetConfig,
    saveSettings,
  };
});