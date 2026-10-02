/**
 * Модуль визуализации швов на базе Three.js.
 * Принимает меш и список рёбер, создаёт и обновляет объект LineSegments.
 */
import * as THREE from "three";
import { SeamEdge } from "D:\\Dev\\pepakura-next\\src\\modules\\renderer-3d\\sceneRuntime";
export declare class SeamVisualizer {
    private scene;
    private lines;
    private material;
    constructor(scene: THREE.Scene);
    /**
     * Обновляет отображение швов.
     * @param geometry Геометрия основного меша (для координат вершин)
     * @param edges Массив рёбер швов
     */
    updateSeams(geometry: THREE.BufferGeometry, edges: SeamEdge[]): void;
    clear(): void;
    dispose(): void;
}
