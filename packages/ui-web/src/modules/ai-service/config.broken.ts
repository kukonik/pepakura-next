import { OllamaClient } from './ollamaClient'
import { HttpAiClient } from './httpAiClient'
import type { IAiClient } from './client'
import type { AiBackendConfig } from '../shared/types/ai.types'

export function createAiClient(config: AiBackendConfig): IAiClient {
  switch (config.type) {
    case 'ollama':
      return new OllamaClient(config.endpoint, config.model)
    
    case 'openai':
    case 'custom':
      return new HttpAiClient({
        endpoint: config.endpoint,
        apiKey: config.apiKey,
        defaultModel: config.model
      })
    
    default:
      throw new Error(Unsupported AI backend type: )
  }
}

// Дефолтные конфигурации для разных backend'ов
export const DEFAULT_AI_CONFIGS: Record<string, Partial<AiBackendConfig>> = {
  ollama: {
    endpoint: 'http://localhost:11434/api',
    model: 'gemma2:2b',
    temperature: 0.7,
    maxTokens: 2048
  },
  openai: {
    endpoint: 'https://api.openai.com/v1/chat/completions',
    model: 'gpt-3.5-turbo',
    temperature: 0.7,
    maxTokens: 2048
  }
}
