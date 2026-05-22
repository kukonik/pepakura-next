/**
 * Контроллер сцены модели
 */

import * as THREE from 'three';
import type { SceneContext } from './SceneContext';
import type { SeamSet } from '@pepakura-next/shared';
import type { SeamVisualizer } from './SeamVisualizer';

/**
 * Контроллер сцены модели
 */
export class ModelSceneController {
  // Контекст сцены
  private sceneContext: SceneContext;
  
  // Визуализатор швов
  private seamVisualizer: SeamVisualizer;
  
  /**
   * Конструктор
   * @param sceneContext Контекст сцены
   * @param seamVisualizer Визуализатор швов
   */
  constructor(sceneContext: SceneContext, seamVisualizer: SeamVisualizer) {
    this.sceneContext = sceneContext;
    this.seamVisualizer = seamVisualizer;
  }
  
  /**
   * Обновление отображения швов
   * @param seamSet Набор швов для отображения
   */
  updateSeams(seamSet: SeamSet): void {
    // Передаем набор швов в визуализатор
    this.seamVisualizer.updateSeams(seamSet);
  }
  
  /**
   * Получение сцены Three.js
   * @returns Сцена Three.js
   */
  getScene(): THREE.Scene {
    return this.sceneContext.scene;
  }
  
  /**
   * Получение камеры Three.js
   * @returns Камера Three.js
   */
  getCamera(): THREE.PerspectiveCamera {
    return this.sceneContext.camera;
  }
  
  /**
   * Получение рендерера Three.js
   * @returns Рендерер Three.js
   */
  getRenderer(): THREE.WebGLRenderer {
    return this.sceneContext.renderer;
  }
}