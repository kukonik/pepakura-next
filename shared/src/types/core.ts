/**
 * Core types shared between Rust and TypeScript.
 */

export interface PingResponse {
  message: string;
  timestamp: number;
  success: boolean;
}

export interface ParseResult {
  success: boolean;
  vertices_count: number;
  faces_count: number;
  error_msg: string | null;
}

export interface ObjGeometry {
  positions: number[]
  indices: number[]
  face_count: number
  warnings: string[]
}

export interface UnfoldResult {
  vertices_2d: Array<number>; // Координаты [x, y] (f64, но в JSON serialize в f64 не поддерживается, используем number)
  faces: Array<[number]>;
  metadata: UnfoldMetadata;
  error_msg: string | null;
}

export interface UnfoldMetadata {
  algorithm: string;
  unfold_time_ms: number;
  iterations: number;
  convergence: string | null;
}

export interface SanitizeReport {
  original_faces: number;
  final_faces: number;
  time_ms: number;
}