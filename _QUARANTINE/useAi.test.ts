/**
 * Тесты для useAi composable.
 */

import { describe, it, expect, vi, beforeEach } from 'vitest'

// Mock Platform Bridge
const mockInvoke = vi.fn()
vi.mock('@pepakura/shared/composables/usePlatform', () => ({
  usePlatform: () => ({
    invoke: mockInvoke
  })
}))

import { useAi } from '../useAi'

describe('useAi', () => {
  let ai: ReturnType<typeof useAi>

  beforeEach(() => {
    vi.clearAllMocks()
    ai = useAi()
  })

  describe('checkStatus', () => {
    it('должен возвращать статус AI', async () => {
      const mockStatus = {
        available: true,
        models: ['llama3.2'],
        provider: 'Ollama'
      }
      
      mockInvoke.mockResolvedValue(mockStatus)
      
      const status = await ai.checkStatus()
      
      expect(mockInvoke).toHaveBeenCalledWith('ai_check_status')
      expect(status).toEqual(mockStatus)
    })

    it('должен обрабатывать ошибку при проверке статуса', async () => {
      mockInvoke.mockRejectedValue(new Error('AI not available'))
      
      await expect(ai.checkStatus()).rejects.toThrow('AI not available')
    })
  })

  describe('getUnfoldAdvice', () => {
    it('должен получать рекомендации по развёртке', async () => {
      const mockAdvice = {
        algorithm: 'MDS',
        max_iterations: 100,
        tolerance: 0.001,
        tips: ['Используйте качественный клей'],
        potential_issues: ['Возможны наложения']
      }
      
      const mockMesh = { vertices: [], faces: [] }
      mockInvoke.mockResolvedValue(mockAdvice)
      
      const advice = await ai.getUnfoldAdvice(mockMesh as any)
      
      expect(mockInvoke).toHaveBeenCalledWith('ai_get_unfold_advice', { mesh: mockMesh })
      expect(advice).toEqual(mockAdvice)
    })
  })

  describe('generateInstructions', () => {
    it('должен генерировать инструкцию по сборке', async () => {
      const mockInstructions = {
        model_name: 'Cube',
        difficulty: 'Easy',
        total_time_minutes: 30,
        steps: [
          { step_number: 1, description: 'Вырежьте детали', part_ids: [1, 2, 3], estimated_time_minutes: 5 }
        ],
        tips: ['Дайте клею высохнуть']
      }
      
      const mockMesh = { vertices: [], faces: [] }
      mockInvoke.mockResolvedValue(mockInstructions)
      
      const instructions = await ai.generateInstructions(mockMesh as any)
      
      expect(mockInvoke).toHaveBeenCalledWith('ai_generate_instructions', { mesh: mockMesh })
      expect(instructions).toEqual(mockInstructions)
    })
  })

  describe('chat', () => {
    it('должен отправлять сообщение в чат', async () => {
      const mockResponse = 'Привет! Чем могу помочь?'
      
      mockInvoke.mockResolvedValue(mockResponse)
      
      const response = await ai.chat('Привет!')
      
      expect(mockInvoke).toHaveBeenCalledWith('ai_chat', {
        message: 'Привет!',
        history: []
      })
      expect(response).toBe(mockResponse)
    })

    it('должен отправлять сообщение с историей', async () => {
      const history = [
        { role: 'user' as const, content: 'Привет!' },
        { role: 'assistant' as const, content: 'Здравствуйте!' }
      ]
      
      mockInvoke.mockResolvedValue('Ответ')
      
      await ai.chat('Вопрос', history)
      
      expect(mockInvoke).toHaveBeenCalledWith('ai_chat', {
        message: 'Вопрос',
        history
      })
    })
  })

  describe('updateConfig', () => {
    it('должен обновлять конфигурацию AI', async () => {
      const newConfig = {
        provider: 'Ollama' as const,
        ollama_url: 'http://localhost:11434',
        model: 'mistral',
        temperature: 0.8,
        max_tokens: 2048,
        timeout_sec: 60
      }
      
      mockInvoke.mockResolvedValue(undefined)
      
      await ai.updateConfig(newConfig)
      
      expect(mockInvoke).toHaveBeenCalledWith('ai_update_config', { config: newConfig })
    })
  })

  describe('getConfig', () => {
    it('должен получать текущую конфигурацию', async () => {
      const mockConfig = {
        provider: 'Ollama' as const,
        ollama_url: 'http://localhost:11434',
        model: 'llama3.2',
        temperature: 0.7,
        max_tokens: 2048,
        timeout_sec: 60
      }
      
      mockInvoke.mockResolvedValue(mockConfig)
      
      const config = await ai.getConfig()
      
      expect(mockInvoke).toHaveBeenCalledWith('ai_get_config')
      expect(config).toEqual(mockConfig)
    })
  })

  describe('recommendPaper', () => {
    it('должен рекомендовать бумагу', async () => {
      const mockRecommendation = 'Используйте бумагу 160 g/m²'
      
      mockInvoke.mockResolvedValue(mockRecommendation)
      
      const recommendation = await ai.recommendPaper('Cube', 1.0)
      
      expect(mockInvoke).toHaveBeenCalledWith('ai_recommend_paper', {
        modelName: 'Cube',
        scale: 1.0
      })
      expect(recommendation).toBe(mockRecommendation)
    })
  })
})
