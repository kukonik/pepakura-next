export interface BaseAiMessage {
  role: 'user' | 'assistant' | 'system'
  content: string
  timestamp: Date
}

export interface AiChatRequest {
  messages: BaseAiMessage[]
  model: string
  temperature?: number
  maxTokens?: number
}

export interface AiChatResponse {
  message: BaseAiMessage
  finishReason?: string
}

export interface AiBackendConfig {
  type: 'ollama' | 'openai' | 'custom'
  endpoint: string
  apiKey?: string
  model: string
  temperature: number
  maxTokens: number
}

export interface AiSession {
  id: string
  projectId?: string
  title: string
  messages: BaseAiMessage[]
  createdAt: Date
  updatedAt: Date
}

export interface OllamaChatRequest {
  model: string
  messages: Array<{ role: string; content: string }>
  stream?: boolean
  options?: {
    temperature?: number
    num_predict?: number
  }
}

export interface OllamaChatResponse {
  model: string
  created_at: string
  message: {
    role: string
    content: string
  }
  done: boolean
  total_duration?: number
  load_duration?: number
  prompt_eval_count?: number
  prompt_eval_duration?: number
  eval_count?: number
  eval_duration?: number
}
