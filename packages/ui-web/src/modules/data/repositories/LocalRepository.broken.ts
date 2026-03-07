import type { IProjectRepository } from './IProjectRepository'
import type { IFileRepository } from './IFileRepository'
import type { IAiSessionRepository } from './IAiSessionRepository'
import type { ISyncRepository } from './ISyncRepository'
import type { Project, ProjectScene } from '@shared/types/project.types'
import type { AiSession } from '@shared/types/ai.types'
import type { SyncOperation, SyncStatus } from '@shared/types/sync.types'

// Мock реализация локального репозитория
export class LocalProjectRepository implements IProjectRepository {
  async getAll(): Promise<Project[]> {
    const projects = localStorage.getItem('projects')
    return projects ? JSON.parse(projects) : []
  }

  async getById(id: string): Promise<Project | null> {
    const projects = await this.getAll()
    return projects.find(p => p.id === id) || null
  }

  async create(project: Omit<Project, 'id' | 'createdAt' | 'updatedAt'>): Promise<Project> {
    const newProject: Project = {
      ...project,
      id: proj_,
      createdAt: new Date(),
      updatedAt: new Date()
    }
    
    const projects = await this.getAll()
    projects.push(newProject)
    localStorage.setItem('projects', JSON.stringify(projects))
    
    return newProject
  }

  async update(id: string, updates: Partial<Project>): Promise<Project | null> {
    const projects = await this.getAll()
    const index = projects.findIndex(p => p.id === id)
    
    if (index === -1) return null
    
    projects[index] = {
      ...projects[index],
      ...updates,
      updatedAt: new Date()
    }
    
    localStorage.setItem('projects', JSON.stringify(projects))
    return projects[index]
  }

  async delete(id: string): Promise<boolean> {
    const projects = await this.getAll()
    const index = projects.findIndex(p => p.id === id)
    
    if (index === -1) return false
    
    projects.splice(index, 1)
    localStorage.setItem('projects', JSON.stringify(projects))
    return true
  }

  async getScene(projectId: string): Promise<ProjectScene | null> {
    const scenes = localStorage.getItem('scenes')
    const projectScenes = scenes ? JSON.parse(scenes) : {}
    return projectScenes[projectId] || null
  }

  async updateScene(projectId: string, scene: ProjectScene): Promise<ProjectScene> {
    const scenes = localStorage.getItem('scenes')
    const projectScenes = scenes ? JSON.parse(scenes) : {}
    projectScenes[projectId] = scene
    localStorage.setItem('scenes', JSON.stringify(projectScenes))
    return scene
  }
}

export class LocalFileRepository implements IFileRepository {
  async upload(file: File, projectId: string): Promise<string> {
    // В реальной реализации файлы будут сохраняться в IndexedDB или файловой системе
    const fileId = ile___
    console.log(File uploaded: )
    return fileId
  }

  async download(fileId: string): Promise<Blob> {
    // Mock implementation
    return new Blob([], { type: 'application/octet-stream' })
  }

  async delete(fileId: string): Promise<boolean> {
    console.log(File deleted: )
    return true
  }

  async getUrl(fileId: string): Promise<string> {
    return local://
  }
}

export class LocalAiSessionRepository implements IAiSessionRepository {
  async getAll(): Promise<AiSession[]> {
    const sessions = localStorage.getItem('ai-sessions')
    return sessions ? JSON.parse(sessions) : []
  }

  async getByProjectId(projectId: string): Promise<AiSession[]> {
    const sessions = await this.getAll()
    return sessions.filter(s => s.projectId === projectId)
  }

  async getById(id: string): Promise<AiSession | null> {
    const sessions = await this.getAll()
    return sessions.find(s => s.id === id) || null
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

  async delete(id: string): Promise<boolean> {
    const sessions = await this.getAll()
    const index = sessions.findIndex(s => s.id === id)
    
    if (index === -1) return false
    
    sessions.splice(index, 1)
    localStorage.setItem('ai-sessions', JSON.stringify(sessions))
    return true
  }
}

export class LocalSyncRepository implements ISyncRepository {
  async queueOperation(operation: Omit<SyncOperation, 'id' | 'timestamp' | 'deviceId'>): Promise<void> {
    const operations = localStorage.getItem('sync-operations')
    const pendingOps: SyncOperation[] = operations ? JSON.parse(operations) : []
    
    const newOp: SyncOperation = {
      ...operation,
      id: op_,
      timestamp: new Date(),
      deviceId: this.getDeviceId()
    }
    
    pendingOps.push(newOp)
    localStorage.setItem('sync-operations', JSON.stringify(pendingOps))
  }

  async getPendingOperations(): Promise<SyncOperation[]> {
    const operations = localStorage.getItem('sync-operations')
    return operations ? JSON.parse(operations) : []
  }

  async markAsSynced(operationIds: string[]): Promise<void> {
    const operations = localStorage.getItem('sync-operations')
    if (!operations) return
    
    const pendingOps: SyncOperation[] = JSON.parse(operations)
    const remainingOps = pendingOps.filter(op => !operationIds.includes(op.id))
    localStorage.setItem('sync-operations', JSON.stringify(remainingOps))
  }

  async getConflicts(): Promise<any[]> {
    const conflicts = localStorage.getItem('sync-conflicts')
    return conflicts ? JSON.parse(conflicts) : []
  }

  async resolveConflict(conflictId: string, resolution: 'local' | 'remote' | 'merge'): Promise<void> {
    const conflicts = localStorage.getItem('sync-conflicts')
    if (!conflicts) return
    
    const conflictList = JSON.parse(conflicts)
    const updatedConflicts = conflictList.map((c: any) => 
      c.id === conflictId ? { ...c, resolved: true, resolution } : c
    )
    
    localStorage.setItem('sync-conflicts', JSON.stringify(updatedConflicts))
  }

  async getStatus(): Promise<SyncStatus> {
    const status = localStorage.getItem('sync-status')
    return status ? JSON.parse(status) : {
      isOnline: navigator.onLine,
      lastSync: null,
      pendingOperations: 0,
      conflicts: []
    }
  }

  async updateStatus(status: Partial<SyncStatus>): Promise<void> {
    const currentStatus = await this.getStatus()
    const newStatus = { ...currentStatus, ...status }
    localStorage.setItem('sync-status', JSON.stringify(newStatus))
  }

  private getDeviceId(): string {
    let deviceId = localStorage.getItem('device-id')
    if (!deviceId) {
      deviceId = device_
      localStorage.setItem('device-id', deviceId)
    }
    return deviceId
  }
}
