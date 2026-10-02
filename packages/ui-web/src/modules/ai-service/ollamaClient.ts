/**
 * Клиент для работы с Ollama API
 */

import type { AiBackendConfig, ChatMessage, AiResponse } from '@pepakura-next/shared';

/**
 * Клиент для работы с Ollama API
 */
export class OllamaClient {
  /**
   * Отправляет сообщение в Ollama и возвращает ответ
   * @param messages Массив сообщений чата
   * @param config Конфигурация AI бэкенда
   * @returns Ответ от Ollama
   */
  async sendMessage(messages: ChatMessage[], config: AiBackendConfig): Promise<AiResponse> {
    // Формируем промпт для Ollama (простое объединение сообщений)
    const prompt = messages.map(msg => msg.content).join('\n\n');

    // Создаем тело запроса
    const requestBody = {
      model: config.model,
      prompt: prompt,
      stream: false,
      options: {
        temperature: config.generationParams.temperature,
        top_p: config.generationParams.topP,
        num_predict: config.generationParams.maxTokens,
      },
    };

    try {
      // Отправляем запрос к Ollama API
      const response = await fetch(config.endpoint, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(requestBody),
      });

      // Проверяем статус ответа
      if (!response.ok) {
        throw new Error(`Ollama API returned ${response.status}: ${response.statusText}`);
      }

      // Парсим ответ
      const data = await response.json();

      // Возвращаем ответ в унифицированном формате
      return {
        content: data.response || '',
        generationTime: data.total_duration ? Math.floor(data.total_duration / 1000000) : 0, // Преобразуем наносекунды в миллисекунды
      };
    } catch (error) {
      // Обрабатываем ошибки
      if (error instanceof Error) {
        throw new Error(`Failed to send message to Ollama: ${error.message}`);
      } else {
        throw new Error('Failed to send message to Ollama: Unknown error');
      }
    }
  }
}