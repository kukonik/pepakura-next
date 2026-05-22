export interface MeshData {
  name: string
  vertices: number[]
  normals?: number[]
  triangles: Array<{ vertices: [number, number, number] }>
  materials?: Array<{
    diffuse: { r: number; g: number; b: number }
    specular: { r: number; g: number; b: number }
    shininess: number
  }>
}

export interface ModelStats {
  vertices: number
  triangles: number
  parts: number
}
