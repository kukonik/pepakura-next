// pepakura-next/ui-desktop/src/stores/projectStore.ts
import { defineStore } from 'pinia'
import { invoke } from '@tauri-apps/api/core'
import { PepaProject } from '@/../../shared/src/types/project'
import { NestParams, NestResult, PartOverride } from '@/../../shared/src/types/nesting'
import { writeTextFile } from '@tauri-apps/plugin-fs'
import type { ParsePdoResult } from '@/types/pdo'

export interface ProjectStoreState {
  // Текущий проект
  currentProject: PepaProject | null
  // Путь к файлу проекта
  projectPath: string | null
  // Статус загрузки проекта
  isLoading: boolean
  // Ошибка загрузки проекта
  loadError: string | null
  // Результат размещения
  nestResult: NestResult | null
  // Переопределения позиций и поворотов частей
  partOverrides: Record<number, PartOverride>
  // Флаг, указывающий, что проект был изменен
  isDirty: boolean
  // Статус размещения
  isNesting: boolean
  // Ошибка размещения
  nestError: string | null
  // Статус экспорта
  isExporting: boolean
  // Ошибка экспорта
  exportError: string | null
  // Ошибка конкретного листа
  exportSheetError: string | null
  // Прогресс экспорта
  exportProgress: number | null
  // Общее количество файлов для экспорта
  exportTotal: number | null
  // Индекс текущего экспортируемого листа
  exportSheetIndex: number | null
  // Общее количество листов для экспорта
  exportSheetTotal: number | null
  // Кэш SVG листов для предпросмотра
  sheetSvgCache: Record<number, string>
  // Статус загрузки SVG
  isSvgLoading: boolean
  // Функция автосохранения
  autoSaveProject: (() => Promise<void>) | null
}

export const useProjectStore = defineStore('projectStore', {
  state: (): ProjectStoreState => ({
    currentProject: null,
    projectPath: null,
    isLoading: false,
    loadError: null,
    nestResult: null,
    partOverrides: {},
    isDirty: false,
    isNesting: false,
    nestError: null,
    isExporting: false,
    exportError: null,
    exportSheetError: null,
    exportProgress: null,
    exportTotal: null,
    exportSheetIndex: null,
    exportSheetTotal: null,
    sheetSvgCache: {},
    isSvgLoading: false,
    autoSaveProject: null
  }),

  getters: {
    // Проверка, есть ли текущий проект
    hasProject: (state) => !!state.currentProject,
    
    // Получение имени проекта
    projectName: (state) => state.currentProject?.projectMeta.name ?? 'Новый проект',
    
    // Получение сцены проекта
    projectScene: (state) => state.currentProject?.scene ?? null
  },

  actions: {
    // Создание нового проекта
    createNewProject(name: string) {
      // TODO: Создать новую сцену
      const newScene = {
        sceneVersion: "1.0",
        meshes: [],
        materials: [],
        boundingBox: undefined
      }
      
      const now = new Date().toISOString()
      this.currentProject = {
        schemaVersion: "1.0",
        projectMeta: {
          name,
          createdAt: now,
          updatedAt: now
        },
        scene: newScene,
        settings: {
          paperFormat: "A4",
          marginMm: 5,
          scale: 1,
          extensions: {}
        },
        extensions: {}
      }
      this.projectPath = null
    },


    // Сохранение проекта (если уже есть путь)
    async saveProject() {
      if (!this.projectPath) {
        throw new Error('Project path is not set')
      }
      
      await this.saveProjectToFile(this.projectPath)
    },

    // Размещение разверток
    async nestProject(params: NestParams) {
      if (!this.currentProject) {
        throw new Error('No project to nest')
      }

      this.isNesting = true
      this.nestError = null

      try {
        const result = await invoke<NestResult>('nest_project', {
          project: this.currentProject,
          params
        })
        this.nestResult = result
      } catch (error: any) {
        this.nestError = error.toString()
        console.error('Failed to nest project:', error)
      } finally {
        this.isNesting = false
      }
    },

    // Экспорт результата размещения в SVG
    async exportNestResultToSvg(params: NestParams, baseFileName?: string) {
      if (!this.currentProject) {
        throw new Error('No project to export')
      }

      this.isExporting = true
      this.exportError = null
      this.exportSheetError = null
      this.exportProgress = null
      this.exportTotal = null
      this.exportSheetIndex = null
      this.exportSheetTotal = null

      try {
        // Получаем результат размещения
        const nestResult = await invoke<import('@/../../shared/src/types/nesting').NestResult>('nest_project', {
          project: this.currentProject,
          params
        })
        
        const svgs = await invoke<string[]>('export_nest_result_to_svg', {
          project: this.currentProject,
          params
        })
        
        // Если передано имя файла, сохраняем файлы
        if (baseFileName) {
          this.exportTotal = svgs.length
          this.exportProgress = 0
          this.exportSheetIndex = 0
          this.exportSheetTotal = svgs.length
          
          for (let i = 0; i < svgs.length; i++) {
            try {
              this.exportSheetIndex = i + 1
              this.exportSheetError = null
              const fileName = svgs.length > 1
                ? `${baseFileName}_${i + 1}.svg`
                : `${baseFileName}.svg`
              
              await writeTextFile(fileName, svgs[i])
              this.exportProgress = i + 1
            } catch (sheetError: any) {
              // Обработка ошибки конкретного листа
              this.exportSheetError = sheetError.toString()
              console.error(`Failed to export sheet ${i}:`, sheetError)
              // Продолжаем экспорт остальных листов
            }
          }
          
          this.exportProgress = null
          this.exportTotal = null
          this.exportSheetIndex = null
          this.exportSheetTotal = null
        }
        
        return svgs
      } catch (error: any) {
        this.exportError = error.toString()
        console.error('Failed to export nest result to SVG:', error)
        throw new Error(`Failed to export nest result to SVG: ${error}`)
      } finally {
        this.isExporting = false
        this.exportSheetIndex = null
        this.exportSheetTotal = null
      }
    },

    // Получение SVG для конкретного листа с кэшированием
    async getSheetSvg(sheetIndex: number): Promise<string> {
      // Проверяем, есть ли уже кэшированная версия
      if (this.sheetSvgCache[sheetIndex]) {
        return this.sheetSvgCache[sheetIndex];
      }

      // Проверяем, есть ли результат размещения
      if (!this.currentProject || !this.nestResult) {
        throw new Error('No nest result available');
      }

      // Проверяем, что индекс листа корректный
      if (sheetIndex < 0 || sheetIndex >= this.nestResult.sheets.length) {
        throw new Error(`Invalid sheet index: ${sheetIndex}`);
      }

      this.isSvgLoading = true;
      
      try {
        // Для PR1 возвращаем заглушку SVG
        // В будущем здесь будет вызов Tauri команды:
        // const svg = await invoke<string>('export_sheet_to_svg', {
        //   project: this.currentProject,
        //   nestResult: this.nestResult,
        //   sheetIndex
        // });
        
        // Создаем заглушку SVG для листа
        const sheet = this.nestResult.sheets[sheetIndex];
        const width = sheet.width_mm;
        const height = sheet.height_mm;
        const partsCount = sheet.parts.length;
        
        const svg = `<svg xmlns="http://www.w3.org/2000/svg" width="${width}mm" height="${height}mm" viewBox="0 0 ${width} ${height}">
  <rect x="0" y="0" width="${width}" height="${height}" fill="white" stroke="black" stroke-width="0.5"/>
  <text x="50%" y="50%" text-anchor="middle" dominant-baseline="middle" font-family="Arial" font-size="12">
    Лист ${sheetIndex + 1}
    ${width} × ${height} мм
    ${partsCount} частей
  </text>
</svg>`;
        
        // Кэшируем результат
        this.sheetSvgCache[sheetIndex] = svg;
        return svg;
      } catch (error: any) {
        console.error(`Failed to get SVG for sheet ${sheetIndex}:`, error);
        throw new Error(`Failed to get SVG for sheet ${sheetIndex}: ${error}`);
      } finally {
        this.isSvgLoading = false;
      }
    },

    // Очистка кэша SVG листов
    clearSvgCache() {
      this.sheetSvgCache = {};
    },

    // Установка переопределения для части
    setPartOverride(override: PartOverride) {
      this.partOverrides[override.partId] = override;
      this.isDirty = true;
      // Запускаем автосохранение
      this.scheduleAutoSave();
    },

    // Удаление переопределения для части
    removePartOverride(partId: number) {
      delete this.partOverrides[partId];
      this.isDirty = true;
      // Запускаем автосохранение
      this.scheduleAutoSave();
    },

    // Сброс всех переопределений
    resetPartOverrides() {
      this.partOverrides = {};
      this.isDirty = true;
      // Запускаем автосохранение
      this.scheduleAutoSave();
    },

    // Получение переопределения для части
    getPartOverride(partId: number): PartOverride | undefined {
      return this.partOverrides[partId];
    },

    // Проверка, есть ли переопределения
    hasPartOverrides(): boolean {
      return Object.keys(this.partOverrides).length > 0;
    },

    // Инициализация автосохранения
    initAutoSave() {
      // Простая реализация debounce
      const debounce = (fn: () => Promise<void>, delay: number) => {
        let timeoutId: number | null = null;
        return () => {
          if (timeoutId !== null) {
            clearTimeout(timeoutId);
          }
          timeoutId = setTimeout(() => {
            fn();
          }, delay) as unknown as number;
        };
      };

      // Создаем дебаунсированную функцию автосохранения с задержкой 5 секунд
      this.autoSaveProject = debounce(async () => {
        if (this.isDirty && this.currentProject && this.projectPath) {
          try {
            // Добавляем overrides в проект перед сохранением
            const projectToSave = { ...this.currentProject };
            if (!projectToSave.extensions) {
              projectToSave.extensions = {};
            }
            projectToSave.extensions['partOverrides'] = this.partOverrides;
            
            // Сохраняем проект
            await invoke('save_project', {
              project: projectToSave,
              path: this.projectPath
            });
            
            // Обновляем время изменения
            projectToSave.projectMeta.updatedAt = new Date().toISOString();
            this.currentProject = projectToSave;
            
            // Сбрасываем флаг изменений
            this.isDirty = false;
            console.log('Project auto-saved successfully');
          } catch (error) {
            console.error('Failed to auto-save project:', error);
          }
        }
      }, 5000); // 5 секунд задержки
    },

    // Запланировать автосохранение
    scheduleAutoSave() {
      if (this.autoSaveProject) {
        this.autoSaveProject();
      }
    },

    // Загрузка проекта из файла с восстановлением overrides
    async loadProjectFromFile(path: string) {
      this.isLoading = true
      this.loadError = null
      
      try {
        const project = await invoke<PepaProject>('load_project', { path })
        
        // Восстанавливаем overrides из расширений проекта
        if (project.extensions && project.extensions['partOverrides']) {
          this.partOverrides = project.extensions['partOverrides'] as Record<number, PartOverride>;
        } else {
          this.partOverrides = {};
        }
        
        this.currentProject = project
        this.projectPath = path
      } catch (error: any) {
        this.loadError = error.toString()
        console.error('Failed to load project:', error)
      } finally {
        this.isLoading = false
      }
    },

    // Сохранение проекта в файл с сохранением overrides
    async saveProjectToFile(path: string) {
      if (!this.currentProject) {
        throw new Error('No project to save')
      }
      
      try {
        // Добавляем overrides в проект перед сохранением
        const projectToSave = { ...this.currentProject };
        if (!projectToSave.extensions) {
          projectToSave.extensions = {};
        }
        projectToSave.extensions['partOverrides'] = this.partOverrides;
        
        await invoke('save_project', {
          project: projectToSave,
          path
        })
        this.projectPath = path
        // Обновляем время изменения
        projectToSave.projectMeta.updatedAt = new Date().toISOString();
        this.currentProject = projectToSave;
        // Сбрасываем флаг изменений
        this.isDirty = false;
      } catch (error: any) {
        throw new Error(`Failed to save project: ${error}`)
      }
    }
  }
})