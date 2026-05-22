/**
 * Хранилище пресетов настроек развёртки
 */

import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import { useSettingsStore } from './settings.store';

export interface UnfoldPresetConfig {
  // Параметры алгоритма развёртки
  algorithm?: 'mds' | 'lscm';
  maxIterations?: number;
  tolerance?: number;
  preserveDetail?: boolean;
  
  // Параметры бумаги и масштаба
  paperSize?: 'A4' | 'A3' | 'Letter' | 'Custom';
  scale?: number; // Масштаб (например, 1.0 = 100%)
  tabSize?: number; // Размер клапана в мм
  margin?: number; // Отступ от края в мм
  overlap?: number; // Перекрытие деталей в мм
  
  // Дополнительные параметры
  quality?: 'low' | 'medium' | 'high';
  orientation?: 'portrait' | 'landscape';
}

export interface UnfoldPreset {
  id: string;
  name: string; // "Стандартная бумага"
  icon: string; // "📄"
  category: 'standard' | 'custom'; // Стандартные или пользовательские
  config: UnfoldPresetConfig;
}

// Дефолтные пресеты
const DEFAULT_PRESETS: UnfoldPreset[] = [
  {
    id: 'paper-a4',
    name: 'Бумага А4',
    icon: '📄',
    category: 'standard',
    config: {
      paperSize: 'A4',
      scale: 1.0,
      tabSize: 10,
      margin: 5,
      overlap: 2,
      algorithm: 'lscm',
      maxIterations: 100,
      tolerance: 0.001,
      preserveDetail: true,
      quality: 'medium',
      orientation: 'portrait',
    },
  },
  {
    id: 'cardboard',
    name: 'Картон',
    icon: '📦',
    category: 'standard',
    config: {
      paperSize: 'A4',
      scale: 1.0,
      tabSize: 15,
      margin: 8,
      overlap: 3,
      algorithm: 'mds',
      maxIterations: 150,
      tolerance: 0.005,
      preserveDetail: false,
      quality: 'high',
      orientation: 'portrait',
    },
  },
  {
    id: 'small-parts',
    name: 'Мелкие детали',
    icon: '🔍',
    category: 'standard',
    config: {
      paperSize: 'A4',
      scale: 2.0,
      tabSize: 5,
      margin: 2,
      overlap: 1,
      algorithm: 'lscm',
      maxIterations: 200,
      tolerance: 0.0005,
      preserveDetail: true,
      quality: 'high',
      orientation: 'landscape',
    },
  },
  {
    id: 'quick-print',
    name: 'Быстрая печать',
    icon: '⚡',
    category: 'standard',
    config: {
      paperSize: 'A4',
      scale: 1.0,
      tabSize: 8,
      margin: 5,
      overlap: 2,
      algorithm: 'mds',
      maxIterations: 50,
      tolerance: 0.01,
      preserveDetail: false,
      quality: 'low',
      orientation: 'portrait',
    },
  },
  {
    id: 'precise-model',
    name: 'Точная модель',
    icon: '🎯',
    category: 'standard',
    config: {
      paperSize: 'A3',
      scale: 1.5,
      tabSize: 12,
      margin: 10,
      overlap: 1,
      algorithm: 'lscm',
      maxIterations: 300,
      tolerance: 0.0001,
      preserveDetail: true,
      quality: 'high',
      orientation: 'landscape',
    },
  },
];

const STORAGE_KEY = 'pepakura-unfold-presets';

export const usePresetsStore = defineStore('presets', () => {
  // Загружаем пользовательские пресеты из LocalStorage
  const loadCustomPresets = (): UnfoldPreset[] => {
    try {
      const stored = localStorage.getItem(STORAGE_KEY);
      if (stored) {
        return JSON.parse(stored);
      }
    } catch (error) {
      console.error('Ошибка загрузки пресетов из LocalStorage:', error);
    }
    return [];
  };

  // Реактивные состояния
  const standardPresets = ref<UnfoldPreset[]>(DEFAULT_PRESETS);
  const customPresets = ref<UnfoldPreset[]>(loadCustomPresets());
  const activePresetId = ref<string | null>(DEFAULT_PRESETS[0].id);

  // Все пресеты (стандартные + пользовательские)
  const allPresets = computed(() => [
    ...standardPresets.value,
    ...customPresets.value,
  ]);

  // Активный пресет
  const activePreset = computed(() =>
    allPresets.value.find(p => p.id === activePresetId.value) || null
  );

  // Группированные пресеты для UI
  const groupedPresets = computed(() => ({
    standard: standardPresets.value,
    custom: customPresets.value,
  }));

  // Сохранение пользовательских пресетов в LocalStorage
  const saveCustomPresets = () => {
    try {
      localStorage.setItem(STORAGE_KEY, JSON.stringify(customPresets.value));
    } catch (error) {
      console.error('Ошибка сохранения пресетов в LocalStorage:', error);
    }
  };

  // Действия
  const addCustomPreset = (preset: Omit<UnfoldPreset, 'id' | 'category'>) => {
    const newPreset: UnfoldPreset = {
      ...preset,
      id: `custom-${Date.now()}-${Math.random().toString(36).substr(2, 9)}`,
      category: 'custom',
    };
    customPresets.value.push(newPreset);
    saveCustomPresets();
    return newPreset;
  };

  const updateCustomPreset = (id: string, updates: Partial<UnfoldPreset>) => {
    const index = customPresets.value.findIndex(p => p.id === id);
    if (index !== -1) {
      customPresets.value[index] = { ...customPresets.value[index], ...updates };
      saveCustomPresets();
    }
  };

  const deleteCustomPreset = (id: string) => {
    const index = customPresets.value.findIndex(p => p.id === id);
    if (index !== -1) {
      customPresets.value.splice(index, 1);
      saveCustomPresets();
      // Если удаляемый пресет был активным, переключаемся на первый стандартный
      if (activePresetId.value === id) {
        activePresetId.value = DEFAULT_PRESETS[0].id;
      }
    }
  };

  const setActivePreset = (id: string) => {
    if (allPresets.value.some(p => p.id === id)) {
      activePresetId.value = id;
    }
  };

  const applyPreset = (presetId: string) => {
    setActivePreset(presetId);
    const preset = allPresets.value.find(p => p.id === presetId);
    if (preset) {
      const settingsStore = useSettingsStore();
      settingsStore.applyPresetConfig(preset.config);
      return preset.config;
    }
    return null;
  };

  const resetToDefaults = () => {
    customPresets.value = [];
    activePresetId.value = DEFAULT_PRESETS[0].id;
    saveCustomPresets();
  };

  return {
    // State
    standardPresets,
    customPresets,
    activePresetId,
    
    // Getters
    allPresets,
    activePreset,
    groupedPresets,
    
    // Actions
    addCustomPreset,
    updateCustomPreset,
    deleteCustomPreset,
    setActivePreset,
    applyPreset,
    resetToDefaults,
  };
});