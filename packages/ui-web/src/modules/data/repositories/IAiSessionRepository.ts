import type { AiSession } from '@shared/types/ai.types'

export interface IAiSessionRepository {
  getAll(): Promise<AiSession[]>
  getByProjectId(projectId: string): Promise<AiSession[]>
  getById(id: string): Promise<AiSession | null>
  create(session: Omit<AiSession, 'id' | 'createdAt' | 'updatedAt'>): Promise<AiSession>
  update(id: string, updates: Partial<AiSession>): Promise<AiSession | null>
  delete(id: string): Promise<boolean>
}
