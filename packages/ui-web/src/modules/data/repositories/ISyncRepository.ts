import type { SyncOperation, SyncStatus } from '@shared/types/sync.types'

export interface ISyncRepository {
  queueOperation(operation: Omit<SyncOperation, 'id' | 'timestamp' | 'deviceId'>): Promise<void>
  getPendingOperations(): Promise<SyncOperation[]>
  markAsSynced(operationIds: string[]): Promise<void>
  getConflicts(): Promise<any[]> // Conflict type to be defined
  resolveConflict(conflictId: string, resolution: 'local' | 'remote' | 'merge'): Promise<void>
  getStatus(): Promise<SyncStatus>
  updateStatus(status: Partial<SyncStatus>): Promise<void>
}
