/**
 * Общий формат конфигурации AI-бэкенда
 */

/**
 * Тип AI-бэкенда
 */
export type AiBackendType = 'ollama' | 'openai' | 'custom';

/**
 * Параметры генерации текста
 */
export interface GenerationParams {
  /**
   * Температура генерации (0.0 - 1.0)
   */
  temperature: number;
  
  /**
   * Параметр top-p для языковых моделей (0.0 - 1.0)
   */
  topP: number;
  
  /**
   * Максимальное количество токенов в ответе
   */
  maxTokens: number;
}

/**
 * Конфигурация AI-бэкенда
 */
export interface AiBackendConfig {
  /**
   * Тип AI-бэкенда
   */
  backendType: AiBackendType;
  
  /**
   * URL endpoint'а AI-бэкенда
   */
  endpoint: string;
  
  /**
   * API-ключ (если требуется)
   */
  apiKey?: string;
  
  /**
   * Имя модели
   */
  model: string;
  
  /**
   * Параметры генерации
   */
  generationParams: GenerationParams;
}