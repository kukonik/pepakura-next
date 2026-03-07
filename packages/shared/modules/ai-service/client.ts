// Базовый интерфейс для AI-клиентов
export interface IAiClient {
  chat(request: import('../shared/types/ai.types').AiChatRequest): Promise<import('../shared/types/ai.types').AiChatResponse>
  abort(): void
}
