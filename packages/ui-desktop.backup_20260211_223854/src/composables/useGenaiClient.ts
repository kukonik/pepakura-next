import { ref, computed } from 'vue'
import type { AiBackendConfig } from '../../../shared/src/ai/AiBackendConfig'

// Mock implementation of AI client
export const useGenaiClient = () => {
  const isProcessing = ref(false)
  const lastResponse = ref<string | null>(null)
  const error = ref<string | null>(null)
  
  // In a real implementation, this would be loaded from store or props
  const config = ref<AiBackendConfig>({
    backend: 'ollama',
    endpoint: 'http://localhost:11434/api/chat',
    model: 'llama2',
    temperature: 0.7,
    maxTokens: 2048
  })
  
  const isConfigured = computed(() => {
    return config.value.endpoint && config.value.model
  })
  
  const sendPrompt = async (prompt: string) => {
    if (!isConfigured.value) {
      error.value = 'AI не настроен'
      return null
    }
    
    try {
      isProcessing.value = true
      error.value = null
      
      // Simulate API call
      await new Promise(resolve => setTimeout(resolve, 1000))
      
      const response = `Это симуляция ответа от ${config.value.backend}. Пользователь спросил: "${prompt}"`
      lastResponse.value = response
      
      return response
    } catch (err) {
      error.value = 'Ошибка при отправке запроса'
      console.error('Failed to send prompt:', err)
      return null
    } finally {
      isProcessing.value = false
    }
  }
  
  const updateConfig = (newConfig: Partial<AiBackendConfig>) => {
    config.value = { ...config.value, ...newConfig }
  }
  
  return {
    isProcessing: isProcessing.value,
    lastResponse: lastResponse.value,
    error: error.value,
    config: config.value,
    isConfigured: isConfigured.value,
    sendPrompt,
    updateConfig
  }
}