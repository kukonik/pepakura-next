/**
 * Common types for PlatformBridge.
 */

export type BridgeResult<T> = 
  | { success: true; data: T }
  | { success: false; error: BridgeError };

export interface BridgeError {
  code: string;
  message: string;
  context?: unknown;
}

// Placeholder types for core data structures.
// These should be refined as the core model evolves.
export interface ModelData {
  // TODO: define actual model structure
  vertices: number[];
  faces: number[];
  normals?: number[];
  textures?: number[];
  metadata?: Record<string, unknown>;
}

export interface MeshData {
  // TODO: define mesh structure
  vertices: number[];
  indices: number[];
  normals?: number[];
  uvs?: number[];
}

export interface UnfoldConfig {
  // TODO: define unfolding configuration
  glueTabs?: boolean;
  tabWidth?: number;
  margin?: number;
  scale?: number;
}

export interface UnfoldResult {
  vertices_2d: Array<number>;
  faces: Array<[number]>;
  metadata: {
    algorithm: string;
    unfold_time_ms: number;
    iterations: number;
    convergence: string | null;
  };
  error_msg: string | null;
}

// Platform type
export type Platform = 'desktop' | 'web';