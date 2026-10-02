export type Vec3 = [number, number, number];
export interface CameraState {
    position: Vec3;
    target: Vec3;
}
export interface LightsState {
    ambient: number;
    dir: number;
}
export interface MaterialState {
    color: string;
}
export declare function loadObjModel(absolutePath: string): void;
export declare function setCamera(position: Vec3, target: Vec3): void;
export declare function setModelMatrix(matrix: number[]): void;
export declare function setLights(ambient: number, dir: number): void;
export declare function applySeams(seams: number[]): void;
export declare function applyMeshData(positions: number[], uvs: number[]): void;
export declare function getCurrentModelPath(): string;
export declare function getCurrentCamera(): CameraState;
export declare function getCurrentModelMatrix(): number[];
export declare function getCurrentLights(): LightsState;
export declare function getCurrentSeams(): number[];
export declare function getCurrentMeshPositions(): number[];
export declare function getCurrentMeshUVs(): number[];
export declare function getCurrentMaterials(): MaterialState[];
