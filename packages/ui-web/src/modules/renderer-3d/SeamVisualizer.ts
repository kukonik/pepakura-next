/**
 * Визуализатор швов
 */

import * as THREE from 'three';
import type { SeamSet } from '@pepakura-next/shared';

/**
 * Визуализатор швов
 */
export class SeamVisualizer {
  // Сцена Three.js
  private scene: THREE.Scene;
  
  // Линии для отображения швов
  private seamLines: THREE.LineSegments | null;
  
  // Материал для отображения швов
  private seamMaterial: THREE.LineBasicMaterial;
  
  /**
   * Конструктор
   * @param scene Сцена Three.js
   */
  constructor(scene: THREE.Scene) {
    this.scene = scene;
    this.seamLines = null;
    
    // Создаем материал для швов
    this.seamMaterial = new THREE.LineBasicMaterial({
      color: 0xff0000, // Красный цвет для швов
      linewidth: 2,
    });
  }
  
  /**
   * Отображение визуализатора швов
   */
  show(): void {
    // TODO: Реализовать отображение визуализатора швов
  }
  
  /**
   * Скрытие визуализатора швов
   */
  hide(): void {
    // Удаляем линии швов из сцены, если они есть
    if (this.seamLines) {
      this.scene.remove(this.seamLines);
      this.seamLines = null;
    }
  }
  
  /**
   * Обновление отображения швов
   * @param seamSet Набор швов для отображения
   */
  updateSeams(seamSet: SeamSet): void {
    // Удаляем старые линии швов из сцены, если они есть
    if (this.seamLines) {
      this.scene.remove(this.seamLines);
      this.seamLines = null;
    }
    
    // Проверяем, что есть швы для отображения
    if (seamSet.edges.length === 0) {
      return;
    }
    
    // Создаем геометрию для линий швов
    const geometry = new THREE.BufferGeometry();
    
    // Создаем массив точек для линий
    const points: THREE.Vector3[] = [];
    
    // Добавляем точки для каждого ребра
    for (const edge of seamSet.edges) {
      points.push(new THREE.Vector3(edge.vertex1.x, edge.vertex1.y, edge.vertex1.z));
      points.push(new THREE.Vector3(edge.vertex2.x, edge.vertex2.y, edge.vertex2.z));
    }
    
    // Устанавливаем позиции точек в геометрию
    geometry.setFromPoints(points);
    
    // Создаем линии швов
    this.seamLines = new THREE.LineSegments(geometry, this.seamMaterial);
    
    // Добавляем линии швов в сцену
    this.scene.add(this.seamLines);
  }
}