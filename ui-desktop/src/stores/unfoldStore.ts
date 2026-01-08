import { defineStore } from 'pinia'
import { invoke } from '@tauri-apps/api/tauri'
import { useProjectStore } from '@/stores/project'

export type LineKind = 'Cut' | 'Valley' | 'Mountain' | 'GlueTab'

export interface UnfoldLine {
  x1: number
  y1: number
  x2: number
  y2: number
  kind: LineKind
}

export interface UnfoldRect {
  x: number
  y: number
  width: number
  height: number
}

export interface UnfoldPart {
  id: number
  name?: string
  bounds: UnfoldRect
  lines: UnfoldLine[]
}

export interface UnfoldSheet {
  id: number
  index: number
  width_mm: number
  height_mm: number
  margin_mm: number
  parts: UnfoldPart[]
}

export interface UnfoldResult {
  sheets: UnfoldSheet[]
}

export type UnfoldStatus = 'idle' | 'running' | 'ready' | 'error'

export interface UnfoldParams {
  paperFormat: 'A4' | 'A3' | 'Letter'
  marginMm: number
  maxSheets: number
  scale: number
}

interface UnfoldState {
  status: UnfoldStatus
  result: UnfoldResult | null
  errorMessage: string | null
}

export const useUnfoldStore = defineStore('unfold', {
  state: (): UnfoldState => ({
    status: 'idle',
    result: null,
    errorMessage: null,
  }),

  getters: {
    totalSheets(state): number {
      return state.result?.sheets.length ?? 0
    },
    totalParts(state): number {
      if (!state.result) return 0
      return state.result.sheets.reduce((sum, sheet) => sum + sheet.parts.length, 0)
    },
  },

  actions: {
    reset() {
      this.status = 'idle'
      this.result = null
      this.errorMessage = null
    },

    async runAutoUnfold(params: UnfoldParams): Promise<void> {
      const projectStore = useProjectStore()
      const modelPath = projectStore.threeD.workingPath || projectStore.threeD.sourcePath

      if (!modelPath) {
        this.status = 'error'
        this.errorMessage = 'Нет пути к 3D‑модели для развёртки.'
        return
      }

      this.status = 'running'
      this.errorMessage = null

      try {
        const rustParams = {
          paper_format: params.paperFormat,
          margin_mm: params.marginMm,
          max_sheets: params.maxSheets,
          scale: params.scale,
        }

        const result = await invoke<UnfoldResult>('unfold_3d_model', {
          modelPath,
          params: rustParams,
        })

        this.result = result
        this.status = 'ready'
      } catch (e: any) {
        console.error('[PepakuraNext] unfold_3d_model failed', e)
        this.status = 'error'
        this.errorMessage =
          typeof e === 'string' ? e : e?.message || 'Ошибка развёртки в движке.'
        this.result = null
      }
    },
  },
})
