/**
 * Composable для работы с AI-помощником.
 * 
 * Предоставляет функции для:
 * - Проверки доступности AI
 * - Получения рекомендаций по развёртке
 * - Генерации инструкций сборки
 * - Чата с AI
 */

import { usePlatform } from '@pepakura/shared/composables/usePlatform'

export interface AiConfig {
  provider: 'Ollama' | 'OpenAI'
  ollama_url: string
  model: string
  temperature: number
  max_tokens: number
  timeout_sec: number
}

export interface AiStatus {
  available: boolean
  models: string[]
  provider: string
}

export interface UnfoldAdvice {
  algorithm: string
  max_iterations: number
  tolerance: number
  tips: string[]
  potential_issues: string[]
}

export interface AssemblyStep {
  step_number: number
  description: string
  part_ids: number[]
  estimated_time_minutes: number
}

export interface AssemblyInstruction {
  model_name: string
  difficulty: string
  total_time_minutes: number
  steps: AssemblyStep[]
  tips: string[]
}

export interface ChatMessage {
  role: 'user' | 'assistant'
  content: string
}

export function useAi() {
  const { invoke } = usePlatform()

  /**
   * Проверяет доступность AI.
   */
  async function checkStatus(): Promise<AiStatus> {
    return await invoke<AiStatus>('ai_check_status')
  }

  /**
   * Получает рекомендации по развёртке.
   * @param mesh - 3D-меш для анализа
   */
  async function getUnfoldAdvice(mesh: any): Promise<UnfoldAdvice> {
    return await invoke<UnfoldAdvice>('ai_get_unfold_advice', { mesh })
  }

  /**
   * Генерирует инструкцию по сборке.
   * @param mesh - 3D-меш для генерации
   */
  async function generateInstructions(mesh: any): Promise<AssemblyInstruction> {
    return await invoke<AssemblyInstruction>('ai_generate_instructions', { mesh })
  }

  /**
   * Отправляет сообщение в AI-чат.
   * @param message - Сообщение пользователю
   * @param history - История сообщений
   */
  async function chat(
    message: string,
    history: ChatMessage[] = []
  ): Promise<string> {
    return await invoke<string>('ai_chat', { message, history })
  }

  /**
   * Обновляет конфигурацию AI.
   * @param config - Новая конфигурация
   */
  async function updateConfig(config: AiConfig): Promise<void> {
    return await invoke('ai_update_config', { config })
  }

  /**
   * Получает текущую конфигурацию AI.
   */
  async function getConfig(): Promise<AiConfig> {
    return await invoke<AiConfig>('ai_get_config')
  }

  /**
   * Получает рекомендации по бумаге.
   * @param modelName - Название модели
   * @param scale - Масштаб
   */
  async function recommendPaper(
    modelName: string,
    scale: number
  ): Promise<string> {
    return await invoke<string>('ai_recommend_paper', { modelName, scale })
  }

  return {
    checkStatus,
    getUnfoldAdvice,
    generateInstructions,
    chat,
    updateConfig,
    getConfig,
    recommendPaper
  }
}