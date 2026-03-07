/**
 * Типы для AI-сервиса
 */

import type { AiBackendConfig, ChatMessage, AiResponse } from '@pepakura-next/shared';

/**
 * Интерфейс AI-клиента
 */
export interface IAiClient {
  /**
   * Отправляет сообщение в AI API и возвращает ответ
   * @param messages Массив сообщений чата
   * @param config Конфигурация AI бэкенда
   * @returns Ответ от AI API
   */
  sendMessage(messages: ChatMessage[], config: AiBackendConfig): Promise<AiResponse>;
}