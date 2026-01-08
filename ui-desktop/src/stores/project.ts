// src/stores/project.ts
import { defineStore } from 'pinia'

export type StageId = 'text' | 'twod' | 'threed' | 'unfold'

export type SheetFormat = 'A4' | 'A3' | 'Letter' | 'Custom'

export interface TextState {
  rawText: string
  lastAiPrompt: string | null
  aiEnabled: boolean
}

export interface TwoDImage {
  id: string
  role: 'reference' | 'blueprint' | 'photo'
  label: string
  path: string | null
}

export interface TwoDState {
  images: TwoDImage[]
  activeImageId: string | null
}

export interface ThreeDState {
  sourcePath: string | null        // исходный OBJ/FBX/GLB и т.п.
  workingPath: string | null       // нормализованный/очищенный mesh
  format: 'obj+mtl' | 'fbx' | 'gltf' | 'unknown'
  faces: number | null
  parts: number | null
  status: 'empty' | 'ready' | 'error'
  lastError: string | null
}

export interface UnfoldState {
  layoutPath: string | null
  sheetFormat: SheetFormat
  flapWidthMm: number
  showNumbers: boolean
  showFolds: boolean
  estimatedPatches: number | null  // оценка числа 2D‑островов
  estimatedSheets: number | null   // оценка числа листов
}

export interface ProjectState {
  id: string | null
  name: string
  createdAt: string | null
  updatedAt: string | null
  stage: StageId
  text: TextState
  twod: TwoDState
  threeD: ThreeDState
  unfold: UnfoldState
}

export const useProjectStore = defineStore('project', {
  state: (): ProjectState => ({
    id: null,
    name: 'Новый проект',
    createdAt: null,
    updatedAt: null,
    stage: 'text',

    text: {
      rawText: '',
      lastAiPrompt: null,
      aiEnabled: true
    },

    twod: {
      images: [],
      activeImageId: null
    },

    threeD: {
      sourcePath: null,
      workingPath: null,
      format: 'unknown',
      faces: null,
      parts: null,
      status: 'empty',
      lastError: null
    },

    unfold: {
      layoutPath: null,
      sheetFormat: 'A3',
      flapWidthMm: 12,
      showNumbers: true,
      showFolds: false,
      estimatedPatches: null,
      estimatedSheets: null
    }
  }),

  getters: {
    isReadyForUnfold: state =>
      !!(state.threeD.workingPath && state.threeD.status === 'ready')
  },

  actions: {
    setStage(stage: StageId) {
      this.stage = stage
      this.touch()
    },

    touch() {
      this.updatedAt = new Date().toISOString()
    },

    // TEXT
    setText(params: Partial<TextState>) {
      this.text = { ...this.text, ...params }
      this.touch()
    },

    // 2D
    addTwoDImage(img: Partial<TwoDImage>) {
      const id = img.id ?? crypto.randomUUID()
      const full: TwoDImage = {
        id,
        role: img.role ?? 'reference',
        label: img.label ?? 'Изображение',
        path: img.path ?? null
      }
      this.twod.images.push(full)
      if (!this.twod.activeImageId) {
        this.twod.activeImageId = id
      }
      this.touch()
    },

    updateTwoDImage(id: string, patch: Partial<TwoDImage>) {
      const idx = this.twod.images.findIndex(i => i.id === id)
      if (idx === -1) return
      this.twod.images[idx] = { ...this.twod.images[idx], ...patch }
      this.touch()
    },

    removeTwoDImage(id: string) {
      this.twod.images = this.twod.images.filter(i => i.id !== id)
      if (this.twod.activeImageId === id) {
        this.twod.activeImageId = this.twod.images[0]?.id ?? null
      }
      this.touch()
    },

    setActiveTwoDImage(id: string | null) {
      this.twod.activeImageId = id
      this.touch()
    },

    // 3D
    setThreeD(params: Partial<ThreeDState>) {
      this.threeD = { ...this.threeD, ...params }
      this.touch()
    },

    markThreeDReady(meta?: { faces?: number; parts?: number }) {
      this.threeD.status = 'ready'
      if (meta?.faces != null) this.threeD.faces = meta.faces
      if (meta?.parts != null) this.threeD.parts = meta.parts
      this.threeD.lastError = null
      this.touch()
    },

    markThreeDError(message: string) {
      this.threeD.status = 'error'
      this.threeD.lastError = message
      this.touch()
    },

    resetThreeD() {
      this.threeD = {
        sourcePath: null,
        workingPath: null,
        format: 'unknown',
        faces: null,
        parts: null,
        status: 'empty',
        lastError: null
      }
      this.touch()
    },

    // UNFOLD (Paper)
    setUnfold(params: Partial<UnfoldState>) {
      this.unfold = { ...this.unfold, ...params }
      this.touch()
    },

    resetUnfold() {
      this.unfold = {
        layoutPath: null,
        sheetFormat: 'A3',
        flapWidthMm: 12,
        showNumbers: true,
        showFolds: false,
        estimatedPatches: null,
        estimatedSheets: null
      }
      this.touch()
    },

    recomputeUnfoldEstimates() {
      const faces = this.threeD.faces ?? 0
      const parts = this.threeD.parts ?? 0

      if (!faces || !parts) {
        this.unfold.estimatedPatches = null
        this.unfold.estimatedSheets = null
        return
      }

      // очень грубые эвристики, дальше их можно заменить данными из Rust‑движка
      const patches = Math.max(1, Math.round(faces / 400))
      const sheets = Math.max(1, Math.round(faces / 600))

      this.unfold.estimatedPatches = patches
      this.unfold.estimatedSheets = sheets
      this.touch()
    }
  }
})
