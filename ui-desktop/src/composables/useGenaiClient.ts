import { ref } from 'vue'
import axios from 'axios'

export type GenaiTextParams = {
  max_tokens: number
  temperature: number
  top_p: number
  top_k: number
  seed?: number | null
}

export type GenaiChatTurn = {
  role: 'User' | 'Assistant'
  content: string
}

export type GenaiTextRequest = {
  model_id: string
  preset_id: string
  prompt: string
  system_prompt?: string | null
  history: GenaiChatTurn[]
  params: GenaiTextParams
}

export type GenaiTextResponse = {
  output: string
  history: GenaiChatTurn[]
}

export function useGenaiClient(baseUrl = 'http://127.0.0.1:8080') {
  const loading = ref(false)
  const error = ref<string | null>(null)

  const textGenerate = async (payload: GenaiTextRequest) => {
    loading.value = true
    error.value = null
    try {
      const res = await axios.post<GenaiTextResponse>(
        `${baseUrl}/api/v1/text/generate`,
        payload,
      )
      return res.data
    } catch (e: any) {
      error.value = e?.message ?? 'GenAI request failed'
      throw e
    } finally {
      loading.value = false
    }
  }

  return {
    loading,
    error,
    textGenerate,
  }
}
