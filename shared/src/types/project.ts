// shared/src/types/project.ts
import { PepaScene } from './model';
import { PaperSettings, NestParams, PartOverride } from './nesting';

// Метаданные проекта
export interface ProjectMeta {
  // Название проекта
  name: string;
  // Описание проекта
  description?: string;
  // Автор проекта
  author?: string;
  // Дата создания
  createdAt: string;
  // Дата последнего изменения
  updatedAt: string;
}

// Настройки проекта
export interface ProjectSettings {
  // Формат листа для печати
  paperFormat: string;
  // Ширина поля в мм
  marginMm: number;
  // Масштаб
  scale: number;
  // Настройки бумаги для размещения
  paperSettings?: PaperSettings;
  // Параметры размещения
  nestParams?: NestParams;
  // Дополнительные настройки (для расширений)
  extensions: Record<string, any>;
}

// Расширения проекта
export interface ProjectExtensions {
  // Переопределения позиций и поворотов частей
  partOverrides?: Record<number, PartOverride>;
  
  // Другие расширения
  [key: string]: any;
}

// Основная структура проекта
export interface PepaProject {
  // Версия схемы
  schemaVersion: string;
  // Метаданные проекта
  projectMeta: ProjectMeta;
  // Сцена проекта
  scene: PepaScene;
  // Настройки проекта
  settings: ProjectSettings;
  // Расширения проекта
  extensions: ProjectExtensions;
}