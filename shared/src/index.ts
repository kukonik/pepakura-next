/**
 * Точка входа для экспорта общих типов и утилит
 */

// Экспорт типов AI-бэкенда
export type { AiBackendType, GenerationParams, AiBackendConfig } from './ai/AiBackendConfig';

// Экспорт типов швов
export type { Point3D, Edge, SeamSet } from './models/SeamTypes';

// Экспорт утилит валидации
export { isValidPoint3D, isValidEdge, isValidSeamSet } from './utils/validation';