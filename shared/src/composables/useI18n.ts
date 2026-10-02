import { ref, computed } from 'vue'

export type Locale = 'en' | 'ru'

const locale = ref<Locale>('en')

const messages = {
  en: {
    app: {
      title: 'Pepakura Next',
      description: 'Cross-platform papercraft unfolding with AI'
    },
    actions: {
      generate: 'Generate',
      import: 'Import',
      export: 'Export',
      save: 'Save',
      load: 'Load',
      web: 'Web',
      ai: 'AI',
      webSearch: 'Search on the web',
      aiSearch: 'Search with AI'
    },
    stages: {
      '3d': '3D Model',
      unfold: 'Unfold',
      nest: 'Nest',
      export: 'Export'
    },
    ai: {
      textTo3d: 'Text to 3D',
      imageTo3d: 'Image to 3D',
      generating: 'Generating...'
    },
    search: {
      placeholder: 'Describe a model, ask AI, or enter a web address...',
      suggestions: 'Suggestions'
    }
  },
  ru: {
    app: {
      title: 'Pepakura Next',
      description: 'Кроссплатформенное развёртывание бумажных моделей с ИИ'
    },
    actions: {
      generate: 'Сгенерировать',
      import: 'Импорт',
      export: 'Экспорт',
      save: 'Сохранить',
      load: 'Загрузить',
      web: 'Веб',
      ai: 'ИИ',
      webSearch: 'Искать в интернете',
      aiSearch: 'Искать с помощью ИИ'
    },
    stages: {
      '3d': '3D Модель',
      unfold: 'Развёртка',
      nest: 'Раскладка',
      export: 'Экспорт'
    },
    ai: {
      textTo3d: 'Текст в 3D',
      imageTo3d: 'Изображение в 3D',
      generating: 'Генерация...'
    },
    search: {
      placeholder: 'Опишите модель, задайте вопрос AI или введите веб-адрес...',
      suggestions: 'Предложения'
    }
  }
}

export function useI18n() {
  const t = (key: string) => {
    const keys = key.split('.')
    let value: any = messages[locale.value]
    for (const k of keys) {
      if (value && typeof value === 'object' && k in value) {
        value = value[k]
      } else {
        return key
      }
    }
    return value
  }

  const setLocale = (newLocale: Locale) => {
    locale.value = newLocale
  }

  return {
    t,
    locale: computed(() => locale.value),
    setLocale
  }
}