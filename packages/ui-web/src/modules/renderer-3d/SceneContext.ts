/**
 * Контекст сцены
 */

import * as THREE from 'three';

/**
 * Контекст сцены
 */
export class SceneContext {
  // Сцена Three.js
  scene: THREE.Scene;
  
  // Камера Three.js
  camera: THREE.PerspectiveCamera;
  
  // Рендерер Three.js
  renderer: THREE.WebGLRenderer;
  
  /**
   * Конструктор
   */
  constructor() {
    // Создаем сцену
    this.scene = new THREE.Scene();
    
    // Создаем камеру
    this.camera = new THREE.PerspectiveCamera(
      75, // fov
      window.innerWidth / window.innerHeight, // aspect
      0.1, // near
      1000 // far
    );
    
    // Создаем рендерер
    this.renderer = new THREE.WebGLRenderer({ antialias: true });
    this.renderer.setSize(window.innerWidth, window.innerHeight);
    this.renderer.setClearColor(0x000000, 0); // Прозрачный фон
  }
}