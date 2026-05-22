import type { BridgeResult, BridgeError } from './types'
import type { ModelData, MeshData, UnfoldConfig, UnfoldResult, ParseResult, ObjGeometry } from '../types/core'

export interface PlatformBridge {
  isReady(): boolean
  initialize?(): Promise<void>
  
  // Core методы
  loadModel(path: string): Promise<BridgeResult<ModelData>>
  unfoldMesh(mesh: MeshData, config?: UnfoldConfig): Promise<BridgeResult<UnfoldResult>>
  parseMockObj(objString: string): Promise<BridgeResult<ParseResult>>
  loadRealObj(objString: string): Promise<BridgeResult<ObjGeometry>>
  
  // AI методы
  generateFromText(prompt: string): Promise<BridgeResult<string>>
  generateFromImage(imagePath: string): Promise<BridgeResult<string>>
  
  // Экспорт методы
  exportToSvg(unfoldId: number): Promise<BridgeResult<string>>
  exportToPdf(unfoldId: number, format: string): Promise<BridgeResult<Uint8Array>>
  
  // Утилиты
  invoke<T>(command: string, args?: Record<string, unknown>): Promise<T>
  invokeWithResult<T>(command: string, args?: Record<string, unknown>): Promise<BridgeResult<T>>
}

export function createPlatformBridge(): PlatformBridge {
  throw new Error('PlatformBridge must be initialized via setBridge()')
}

let _instance: PlatformBridge | null = null

export function setBridge(bridge: PlatformBridge): void {
  _instance = bridge
}

export function getBridge(): PlatformBridge {
  if (!_instance) {
    throw new Error('PlatformBridge not initialized. Call setBridge() first.')
  }
  return _instance
}

export function hasBridge(): boolean {
  return _instance !== null
}
