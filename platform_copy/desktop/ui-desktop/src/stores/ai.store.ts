/**
 * Pinia store для управления состоянием AI.
 */

import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import type { AiConfig, AiStatus } from '@/composables/useAi'

export const useAiStore = defineStore('ai', () => {
  // State
  const config = ref<AiConfig>({
    provider: 'Ollama',
    ollama_url: 'http://localhost:11434',
    model: 'llama3.2',
    temperature: 0.7,
    max_tokens: 2048,
    timeout_sec: 60
  })

  const status = ref<AiStatus | null>(null)
  const isLoading = ref(false)
  const error = ref<string | null>(null)

  // Getters
  const isAvailable = computed(() => status.value?.available ?? false)
  const models = computed(() => status.value?.models ?? [])
  const hasError = computed(() => error.value !== null)

  // Actions
  async function loadConfig() {
    try {
      const { getConfig } = await import('@/composables/useAi')
      config.value = await getConfig()
    } catch (e) {
      error.value = `Failed to load config: ${e}`
    }
  }

  async function checkStatus() {
    isLoading.value = true
    error.value = null
    
    try {
      const { checkStatus } = await import('@/composables/useAi')
      status.value = await checkStatus()
    } catch (e) {
      error.value = `Failed to check AI status: ${e}`
      status.value = null
    } finally {
      isLoading.value = false
    }
  }

  async function updateConfig(newConfig: Partial<AiConfig>) {
    try {
      const { updateConfig } = await import('@/composables/useAi')
      config.value = { ...config.value, ...newConfig }
      await updateConfig(config.value)
      
      // Перепроверяем статус после обновления конфига
      await checkStatus()
    } catch (e) {
      error.value = `Failed to update config: ${e}`
      throw e
    }
  }

  async function setProvider(provider: 'Ollama' | 'OpenAI') {
    await updateConfig({ provider })
  }

  async function setModel(model: string) {
    await updateConfig({ model })
  }

  async function setTemperature(temperature: number) {
    await updateConfig({ temperature: Math.max(0, Math.min(1, temperature)) })
  }

  function clearError() {
    error.value = null
  }

  function reset() {
    status.value = null
    error.value = null
    isLoading.value = false
    config.value = {
      provider: 'Ollama',
      ollama_url: 'http://localhost:11434',
      model: 'llama3.2',
      temperature: 0.7,
      max_tokens: 2048,
      timeout_sec: 60
    }
  }

  return {
    // State
    config,
    status,
    isLoading,
    error,
    
    // Getters
    isAvailable,
    models,
    hasError,
    
    // Actions
    loadConfig,
    checkStatus,
    updateConfig,
    setProvider,
    setModel,
    setTemperature,
    clearError,
    reset
  }
})
