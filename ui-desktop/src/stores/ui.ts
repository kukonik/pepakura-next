import { defineStore } from 'pinia'

export type PageId = 'welcome' | 'editor' | 'settings'
export type InterfaceMode = 'simple' | 'advanced'

export interface WindowState {
  id: string
  type: 'viewer' | 'log' | 'properties'
  title: string
  visible: boolean
  dock: 'right' | 'bottom'
}

export interface UiState {
  currentPage: PageId
  interfaceMode: InterfaceMode
  viewerFullscreen: boolean
  windows: WindowState[]
}

export const useUiStore = defineStore('ui', {
  state: (): UiState => ({
    currentPage: 'welcome',
    interfaceMode: 'simple',
    viewerFullscreen: false,
    windows: [
      {
        id: 'viewer',
        type: 'viewer',
        title: 'Просмотрщик',
        visible: true,
        dock: 'right'
      },
      {
        id: 'log',
        type: 'log',
        title: 'Лог',
        visible: false,
        dock: 'bottom'
      }
    ]
  }),
  actions: {
    setPage(page: PageId) {
      this.currentPage = page
    },
    setInterfaceMode(mode: InterfaceMode) {
      this.interfaceMode = mode
    },
    toggleInterfaceMode() {
      this.interfaceMode = this.interfaceMode === 'simple' ? 'advanced' : 'simple'
    },
    setViewerFullscreen(value: boolean) {
      this.viewerFullscreen = value
    },
    toggleViewerFullscreen() {
      this.viewerFullscreen = !this.viewerFullscreen
    },
    toggleWindow(id: string) {
      const w = this.windows.find(w => w.id === id)
      if (!w) return
      w.visible = !w.visible
    },
    setWindowDock(id: string, dock: 'right' | 'bottom') {
      const w = this.windows.find(w => w.id === id)
      if (!w) return
      w.dock = dock
    }
  }
})
