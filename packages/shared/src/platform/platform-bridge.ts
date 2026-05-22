/**
 * Platform Bridge - абстрактный слой для платформонезависимого кода
 *
 * Этот модуль предоставляет единый API для работы с платформенными функциями:
 * - Tauri (Desktop)
 * - Web (Browser + WASM)
 *
 * @module platform-bridge
 */

// ============================================================================
// Типы и интерфейсы
// ============================================================================

/** Результат выполнения команды */
export interface CommandResult<T = unknown> {
  success: boolean;
  data?: T;
  error?: string;
}

/** Параметры проекта */
export interface ProjectData {
  id: string;
  name: string;
  description?: string;
  createdAt: string;
  updatedAt: string;
  scene: unknown;
  settings: ProjectSettings;
}

/** Настройки проекта */
export interface ProjectSettings {
  paperFormat: string;
  marginMm: number;
  scale: number;
}

/** Данные для экспорта */
export interface ExportData {
  format: 'svg' | 'pdf' | 'dxf' | 'png';
  path: string;
  options?: Record<string, unknown>;
}

/** Интерфейс платформенного моста */
export interface IPlatformBridge {
  /** Название платформы */
  readonly platformName: string;

  /** Инициализация моста */
  initialize(): Promise<void>;

  /** Проверка доступности платформы */
  isAvailable(): boolean;

  // --------------------------------------------------------------------------
  // Проектные операции
  // --------------------------------------------------------------------------

  /** Загрузить проект */
  loadProject(path: string): Promise<CommandResult<ProjectData>>;

  /** Сохранить проект */
  saveProject(project: ProjectData, path: string): Promise<CommandResult<void>>;

  /** Создать новый проект */
  createProject(name: string): Promise<CommandResult<ProjectData>>;

  // --------------------------------------------------------------------------
  // Операции с файлами
  // --------------------------------------------------------------------------

  /** Открыть диалог выбора файла */
  openFileDialog(options?: OpenFileDialogOptions): Promise<string | null>;

  /** Открыть диалог выбора директории */
  openDirectoryDialog(options?: OpenDirectoryDialogOptions): Promise<string | null>;

  /** Открыть диалог сохранения файла */
  saveFileDialog(options?: SaveFileDialogOptions): Promise<string | null>;

  /** Прочитать файл */
  readFile(path: string): Promise<CommandResult<Uint8Array>>;

  /** Записать файл */
  writeFile(path: string, data: Uint8Array): Promise<CommandResult<void>>;

  // --------------------------------------------------------------------------
  // 3D операции
  // --------------------------------------------------------------------------

  /** Импорт 3D модели */
  import3DModel(path: string): Promise<CommandResult<unknown>>;

  /** Развёртка 3D модели */
  unfoldModel(modelId: string, config?: UnfoldConfig): Promise<CommandResult<unknown>>;

  /** Экспорт развёртки */
  exportUnfold(data: ExportData): Promise<CommandResult<string>>;

  // --------------------------------------------------------------------------
  // AI операции
  // --------------------------------------------------------------------------

  /** Генерация 3D модели из изображения */
  generateFromImage(imagePath: string): Promise<CommandResult<string>>;

  /** Генерация 3D модели из текста */
  generateFromText(prompt: string): Promise<CommandResult<string>>;

  // --------------------------------------------------------------------------
  // Утилиты
  // --------------------------------------------------------------------------

  /** Вызвать команду платформы */
  invoke<T>(command: string, args?: Record<string, unknown>): Promise<T>;
}

// ============================================================================
// Опции диалогов
// ============================================================================

export interface OpenFileDialogOptions {
  title?: string;
  filters?: FileFilter[];
  multiple?: boolean;
  defaultPath?: string;
}

export interface OpenDirectoryDialogOptions {
  title?: string;
  defaultPath?: string;
}

export interface SaveFileDialogOptions {
  title?: string;
  filters?: FileFilter[];
  defaultPath?: string;
}

export interface FileFilter {
  name: string;
  extensions: string[];
}

export interface UnfoldConfig {
  algorithm?: 'mds' | 'lscm';
  preserveDetail?: boolean;
  maxIterations?: number;
  tolerance?: number;
}

// ============================================================================
// Tauri Bridge (Desktop)
// ============================================================================

/** Tauri реализация Platform Bridge */
export class TauriBridge implements IPlatformBridge {
  readonly platformName = 'tauri-desktop';

  private isInitialized = false;

  async initialize(): Promise<void> {
    if (this.isInitialized) return;

    // Проверка наличия Tauri API
    if (!this.isAvailable()) {
      throw new Error('Tauri API not available');
    }

    this.isInitialized = true;
    console.log('[TauriBridge] Initialized');
  }

  isAvailable(): boolean {
    return typeof window !== 'undefined' && '__TAURI__' in window;
  }

  async loadProject(path: string): Promise<CommandResult<ProjectData>> {
    try {
      const data = await this.invoke<ProjectData>('load_project', { path });
      return { success: true, data };
    } catch (error) {
      return {
        success: false,
        error: error instanceof Error ? error.message : 'Unknown error',
      };
    }
  }

  async saveProject(project: ProjectData, path: string): Promise<CommandResult<void>> {
    try {
      await this.invoke<void>('save_project', { project, path });
      return { success: true };
    } catch (error) {
      return {
        success: false,
        error: error instanceof Error ? error.message : 'Unknown error',
      };
    }
  }

  async createProject(name: string): Promise<CommandResult<ProjectData>> {
    try {
      // Пока создаём пустой проект на клиенте
      const project: ProjectData = {
        id: crypto.randomUUID(),
        name,
        createdAt: new Date().toISOString(),
        updatedAt: new Date().toISOString(),
        scene: {},
        settings: {
          paperFormat: 'A4',
          marginMm: 5,
          scale: 1,
        },
      };
      return { success: true, data: project };
    } catch (error) {
      return {
        success: false,
        error: error instanceof Error ? error.message : 'Unknown error',
      };
    }
  }

  async openFileDialog(options?: OpenFileDialogOptions): Promise<string | null> {
    try {
      const { open } = await import('@tauri-apps/plugin-dialog');
      const result = await open({
        title: options?.title,
        filters: options?.filters,
        multiple: options?.multiple,
        defaultPath: options?.defaultPath,
      });
      return Array.isArray(result) ? result[0] : result;
    } catch (error) {
      console.error('[TauriBridge] openFileDialog error:', error);
      return null;
    }
  }

  async openDirectoryDialog(options?: OpenDirectoryDialogOptions): Promise<string | null> {
    try {
      const { open } = await import('@tauri-apps/plugin-dialog');
      const result = await open({
        title: options?.title,
        directory: true,
        defaultPath: options?.defaultPath,
      });
      return Array.isArray(result) ? result[0] : result;
    } catch (error) {
      console.error('[TauriBridge] openDirectoryDialog error:', error);
      return null;
    }
  }

  async saveFileDialog(options?: SaveFileDialogOptions): Promise<string | null> {
    try {
      const { save } = await import('@tauri-apps/plugin-dialog');
      return await save({
        title: options?.title,
        filters: options?.filters,
        defaultPath: options?.defaultPath,
      });
    } catch (error) {
      console.error('[TauriBridge] saveFileDialog error:', error);
      return null;
    }
  }

  async readFile(path: string): Promise<CommandResult<Uint8Array>> {
    try {
      const { readBinaryFile } = await import('@tauri-apps/plugin-fs');
      const data = await readBinaryFile(path);
      return { success: true, data: new Uint8Array(data) };
    } catch (error) {
      return {
        success: false,
        error: error instanceof Error ? error.message : 'Unknown error',
      };
    }
  }

  async writeFile(path: string, data: Uint8Array): Promise<CommandResult<void>> {
    try {
      const { writeBinaryFile } = await import('@tauri-apps/plugin-fs');
      await writeBinaryFile(path, data);
      return { success: true };
    } catch (error) {
      return {
        success: false,
        error: error instanceof Error ? error.message : 'Unknown error',
      };
    }
  }

  async import3DModel(path: string): Promise<CommandResult<unknown>> {
    try {
      const result = await this.invoke<unknown>('import_3d_model', { path });
      return { success: true, data: result };
    } catch (error) {
      return {
        success: false,
        error: error instanceof Error ? error.message : 'Unknown error',
      };
    }
  }

  async unfoldModel(modelId: string, config?: UnfoldConfig): Promise<CommandResult<unknown>> {
    try {
      const result = await this.invoke<unknown>('unfold_3d_model', { modelId, config });
      return { success: true, data: result };
    } catch (error) {
      return {
        success: false,
        error: error instanceof Error ? error.message : 'Unknown error',
      };
    }
  }

  async exportUnfold(data: ExportData): Promise<CommandResult<string>> {
    try {
      const command = `export_nest_result_to_svg`;
      const result = await this.invoke<string>(command, { data });
      return { success: true, data: result };
    } catch (error) {
      return {
        success: false,
        error: error instanceof Error ? error.message : 'Unknown error',
      };
    }
  }

  async generateFromImage(imagePath: string): Promise<CommandResult<string>> {
    try {
      const result = await this.invoke<string>('ai_generate_from_image', { imagePath });
      return { success: true, data: result };
    } catch (error) {
      return {
        success: false,
        error: error instanceof Error ? error.message : 'Unknown error',
      };
    }
  }

  async generateFromText(prompt: string): Promise<CommandResult<string>> {
    try {
      const result = await this.invoke<string>('ai_generate_from_text', { prompt });
      return { success: true, data: result };
    } catch (error) {
      return {
        success: false,
        error: error instanceof Error ? error.message : 'Unknown error',
      };
    }
  }

  async invoke<T>(command: string, args?: Record<string, unknown>): Promise<T> {
    const { invoke } = await import('@tauri-apps/api/core');
    return invoke<T>(command, args);
  }
}

// ============================================================================
// Web Bridge (Browser + WASM)
// ============================================================================

/** Web реализация Platform Bridge */
export class WebBridge implements IPlatformBridge {
  readonly platformName = 'web-browser';

  private isInitialized = false;
  private wasmModule: unknown = null;

  async initialize(): Promise<void> {
    if (this.isInitialized) return;

    // Загрузка WASM модуля если доступен
    try {
      // @ts-ignore - динамическая загрузка WASM
      const wasm = await import('../wasm/pepakura_wasm_bg.js');
      this.wasmModule = wasm;
      console.log('[WebBridge] WASM module loaded');
    } catch (error) {
      console.warn('[WebBridge] WASM module not available');
    }

    this.isInitialized = true;
    console.log('[WebBridge] Initialized');
  }

  isAvailable(): boolean {
    return typeof window !== 'undefined';
  }

  async loadProject(path: string): Promise<CommandResult<ProjectData>> {
    // В web-среде загружаем через File API
    try {
      const fileInput = document.createElement('input');
      fileInput.type = 'file';
      fileInput.accept = '.json,.pepa';

      return new Promise((resolve) => {
        fileInput.onchange = async (e) => {
          const file = (e.target as HTMLInputElement).files?.[0];
          if (!file) {
            resolve({ success: false, error: 'No file selected' });
            return;
          }

          try {
            const text = await file.text();
            const project = JSON.parse(text) as ProjectData;
            resolve({ success: true, data: project });
          } catch (error) {
            resolve({
              success: false,
              error: error instanceof Error ? error.message : 'Parse error',
            });
          }
        };

        fileInput.click();
      });
    } catch (error) {
      return {
        success: false,
        error: error instanceof Error ? error.message : 'Unknown error',
      };
    }
  }

  async saveProject(project: ProjectData, path: string): Promise<CommandResult<void>> {
    // В web-среде скачиваем файл
    try {
      const blob = new Blob([JSON.stringify(project, null, 2)], {
        type: 'application/json',
      });
      const url = URL.createObjectURL(blob);
      const a = document.createElement('a');
      a.href = url;
      a.download = path || `${project.name}.json`;
      a.click();
      URL.revokeObjectURL(url);

      return { success: true };
    } catch (error) {
      return {
        success: false,
        error: error instanceof Error ? error.message : 'Unknown error',
      };
    }
  }

  async createProject(name: string): Promise<CommandResult<ProjectData>> {
    try {
      const project: ProjectData = {
        id: crypto.randomUUID(),
        name,
        createdAt: new Date().toISOString(),
        updatedAt: new Date().toISOString(),
        scene: {},
        settings: {
          paperFormat: 'A4',
          marginMm: 5,
          scale: 1,
        },
      };
      return { success: true, data: project };
    } catch (error) {
      return {
        success: false,
        error: error instanceof Error ? error.message : 'Unknown error',
      };
    }
  }

  async openFileDialog(options?: OpenFileDialogOptions): Promise<string | null> {
    return new Promise((resolve) => {
      const fileInput = document.createElement('input');
      fileInput.type = 'file';

      if (options?.filters?.length) {
        const accept = options.filters
          .flatMap((f) => f.extensions.map((ext) => `.${ext}`))
          .join(',');
        fileInput.accept = accept;
      }

      fileInput.onchange = (e) => {
        const file = (e.target as HTMLInputElement).files?.[0];
        resolve(file?.name ?? null);
      };

      fileInput.click();
    });
  }

  async openDirectoryDialog(): Promise<string | null> {
    console.warn('[WebBridge] openDirectoryDialog not supported in browser');
    return null;
  }

  async saveFileDialog(options?: SaveFileDialogOptions): Promise<string | null> {
    return Promise.resolve(options?.defaultPath ?? 'export.json');
  }

  async readFile(path: string): Promise<CommandResult<Uint8Array>> {
    return {
      success: false,
      error: 'readFile not directly supported in browser. Use openFileDialog instead.',
    };
  }

  async writeFile(path: string, data: Uint8Array): Promise<CommandResult<void>> {
    // Скачивание файла в браузере
    try {
      const blob = new Blob([data], { type: 'application/octet-stream' });
      const url = URL.createObjectURL(blob);
      const a = document.createElement('a');
      a.href = url;
      a.download = path.split('/').pop() ?? 'file';
      a.click();
      URL.revokeObjectURL(url);
      return { success: true };
    } catch (error) {
      return {
        success: false,
        error: error instanceof Error ? error.message : 'Unknown error',
      };
    }
  }

  async import3DModel(path: string): Promise<CommandResult<unknown>> {
    // В web-среде используем Three.js загрузчики
    return {
      success: false,
      error: 'import3DModel: Use WebGL/Three.js loaders directly',
    };
  }

  async unfoldModel(modelId: string, config?: UnfoldConfig): Promise<CommandResult<unknown>> {
    // Вызов WASM функции развёртки
    if (!this.wasmModule) {
      return {
        success: false,
        error: 'WASM module not loaded',
      };
    }

    try {
      // @ts-ignore - вызов WASM функции
      const result = this.wasmModule.unfold_mesh(modelId, config);
      return { success: true, data: result };
    } catch (error) {
      return {
        success: false,
        error: error instanceof Error ? error.message : 'Unfold error',
      };
    }
  }

  async exportUnfold(data: ExportData): Promise<CommandResult<string>> {
    // В web-среже генерируем SVG/PDF через JS библиотеки
    return {
      success: false,
      error: 'exportUnfold: Use web-specific export libraries',
    };
  }

  async generateFromImage(imagePath: string): Promise<CommandResult<string>> {
    // Вызов AI сервиса через HTTP API
    return {
      success: false,
      error: 'generateFromImage: Use HTTP API for AI services',
    };
  }

  async generateFromText(prompt: string): Promise<CommandResult<string>> {
    // Вызов AI сервиса через HTTP API
    return {
      success: false,
      error: 'generateFromText: Use HTTP API for AI services',
    };
  }

  async invoke<T>(command: string, args?: Record<string, unknown>): Promise<T> {
    throw new Error(`[WebBridge] invoke("${command}") not supported in web environment`);
  }
}

// ============================================================================
// Фабрика для создания моста
// ============================================================================

/** Определить тип текущей платформы */
export function detectPlatform(): 'tauri' | 'web' {
  if (typeof window !== 'undefined' && '__TAURI__' in window) {
    return 'tauri';
  }
  return 'web';
}

/** Создать Platform Bridge для текущей платформы */
export function createPlatformBridge(): IPlatformBridge {
  const platform = detectPlatform();

  console.log(`[PlatformBridge] Detected platform: ${platform}`);

  switch (platform) {
    case 'tauri':
      return new TauriBridge();
    case 'web':
      return new WebBridge();
    default:
      throw new Error(`Unknown platform: ${platform}`);
  }
}

/** Экземпляр моста по умолчанию */
export const platformBridge: IPlatformBridge = createPlatformBridge();
