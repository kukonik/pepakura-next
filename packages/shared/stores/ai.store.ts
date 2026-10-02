import { defineStore } from 'pinia'
import { createAiClient } from '../modules/ai-service/config'
import type { IAiClient } from '../modules/ai-service/client'
import type { 
  BaseAiMessage, 
  AiBackendConfig, 
  AiChatRequest,
  AiSession
} from '../types/ai.types'

// Mock реализация репозитория для демонстрации
class MockAiSessionRepository {
  async getAll(): Promise<AiSession[]> {
    const sessions = localStorage.getItem('ai-sessions')
    return sessions ? JSON.parse(sessions) : []
  }

  async create(session: Omit<AiSession, 'id' | 'createdAt' | 'updatedAt'>): Promise<AiSession> {
    const newSession: AiSession = {
      ...session,
      id: isess_,
      createdAt: new Date(),
      updatedAt: new Date()
    }
    
    const sessions = await this.getAll()
    sessions.push(newSession)
    localStorage.setItem('ai-sessions', JSON.stringify(sessions))
    
    return newSession
  }

  async update(id: string, updates: Partial<AiSession>): Promise<AiSession | null> {
    const sessions = await this.getAll()
    const index = sessions.findIndex(s => s.id === id)
    
    if (index === -1) return null
    
    sessions[index] = {
      ...sessions[index],
      ...updates,
      updatedAt: new Date()
    }
    
    localStorage.setItem('ai-sessions', JSON.stringify(sessions))
    return sessions[index]
  }
}

export interface AiState {
  // Конфигурация
  config: AiBackendConfig
  
  // Сессии
  sessions: AiSession[]
  currentSessionId: string | null
  
  // Статус
  isThinking: boolean
  error: string | null
  
  // Настройки UI
  isPanelVisible: boolean
  isPanelExpanded: boolean
}

export const useAiStore = defineStore('ai', {
  state: (): AiState => ({
    config: {
      type: 'ollama',
      endpoint: 'http://localhost:11434/api',
      model: 'gemma2:2b',
      temperature: 0.7,
      maxTokens: 2048
    },
    sessions: [],
    currentSessionId: null,
    isThinking: false,
    error: null,
    isPanelVisible: true,
    isPanelExpanded: true
  }),

  getters: {
    currentSession(state): AiSession | null {
      if (!state.currentSessionId) return null
      return state.sessions.find(s => s.id === state.currentSessionId) || null
    },
    
    sessionList(state): AiSession[] {
      return [...state.sessions]
        .sort((a, b) => b.updatedAt.getTime() - a.updatedAt.getTime())
    },
    
    canSendMessage(state): boolean {
      return !state.isThinking && !!state.currentSessionId
    }
  },

  actions: {
    // Конфигурация
    updateConfig(newConfig: Partial<AiBackendConfig>) {
      this.config = { ...this.config, ...newConfig }
      localStorage.setItem('ai-config', JSON.stringify(this.config))
    },
    
    loadConfig() {
      const saved = localStorage.getItem('ai-config')
      if (saved) {
        try {
          const parsed = JSON.parse(saved)
          this.config = { ...this.config, ...parsed }
        } catch (e) {
          console.warn('Failed to load AI config from localStorage:', e)
        }
      }
    },
    
    // Сессии
    async loadSessions() {
      try {
        const repo = new MockAiSessionRepository()
        this.sessions = await repo.getAll()
      } catch (error) {
        console.error('Failed to load AI sessions:', error)
      }
    },
    
    async createSession(title: string = 'Новая сессия', projectId?: string): Promise<string> {
      try {
        const repo = new MockAiSessionRepository()
        const session = await repo.create({ 
          title, 
          projectId,
          messages: [] 
        })
        
        this.sessions.push(session)
        this.currentSessionId = session.id
        return session.id
      } catch (error) {
        console.error('Failed to create AI session:', error)
        throw error
      }
    },
    
    async deleteSession(sessionId: string) {
      try {
        const repo = new MockAiSessionRepository()
        // В реальной реализации здесь будет удаление
        this.sessions = this.sessions.filter(s => s.id !== sessionId)
        if (this.currentSessionId === sessionId) {
          this.currentSessionId = null
        }
      } catch (error) {
        console.error('Failed to delete AI session:', error)
        throw error
      }
    },
    
    async switchSession(sessionId: string) {
      const session = this.sessions.find(s => s.id === sessionId)
      if (session) {
        this.currentSessionId = sessionId
      }
    },
    
    // Сообщения
    async addMessage(sessionId: string, message: Omit<BaseAiMessage, 'timestamp'>) {
      try {
        const session = this.sessions.find(s => s.id === sessionId)
        if (session) {
          session.messages.push({
            ...message,
            timestamp: new Date()
          })
          session.updatedAt = new Date()
          
          // Сохраняем в репозитории
          const repo = new MockAiSessionRepository()
          await repo.update(sessionId, { 
            messages: session.messages,
            updatedAt: session.updatedAt
          })
        }
      } catch (error) {
        console.error('Failed to add message:', error)
        throw error
      }
    },
    
    // AI взаимодействие
    async sendMessage(content: string, projectId?: string) {
      if (!this.currentSessionId) {
        await this.createSession('Сессия ассистента', projectId)
      }
      
      if (!this.currentSessionId) return
      
      // Добавляем сообщение пользователя
      await this.addMessage(this.currentSessionId, { role: 'user', content })
      
      // Начинаем обработку
      this.isThinking = true
      this.error = null
      
      try {
        const client: IAiClient = createAiClient(this.config)
        
        const currentSession = this.sessions.find(s => s.id === this.currentSessionId)
        if (!currentSession) throw new Error('Current session not found')
        
        const request: AiChatRequest = {
          messages: currentSession.messages,
          model: this.config.model,
          temperature: this.config.temperature,
          maxTokens: this.config.maxTokens
        }
        
        const response = await client.chat(request)
        
        // Добавляем ответ ассистента
        await this.addMessage(this.currentSessionId, response.message)
        
      } catch (error) {
        this.error = error instanceof Error ? error.message : 'Unknown error occurred'
        console.error('AI request failed:', error)
      } finally {
        this.isThinking = false
      }
    },
    
    // UI управление
    togglePanel() {
      this.isPanelVisible = !this.isPanelVisible
      if (this.isPanelVisible) {
        localStorage.setItem('ai-panel-visible', 'true')
      } else {
        localStorage.setItem('ai-panel-visible', 'false')
      }
    },
    
    togglePanelExpand() {
      this.isPanelExpanded = !this.isPanelExpanded
      localStorage.setItem('ai-panel-expanded', this.isPanelExpanded.toString())
    },
    
    loadUiState() {
      const visible = localStorage.getItem('ai-panel-visible')
      const expanded = localStorage.getItem('ai-panel-expanded')
      
      if (visible !== null) {
        this.isPanelVisible = visible === 'true'
      }
      
      if (expanded !== null) {
        this.isPanelExpanded = expanded === 'true'
      }
    },
    
    // Инициализация
    async initialize() {
      this.loadConfig()
      this.loadUiState()
      await this.loadSessions()
    }
  }
})
