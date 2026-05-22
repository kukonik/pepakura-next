// Экспорт интерфейсов и типов
export type { PlatformBridge } from './PlatformBridge'
export type { BridgeResult, BridgeError } from './types'
export type { 
  ModelData, MeshData, UnfoldConfig, UnfoldResult, 
  ParseResult, ObjGeometry 
} from '../types/core'

// ✅ CoreResult — алиас для обратной совместимости с usePlatform.ts
export type CoreResult<T> = BridgeResult<T>

// Экспорт фабрик
export { createTauriBridge } from './TauriBridge'
export { createWasmBridge } from './WasmBridge'

// Экспорт синглтона
export { setBridge, getBridge, hasBridge } from './PlatformBridge'
