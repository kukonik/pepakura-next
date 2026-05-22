// shared/src/types/nesting.ts

/**
 * Параметры бумаги
 */
export interface PaperSettings {
  /**
   * Формат бумаги (A4, A3, Letter и т.д.)
   */
  format: string;
  
  /**
   * Ширина бумаги в мм
   */
  width_mm: number;
  
  /**
   * Высота бумаги в мм
   */
  height_mm: number;
  
  /**
   * Отступ от края бумаги в мм
   */
  margin_mm: number;
}

/**
 * Результат размещения разверток
 */
export interface NestResult {
  /**
   * Список листов с размещенными развертками
   */
  sheets: NestSheet[];
  /**
   * Метрики качества размещения
   */
  metrics: NestMetrics;
  /**
   * Снимок параметров размещения
   */
  paramsSnapshot: NestParams;
}

/**
 * Метрики качества размещения
 */
export interface NestMetrics {
  /**
   * Общее количество листов
   */
  totalSheets: number;
  /**
   * Общее количество частей
   */
  totalParts: number;
  /**
   * Средняя заполненность листов (%)
   */
  avgFillRate: number;
  /**
   * Общая площадь всех частей (мм²)
   */
  totalPartsArea: number;
  /**
   * Общая площадь использованных листов (мм²)
   */
  totalSheetsArea: number;
}

/**
 * Лист с размещенными развертками
 */
export interface NestSheet {
  /**
   * Номер листа
   */
  id: number;
  
  /**
   * Индекс листа
   */
  index: number;
  
  /**
   * Ширина листа в мм
   */
  width_mm: number;
  
  /**
   * Высота листа в мм
   */
  height_mm: number;
  
  /**
   * Отступ от края листа в мм
   */
  margin_mm: number;
  
  /**
   * Размещенные части
   */
  parts: NestPart[];
}

/**
 * Часть развертки на листе
 */
export interface NestPart {
  /**
   * ID части
   */
  id: number;
  
  /**
   * Имя части
   */
  name?: string;
  
  /**
   * Позиция X на листе (в мм)
   */
  x_mm: number;
  
  /**
   * Позиция Y на листе (в мм)
   */
  y_mm: number;
  
  /**
   * Ширина части (в мм)
   */
  width_mm: number;
  
  /**
   * Высота части (в мм)
   */
  height_mm: number;
  
  /**
   * Угол поворота в градусах
   */
  rotation: number;
}

/**
 * Параметры размещения
 */
export interface NestParams {
  /**
   * Настройки бумаги
   */
  paper: PaperSettings;
  
  /**
   * Максимальное количество листов
   */
  max_sheets: number;
  
  /**
   * Масштаб
   */
  scale: number;
  
  /**
   * Шаг вращения в градусах
   */
  rotationStepDeg: number;
}

/**
 * Переопределение позиции и поворота части
 */
export interface PartOverride {
  /**
   * ID части
   */
  partId: number;
  
  /**
   * Изменение позиции X (в мм)
   */
  deltaX?: number;
  
  /**
   * Изменение позиции Y (в мм)
   */
  deltaY?: number;
  
  /**
   * Изменение угла поворота (в градусах)
   */
  deltaRotation?: number;
  
  /**
   * Флаг, указывающий, что часть была изменена пользователем
   */
  isManual?: boolean;
}