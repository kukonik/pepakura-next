export interface Warning {
  code: string
  message: string
  partId?: number
  severity: 'info' | 'warning' | 'error'
}
export interface BoundingBox {
  minX: number;
  minY: number;
  minZ: number;
  maxX: number;
  maxY: number;
  maxZ: number;
}

export interface Material {
  id: number;
  name: string;
  color: [number, number, number, number]; // RGBA
}

export interface PepaMesh {
  positions: number[];
  normals: number[];
  uvs: number[];
  indices: number[];
  materialId: number;
}

export interface PepaScene {
  meshes: PepaMesh[];
  materials: Material[];
  boundingBox: BoundingBox;
  vertexCount: number;
  faceCount: number;
}

export interface ParsePdoResult {
  success: boolean;
  error?: string;
  scene?: PepaScene;
}

