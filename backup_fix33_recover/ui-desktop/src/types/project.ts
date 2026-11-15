// Расширенная структура проекта с поддержкой AI-генерации

export interface BaseProject {
  id: string
  name: string
  thumbnail?: string      // base64 или путь к превью
  lastModified: Date
  fileSize: number       // в МБ
  tags: string[]
  filePath?: string      // путь к исходному файлу .obj
  sheetCount: number     // количество листов
  modelType: string      // тип модели (человек, транспорт и т.д.)
}

export type GenerationSource = 'text' | 'image' | 'manual' | 'import'

export interface GeneratedModel {
  id: string
  projectId: string
  name: string
  sourceType: GenerationSource
  sourceData: string  // Текстовое описание или путь к изображению
  generatedAt: Date
  modelPath: string  // Путь к сгенерированной 3D модели
  previewPath?: string // Путь к превью изображению
  parameters: Record<string, any>  // Параметры генерации
}

export interface Project extends BaseProject {
  generatedModels: GeneratedModel[]
  currentModelId?: string  // ID текущей активной модели
}

export interface ProjectStats {
  total: number
  totalSize: number
  recentCount: number
  lastActive: Date | null
  avgSheetCount: number
  generatedModelsCount: number
}

export interface ProjectSettings {
  aiBackend: string
  aiModel: string
  exportFormat: string
  autoSaveEnabled: boolean
  autoSaveInterval: number  // в минутах
}