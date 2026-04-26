// TypeScript types mirroring Rust structures from pepakura_core

// Vertex
export interface Vertex {
  id: number;
  position: [number, number, number];
  normal?: [number, number, number];
  uv?: [number, number];
}

// Face
export interface Face {
  vertices: [number, number, number];
  materialId?: number;
}

// Mesh metadata
export interface MeshMetadata {
  author?: string;
  createdAt?: string;
  units: string; // "mm", "cm", "inch"
  boundingBox?: BoundingBox;
}

// Bounding box
export interface BoundingBox {
  min: [number, number, number];
  max: [number, number, number];
}

// Mesh
export interface Mesh {
  vertices: Vertex[];
  faces: Face[];
  name: string;
  metadata: MeshMetadata;
}

// Unfold configuration
export interface UnfoldConfig {
  preserveDetail: boolean;
  maxIterations: number;
  tolerance: number;
}

// Unfold metadata
export interface UnfoldMetadata {
  iterations: number;
  stress: number;
  elapsedMs: number;
  config: UnfoldConfig;
}

// Unfolded mesh
export interface UnfoldedMesh {
  vertices2d: [number, number][];
  faces: [number, number, number][]; // same indices as original mesh
  sourceMesh: Mesh; // reference to original mesh (clone)
  metadata: UnfoldMetadata;
}

// Page size for SVG export
export type PageSize = 'A4' | 'A3' | { custom: { width: number; height: number } };

// SVG export configuration
export interface SvgExportConfig {
  pageSize: PageSize;
  scale: number; // mm per unit
  showVertexIds: boolean;
  showFoldLines: boolean;
  showCutLines: boolean;
}

// Project ID (string UUID)
export type ProjectId = string;

// Project info
export interface ProjectInfo {
  id: ProjectId;
  name: string;
  createdAt: string;
  updatedAt: string;
  meshCount: number;
  thumbnail?: string;
}

// App settings
export interface AppSettings {
  language: 'ru' | 'en';
  theme: 'light' | 'dark' | 'system';
  defaultExportPath: string;
  unfoldConfig: UnfoldConfig;
  aiConfig: AiConfig;
}

// AI configuration (placeholder)
export interface AiConfig {
  provider: 'ollama' | 'openai' | 'none';
  model: string;
  apiKey?: string;
}

// Tauri command result wrapper
export type TauriResult<T> =
  | { ok: true; data: T }
  | { ok: false; error: string };