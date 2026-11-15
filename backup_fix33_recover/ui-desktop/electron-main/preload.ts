import { contextBridge, ipcRenderer } from 'electron'

// Безопасный API для работы с файловой системой
contextBridge.exposeInMainWorld('electronAPI', {
  // Файловые операции
  openFileDialog: () => ipcRenderer.invoke('dialog:open-file'),
  saveFileDialog: (defaultPath?: string) => ipcRenderer.invoke('dialog:save-file', defaultPath),
  readFile: (filePath: string) => ipcRenderer.invoke('fs:read-file', filePath),
  writeFile: (filePath: string, data: string) => ipcRenderer.invoke('fs:write-file', filePath, data),
  
  // Системные операции
  getAppPath: () => ipcRenderer.invoke('app:get-path'),
  getPlatform: () => ipcRenderer.invoke('app:get-platform'),
})
