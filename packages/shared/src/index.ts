// Основной экспорт shared пакета
export { default as ModelViewer } from './components/ModelViewer.vue'
export { default as PepakuraLayout } from './components/PepakuraLayout.vue'
export { useThreeJsScene } from './composables/useThreeJsScene'
export { i18n } from './i18n'
export type { MeshData } from './types/model'

export { useProjectStore } from './stores/useProjectStore';

// Platform Bridge
export {
  platformBridge,
  createPlatformBridge,
  detectPlatform,
  TauriBridge,
  WebBridge,
  type IPlatformBridge,
  type CommandResult,
  type ProjectData,
  type ProjectSettings,
  type ExportData,
  type OpenFileDialogOptions,
  type OpenDirectoryDialogOptions,
  type SaveFileDialogOptions,
  type FileFilter,
  type UnfoldConfig,
} from './platform/platform-bridge';

// Platform utils
export { isTauri, getPlatformName } from './platform/platform';
