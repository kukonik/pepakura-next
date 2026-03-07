/**
 * Клиент для работы с HTTP-совместимыми AI API (например, OpenAI)
 */

import type { AiBackendConfig, ChatMessage, AiResponse } from '@pepakura-next/shared';

/**
 * Клиент для работы с HTTP-совместимыми AI API
 */
export class HttpAiClient {
  /**
   * Отправляет сообщение в HTTP-совместимый AI API и возвращает ответ
   * @param messages Массив сообщений чата
   * @param config Конфигурация AI бэкенда
   * @returns Ответ от AI API
   */
  async sendMessage(messages: ChatMessage[], config: AiBackendConfig): Promise<AiResponse> {
    // Создаем тело запроса в формате OpenAI
    const requestBody = {
      model: config.model,
      messages: messages,
      temperature: config.generationParams.temperature,
      top_p: config.generationParams.topP,
      max_tokens: config.generationParams.maxTokens,
    };

    try {
      // Отправляем запрос к HTTP-совместимому AI API
      const response = await fetch(config.endpoint, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'Authorization': `Bearer ${config.apiKey || ''}`,
        },
        body: JSON.stringify(requestBody),
      });

      // Проверяем статус ответа
      if (!response.ok) {
        throw new Error(`AI API returned ${response.status}: ${response.statusText}`);
      }

      // Парсим ответ
      const data = await response.json();

      // Извлекаем ответ из данных (предполагаем формат OpenAI)
      const content = data.choices?.[0]?.message?.content || '';
      const generationTime = data.usage?.total_time || 0;

      // Возвращаем ответ в унифицированном формате
      return {
        content: content,
        generationTime: generationTime,
      };
    } catch (error) {
      // Обрабатываем ошибки
      if (error instanceof Error) {
        throw new Error(`Failed to send message to AI API: ${error.message}`);
      } else {
        throw new Error('Failed to send message to AI API: Unknown error');
      }
    }
  }
}