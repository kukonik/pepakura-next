import { defineStore } from 'pinia'
import { invoke } from '@tauri-apps/api/core'
import { listen, UnlistenFn } from '@tauri-apps/api/event'

export interface AiBackendConfig {
  endpoint: string
  model: string
  temperature: number
  maxTokens: number
  apiKey: string
}

export interface AiChatMessage {
  role: 'user' | 'assistant' | 'system'
  content: string
  timestamp: Date
  isStreaming?: boolean
}

export interface OllamaStatus {
  available: boolean
  models: string[]
  provider: string
  checking: boolean
  lastCheck: Date | null
}

export interface AiAnalysisResult {
  type: 'unfold' | 'distortion' | 'nesting'
  data: any
  timestamp: Date
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
  // Chat state
  chatMessages: AiChatMessage[]
  chatHistory: Array<{ role: string; content: string }>
  // Ollama status
  ollamaStatus: OllamaStatus
  // Analysis results
  analysisResults: AiAnalysisResult[]
  // Streaming
  isStreaming: boolean
  streamingMessage: string
  // Активные слушатели стриминга (для очистки)
  streamingUnlisteners: UnlistenFn[]
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
    requestHistory: [],
    // Chat state
    chatMessages: [],
    chatHistory: [],
    // Ollama status
    ollamaStatus: {
      available: false,
      models: [],
      provider: 'ollama',
      checking: false,
      lastCheck: null
    },
    // Analysis results
    analysisResults: [],
    // Streaming
    isStreaming: false,
    streamingMessage: '',
    // Активные слушатели стриминга (для очистки)
    streamingUnlisteners: []
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
    /**
     * Очистка активных слушателей стриминга
     */
    cleanupStreamingListeners() {
      this.streamingUnlisteners.forEach(unlisten => unlisten())
      this.streamingUnlisteners = []
    },

    // =========================================================================
    // Утилиты
    // =========================================================================

    /**
     * Парсит строку ошибки от Rust/Tauri и возвращает понятный текст для юзера.
     *
     * @param error - Необработанный объект ошибки
     * @returns Локализованное описание проблемы
     */
    parseOllamaError(error: unknown): string {
      // Извлекаем текст ошибки
      let rawMessage = ''
      if (error instanceof Error) {
        rawMessage = error.message
      } else if (typeof error === 'string') {
        rawMessage = error
      } else {
        rawMessage = JSON.stringify(error)
      }

      // Нормализуем для сравнения (нижний регистр)
      const msg = rawMessage.toLowerCase()

      // 1. Модель не найдена
      if (msg.includes('model not found') || msg.includes('pull model')) {
        return 'Модель не найдена. Убедитесь, что она загружена в Ollama (проверьте статус в панели).'
      }

      // 2. Нехватка видеопамяти
      if (msg.includes('out of memory') || msg.includes('oom') || msg.includes('cuda out of memory')) {
        return 'Не хватает видеопамяти (VRAM) для этой модели. Попробуйте выбрать модель меньшего размера (например, Q4_K_M вместо Q8_0).'
      }

      // 3. Отказ подключения
      if (msg.includes('connection refused') || msg.includes('econnrefused')) {
        return 'Не удалось подключиться к Ollama. Убедитесь, что сервер запущен.'
      }

      // 4. Таймаут
      if (msg.includes('timeout') || msg.includes('timed out')) {
        return 'Сервер Ollama не ответил вовремя. Возможно, модель слишком тяжёлая для вашего ПК.'
      }

      // 5. Всё остальное — обрезаем до 150 символов
      const trimmed = rawMessage.trim()
      if (trimmed.length > 150) {
        return trimmed.substring(0, 147) + '...'
      }
      return trimmed
    },

    // =========================================================================
    // Основные действия
    // =========================================================================

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
    },

    // =========================================================================
    // Ollama Status
    // =========================================================================

    /**
     * Проверка статуса Ollama через Tauri команду
     */
    async checkOllamaStatus() {
      this.ollamaStatus.checking = true
      try {
        const status = await invoke<any>('ai_check_status')
        this.ollamaStatus.available = status.available
        this.ollamaStatus.models = status.models || []
        this.ollamaStatus.provider = status.provider || 'ollama'
        this.ollamaStatus.lastCheck = new Date()
        return status
      } catch (error) {
        const friendlyMsg = this.parseOllamaError(error)
        this.ollamaStatus.available = false
        this.ollamaStatus.lastError = friendlyMsg
        this.lastError = friendlyMsg
        throw error
      } finally {
        this.ollamaStatus.checking = false
      }
    },

    /**
     * Запуск периодического обновления статуса
     */
    startStatusPolling(intervalMs: number = 10000) {
      setInterval(async () => {
        try {
          await this.checkOllamaStatus()
        } catch (err) {
          // Тихая обработка ошибок при поллинге
          console.warn('Ollama status check failed:', err)
        }
      }, intervalMs)
    },

    // =========================================================================
    // Chat
    // =========================================================================

    /**
     * Отправка сообщения в AI чат через Tauri с поддержкой стриминга.
     *
     * Метод запускает асинхронный стриминг через Tauri events и обновляет
     * сообщение AI в реальном времени по мере поступления чанков.
     *
     * @param message - Текст сообщения пользователя
     * @returns Promise<void> (стриминг продолжается после разрешения Promise)
     */
    async sendMessageToAI(message: string): Promise<void> {
      if (!message.trim()) {
        throw new Error('Message cannot be empty')
      }

      // Очищаем предыдущие слушатели если они остались
      this.cleanupStreamingListeners()

      this.isBusy = true
      this.lastError = null
      this.isStreaming = true
      this.streamingMessage = ''

      try {
        // 1. Добавляем сообщение пользователя в историю и чат
        this.chatHistory.push({ role: 'user', content: message })
        const userMessage: AiChatMessage = {
          role: 'user',
          content: message.trim(),
          timestamp: new Date()
        }
        this.chatMessages.push(userMessage)

        // 2. Добавляем "пустое" сообщение AI, которое будем заполнять стримом
        const aiMessage: AiChatMessage = {
          role: 'assistant',
          content: '',
          timestamp: new Date(),
          isStreaming: true
        }
        this.chatMessages.push(aiMessage)

        // Сохраняем ссылку на сообщение AI для реактивного обновления
        const currentAiMessage = aiMessage

        // 3. Запускаем стриминг через Tauri (функция теперь не возвращает текст)
        await invoke('ai_chat_stream_native', {
          message: message.trim(),
          history: this.chatHistory.slice(0, -1) // Передаём историю без текущего сообщения
        })

        // 4. Настраиваем слушатели для приёма стриминговых данных
        const unlistenChunk = await listen<string>(
          'ollama-stream-chunk',
          (event) => {
            // Добавляем полученный чанк к сообщению AI
            currentAiMessage.content += event.payload
            this.streamingMessage = currentAiMessage.content
          }
        )
        this.streamingUnlisteners.push(unlistenChunk)

        const unlistenDone = await listen(
          'ollama-stream-done',
          () => {
            this.isBusy = false
            this.isStreaming = false
            currentAiMessage.isStreaming = false

            // Добавляем полный ответ в историю
            this.chatHistory.push({
              role: 'assistant',
              content: currentAiMessage.content
            })

            this.addRequestToHistory('ollama', 'success', message)
            this.cleanupStreamingListeners()
          }
        )
        this.streamingUnlisteners.push(unlistenDone)

        const unlistenError = await listen<string>(
          'ollama-stream-error',
          (event) => {
            const errorMsg = this.parseOllamaError(event.payload)
            this.lastError = errorMsg
            this.isBusy = false
            this.isStreaming = false
            currentAiMessage.isStreaming = false
            currentAiMessage.content = ''

            this.addRequestToHistory('ollama', 'error', errorMsg)
            this.cleanupStreamingListeners()
          }
        )
        this.streamingUnlisteners.push(unlistenError)

        // Promise разрешается сразу после запуска стрима,
        // данные продолжают поступать через events
      } catch (error) {
        // Ошибка запуска стрима (не ошибка обработки)
        const errorMsg = this.parseOllamaError(error)
        this.lastError = errorMsg
        this.isBusy = false
        this.isStreaming = false
        this.addRequestToHistory('ollama', 'error', errorMsg)
        this.cleanupStreamingListeners()
        throw error
      }
    },

    /**
     * Добавление сообщения пользователя в чат
     */
    addUserMessage(message: string) {
      const userMessage: AiChatMessage = {
        role: 'user',
        content: message,
        timestamp: new Date()
      }
      this.chatMessages.push(userMessage)
    },

    /**
     * Очистка истории чата
     */
    clearChat() {
      this.chatMessages = []
      this.chatHistory = []
    },

    // =========================================================================
    // Analysis
    // =========================================================================

    /**
     * Анализ модели через AI
     */
    async analyzeModel(mesh: any) {
      this.isBusy = true
      this.lastError = null

      try {
        // Получаем рекомендации по развёртке
        const unfoldAdvice = await invoke<any>('ai_get_unfold_advice', { mesh })

        const result: AiAnalysisResult = {
          type: 'unfold',
          data: unfoldAdvice,
          timestamp: new Date()
        }

        this.analysisResults.push(result)
        this.addRequestToHistory('ollama', 'success', 'Model analysis')
        return result
      } catch (error) {
        const errorMsg = this.parseOllamaError(error)
        this.lastError = errorMsg
        this.addRequestToHistory('ollama', 'error', errorMsg)
        throw error
      } finally {
        this.isBusy = false
      }
    },

    /**
     * Анализ искажений развёртки
     */
    async analyzeDistortion(mesh: any, unfolded: any) {
      this.isBusy = true
      this.lastError = null

      try {
        const analysis = await invoke<any>('ai_analyze_distortion', {
          mesh,
          unfolded
        })

        const result: AiAnalysisResult = {
          type: 'distortion',
          data: analysis,
          timestamp: new Date()
        }

        this.analysisResults.push(result)
        return result
      } catch (error) {
        const errorMsg = this.parseOllamaError(error)
        this.lastError = errorMsg
        throw error
      } finally {
        this.isBusy = false
      }
    },

    /**
     * Очистка результатов анализа
     */
    clearAnalysisResults() {
      this.analysisResults = []
    }
  }
})
