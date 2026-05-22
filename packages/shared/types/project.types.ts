export interface Project {
  id: string
  name: string
  description?: string
  createdAt: Date
  updatedAt: Date
  thumbnail?: string
  tags: string[]
}

export interface ProjectScene {
  projectId: string
  objects: SceneObject[]
  materials: Material[]
  textures: Texture[]
  settings: UnfoldSettings
}

export interface SceneObject {
  id: string
  name: string
  type: 'mesh' | 'group' | 'light' | 'camera'
  parentId?: string
  transform: {
    position: [number, number, number]
    rotation: [number, number, number]
    scale: [number, number, number]
  }
  visible: boolean
  locked: boolean
}

export interface Material {
  id: string
  name: string
  color: string
  textureId?: string
  properties: Record<string, any>
}

export interface Texture {
  id: string
  name: string
  url: string
  width: number
  height: number
}

export interface UnfoldSettings {
  scale: number
  gap: number
  addTabs: boolean
  addNumbers: boolean
  sheetSize: {
    width: number
    height: number
  }
  lineWidth: number
  lineColor: string
}
