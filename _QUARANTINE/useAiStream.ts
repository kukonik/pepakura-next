/**
 * Composable для AI стриминга.
 * 
 * Предоставляет:
 * - Стриминг ответов от AI
 * - Прогресс генерации
 * - Обработку ошибок
 */

import { ref, computed } from 'vue'
import { listen, type UnlistenFn } from '@tauri-apps/api/event'
import { usePlatform } from '@shared/composables/usePlatform'

export interface StreamToken {
  token: string
  total_tokens: number
  done: boolean
}

export interface UseAiStreamOptions {
  onToken?: (token: string, totalTokens: number) => void
  onComplete?: (fullResponse: string, totalTokens: number) => void
  onError?: (error: string) => void
}

export function useAiStream(options: UseAiStreamOptions = {}) {
  const isStreaming = ref(false)
  const currentResponse = ref('')
  const totalTokens = ref(0)
  const error = ref<string | null>(null)
  const progress = computed(() => {
    // Прогресс на основе токенов (примерный)
    return Math.min(100, (totalTokens.value / 100) * 100)
  })

  const { invoke } = usePlatform()

  let unlistenToken: UnlistenFn | null = null
  let unlistenDone: UnlistenFn | null = null
  let unlistenError: UnlistenFn | null = null

  // Подписка на события стриминга
  const setupListeners = () => {
    // Слушаем токены
    unlistenToken = listen<AiStreamResponse>('ai-stream-token', (event) => {
      const payload = event.payload
      currentResponse.value += payload.token
      totalTokens.value = payload.total_tokens
      
      if (options.onToken) {
        options.onToken(payload.token, payload.total_tokens)
      }
    })

    // Слушаем завершение
    unlistenDone = listen<AiStreamResponse>('ai-stream-done', (event) => {
      const payload = event.payload
      
      if (options.onComplete) {
        options.onComplete(currentResponse.value, payload.total_tokens)
      }
      
      isStreaming.value = false
      cleanupListeners()
    })

    // Слушаем ошибки
    unlistenError = listen<string>('ai-stream-error', (event) => {
      error.value = event.payload
      isStreaming.value = false
      
      if (options.onError) {
        options.onError(event.payload)
      }
      
      cleanupListeners()
    })
  }

  // Очистка слушателей
  const cleanupListeners = () => {
    unlistenToken?.()
    unlistenDone?.()
    unlistenError?.()
    unlistenToken = null
    unlistenDone = null
    unlistenError = null
  }

  // Запуск стриминга
  const streamChat = async (
    message: string,
    history: Array<{ role: string; content: string }> = []
  ) => {
    if (isStreaming.value) {
      console.warn('Already streaming')
      return
    }

    isStreaming.value = true
    currentResponse.value = ''
    totalTokens.value = 0
    error.value = null

    setupListeners()

    try {
      await invoke('ai_chat_stream', {
        message,
        history,
      })
    } catch (e) {
      error.value = e instanceof Error ? e.message : String(e)
      isStreaming.value = false
      cleanupListeners()
      
      if (options.onError) {
        options.onError(error.value)
      }
    }
  }

  // Остановка стриминга
  const stopStreaming = () => {
    isStreaming.value = false
    cleanupListeners()
  }

  // Получение полного ответа (без стриминга)
  const chatComplete = async (
    message: string
  ): Promise<string> => {
    try {
      const response = await invoke<string>('ai_chat_complete', {
        message,
      })
      return response
    } catch (e) {
      error.value = e instanceof Error ? e.message : String(e)
      throw error.value
    }
  }

  // Сброс состояния
  const reset = () => {
    stopStreaming()
    currentResponse.value = ''
    totalTokens.value = 0
    error.value = null
  }

  return {
    // State
    isStreaming,
    currentResponse,
    totalTokens,
    error,
    progress,

    // Actions
    streamChat,
    stopStreaming,
    chatComplete,
    reset,
  }
}
