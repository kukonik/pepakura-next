import { defineStore } from 'pinia'

export interface AiBackendConfig {
  endpoint: string
  model: string
  temperature: number
  maxTokens: number
  apiKey: string
}

export interface AiState {
  activeProvider: 'ollama' | 'openai' | 'custom'
  providers: Record<'ollama' | 'openai' | 'custom', AiBackendConfig>
  isBusy: boolean
  lastError: string | null
  requestHistory: Array<{
    timestamp: Date
    provider: string
    status: 'success' | 'error'
    message: string
  }>
}

export const useAiStore = defineStore('ai', {
  state: (): AiState => ({
    activeProvider: 'ollama',
    providers: {
      ollama: {
        endpoint: 'http://localhost:11434',
        model: 'llama3',
        temperature: 0.7,
        maxTokens: 2048,
        apiKey: ''
      },
      openai: {
        endpoint: 'https://api.openai.com/v1',
        model: 'gpt-4',
        temperature: 0.7,
        maxTokens: 2048,
        apiKey: ''
      },
      custom: {
        endpoint: '',
        model: '',
        temperature: 0.7,
        maxTokens: 2048,
        apiKey: ''
      }
    },
    isBusy: false,
    lastError: null,
    requestHistory: []
  }),

  getters: {
    currentProviderConfig: (state) => state.providers[state.activeProvider],
    isProviderValid: (state) => {
      const config = state.providers[state.activeProvider]
      if (state.activeProvider === 'ollama' || state.activeProvider === 'openai') {
        return config.endpoint && config.model
      }
      return config.endpoint && config.model && config.apiKey
    }
  },

  actions: {
    setActiveProvider(provider: 'ollama' | 'openai' | 'custom') {
      this.activeProvider = provider
    },

    updateProviderConfig(provider: 'ollama' | 'openai' | 'custom', config: Partial<AiBackendConfig>) {
      this.providers[provider] = { ...this.providers[provider], ...config }
    },

    setBusy(busy: boolean) {
      this.isBusy = busy
    },

    setError(error: string) {
      this.lastError = error
    },

    addRequestToHistory(provider: string, status: 'success' | 'error', message: string) {
      this.requestHistory.unshift({
        timestamp: new Date(),
        provider,
        status,
        message
      })
      // Ограничиваем историю 20 записями
      if (this.requestHistory.length > 20) {
        this.requestHistory.pop()
      }
    },

    resetError() {
      this.lastError = null
    }
  }
})
