/**
 * Клиент для работы с AI-сервисом
 */

import type { AiBackendConfig, ChatMessage, AiResponse } from '@pepakura-next/shared';
import { OllamaClient } from './ollamaClient';
import { HttpAiClient } from './httpAiClient';

/**
 * Клиент для работы с AI-сервисом
 */
export class AiServiceClient {
  /**
   * Отправляет сообщение в AI API и возвращает ответ
   * @param messages Массив сообщений чата
   * @param config Конфигурация AI бэкенда
   * @returns Ответ от AI API
   */
  async sendMessage(messages: ChatMessage[], config: AiBackendConfig): Promise<AiResponse> {
    // Выбираем клиент в зависимости от типа бэкенда
    let client;
    
    switch (config.backendType) {
      case 'ollama':
        client = new OllamaClient();
        break;
      case 'openai':
      case 'custom':
        client = new HttpAiClient();
        break;
      default:
        throw new Error(`Unsupported AI backend type: ${config.backendType}`);
    }
    
    // Отправляем сообщение через выбранный клиент
    return client.sendMessage(messages, config);
  }
}