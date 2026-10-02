export enum AppMode {
  VIEWER_3D = 'viewer_3d',
  EDITOR_2D = 'editor_2d',
  TXT_MODE = 'txt_mode',
  PRINT_MODE = 'print_mode'
}

export interface Project {
  id: string;
  name: string;
  createdAt: string;
  updatedAt: string;
  // Ссылки на файлы (локальные пути или blob URLs)
  modelObj: string | null;
  modelMtl: string | null;
  // Метаданные проекта
  config: {
    scale: number;
    units: 'mm' | 'cm' | 'inch';
  };
}

export interface ProjectState {
  currentProject: Project | null;
  appMode: AppMode;
  isLoading: boolean;
}
