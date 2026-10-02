export interface SyncOperation {
  id: string
  entityType: 'project' | 'scene' | 'file' | 'aiSession' | 'setting'
  entityId: string
  operation: 'create' | 'update' | 'delete'
  data: any
  timestamp: Date
  revision: number
  userId?: string
  deviceId: string
}

export interface SyncConflict {
  localOperation: SyncOperation
  remoteOperation: SyncOperation
  resolved: boolean
  resolution?: 'local' | 'remote' | 'merge'
}

export interface SyncStatus {
  isOnline: boolean
  lastSync: Date | null
  pendingOperations: number
  conflicts: SyncConflict[]
}
