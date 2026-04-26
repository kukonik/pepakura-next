/**
 * Platform Bridge interface.
 * Abstracts core functionality for different platforms (Desktop/Tauri, Web/WASM).
 */
import type {
  BridgeResult,
  ModelData,
  MeshData,
  UnfoldConfig,
  UnfoldResult,
  Platform,
} from './types';
import type { PingResponse, ParseResult, ObjGeometry } from '../types/core';

export type {
  BridgeResult,
  BridgeError,
  ModelData,
  MeshData,
  UnfoldConfig,
  UnfoldResult,
  Platform,
} from './types';

export interface PlatformBridge {
  // Core operations with typed results
  loadModel(path: string): Promise<BridgeResult<ModelData>>;
  unfoldMesh(mesh: MeshData, config?: UnfoldConfig): Promise<BridgeResult<UnfoldResult>>;
  isReady(): boolean;

  // Legacy operations (to be migrated to BridgeResult style)
  exportSVG(model: unknown, options: unknown): Promise<string>;
  exportPDF(model: unknown, options: unknown): Promise<Uint8Array>;

  // File system operations (optional, platform-dependent)
  openFileDialog(options?: unknown): Promise<Uint8Array | null>;
  saveFileDialog(data: Uint8Array, suggestedName: string): Promise<boolean>;

  // Platform info
  getPlatform(): Platform;

  // Generic invoke for backward compatibility and extensibility
  invoke<T>(cmd: string, args?: unknown): Promise<T>;
  invokeWithResult<T>(cmd: string, args?: unknown): Promise<CoreResult<T>>;

  // Ping-pong test method
  pingPong(message: string): Promise<PingResponse>;

  // Parse mock OBJ string (for testing without filesystem)
  parseMockObj(objString: string): Promise<ParseResult>;

  // Production-level OBJ parser with validation and memory safety
  loadRealObj(objString: string): Promise<ObjGeometry>;

  // Async unfold pipeline with progress events
  startMockUnfold(totalFaces: number): Promise<string>;
  onUnfoldProgress(callback: (percent: number, msg: string) => void): Promise<() => void>;

  // LSCM unfolding algorithm
  unfold_lscm(meshJson: string, optionsJson: string): Promise<UnfoldResult>;
}

/**
 * Result of a core operation (legacy, prefer BridgeResult).
 * @deprecated Use BridgeResult instead.
 */
export interface CoreResult<T> {
  ok: boolean;
  data?: T;
  error?: string;
}