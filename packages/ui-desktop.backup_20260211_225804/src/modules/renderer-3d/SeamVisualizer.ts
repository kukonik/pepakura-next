import * as THREE from 'three';
import type { SeamSet } from '../types/seam-set.type';

export class SeamVisualizer {
  private readonly scene: THREE.Scene;
  private lineMaterial: THREE.LineBasicMaterial;
  private lineGeometry: THREE.BufferGeometry;
  private line: THREE.LineSegments | null = null;

  constructor(scene: THREE.Scene) {
    this.scene = scene;

    this.lineMaterial = new THREE.LineBasicMaterial({ color: 0xff0000 });
    this.lineGeometry = new THREE.BufferGeometry();
  }

  update(seams: SeamSet): void {
    // Удаление старых линий
    if (this.line) {
      this.scene.remove(this.line);
    }

    if (seams.length === 0) {
      return;
    }

    // Преобразование швов в массив точек
    const points: number[] = [];
    for (const edge of seams) {
      points.push(edge.a, edge.b); // Пример: каждый шов - это пара индексов вершин
    }

    const positions = new Float32Array(points.length * 3); // 3 координаты на точку
    for (let i = 0; i < points.length; i += 2) {
      // Здесь мы предполагаем, что индексы указывают на вершины в геометрии
      // В реальности нужна более сложная логика для получения координат
      positions[i * 3] = points[i];     // x
      positions[i * 3 + 1] = points[i + 1]; // y
      positions[i * 3 + 2] = 0;         // z
    }

    this.lineGeometry.setAttribute('position', new THREE.BufferAttribute(positions, 3));

    this.line = new THREE.LineSegments(this.lineGeometry, this.lineMaterial);
    this.scene.add(this.line);
  }

  clear(): void {
    if (this.line) {
      this.scene.remove(this.line);
      this.line = null;
    }
  }
}
