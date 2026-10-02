import type { IAiClient } from './client'
import type { AiChatRequest, AiChatResponse, OllamaChatRequest, OllamaChatResponse } from '../shared/types/ai.types'

export class OllamaClient implements IAiClient {
  private controller: AbortController | null = null
  private readonly baseUrl: string
  private readonly defaultModel: string

  constructor(
    private readonly endpoint: string,
    private readonly model: string
  ) {
    this.baseUrl = endpoint.endsWith('/') ? endpoint : ${endpoint}/
    this.defaultModel = model
  }

  async chat(request: AiChatRequest): Promise<AiChatResponse> {
    this.controller = new AbortController()
    
    const ollamaRequest: OllamaChatRequest = {
      model: request.model || this.defaultModel,
      messages: request.messages.map(msg => ({
        role: msg.role,
        content: msg.content
      })),
      stream: false,
      options: {
        temperature: request.temperature,
        num_predict: request.maxTokens
      }
    }

    try {
      const response = await fetch(${this.baseUrl}chat, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify(ollamaRequest),
        signal: this.controller.signal
      })

      if (!response.ok) {
        throw new Error(HTTP error! status: )
      }

      const data: OllamaChatResponse = await response.json()
      
      return {
        message: {
          role: 'assistant',
          content: data.message.content,
          timestamp: new Date()
        },
        finishReason: data.done ? 'stop' : undefined
      }
    } catch (error) {
      if (error instanceof Error && error.name === 'AbortError') {
        throw new Error('Request aborted')
      }
      throw error
    }
  }

  abort(): void {
    if (this.controller) {
      this.controller.abort()
    }
  }
}
