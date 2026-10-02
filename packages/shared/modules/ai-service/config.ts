import { OllamaClient } from './ollamaClient'
import type { IAiClient } from './client'
import type { AiBackendConfig } from '../types/ai.types'

export function createAiClient(config: AiBackendConfig): IAiClient {
  switch (config.type) {
    case 'ollama':
      return new OllamaClient(config.endpoint, config.model)
    
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
  }
}
