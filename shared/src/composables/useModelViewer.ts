// shared/src/composables/useModelViewer.ts
import { ref, onMounted, onUnmounted, type Ref } from 'vue';
import * as THREE from 'three';
import { OrbitControls } from 'three/examples/jsm/controls/OrbitControls.js';
import type { MeshData } from '../types/model';

// --- Глобальные переменные для Three.js (singleton) ---
let scene: THREE.Scene | null = null;
let camera: THREE.PerspectiveCamera | null = null;
let renderer: THREE.WebGLRenderer | null = null;
let controls: OrbitControls | null = null;
let mesh3d: THREE.Mesh | null = null;
let containerElement: HTMLElement | null = null;
let animationFrameId: number | null = null;
let isInitialized = false;
let resizeHandler: (() => void) | null = null;

// --- Функция инициализации Three.js ---
function initThree(containerEl: HTMLElement) {
  // Если уже инициализирован, очищаем предыдущую инициализацию
  if (isInitialized) {
    cleanup();
  }

  console.log('initThree: Инициализация Three.js для элемента:', containerEl.id || containerEl.tagName);

  const rect = containerEl.getBoundingClientRect();
  const width = rect.width || 800;
  const height = rect.height || 600;

  // Сцена
  scene = new THREE.Scene();
  scene.background = new THREE.Color(0x0b1120);

  // Камера
  camera = new THREE.PerspectiveCamera(75, width / height, 0.1, 1000);
  camera.position.z = 5;

  // Рендерер
  renderer = new THREE.WebGLRenderer({ antialias: true, alpha: true });
  renderer.setSize(width, height);
  renderer.setPixelRatio(window.devicePixelRatio);
  containerEl.appendChild(renderer.domElement);

  // Controls
  controls = new OrbitControls(camera, renderer.domElement);
  controls.enableDamping = true;
  controls.dampingFactor = 0.05;

  // Освещение
  const ambientLight = new THREE.AmbientLight(0xffffff, 0.6);
  scene.add(ambientLight);
  const directionalLight = new THREE.DirectionalLight(0xffffff, 0.8);
  directionalLight.position.set(10, 10, 5);
  scene.add(directionalLight);

  // Обработка изменения размера окна
  resizeHandler = () => {
    if (!containerElement || !camera || !renderer) return;
    const rect = containerElement.getBoundingClientRect();
    camera.aspect = rect.width / rect.height;
    camera.updateProjectionMatrix();
    renderer.setSize(rect.width, rect.height);
  };
  window.addEventListener('resize', resizeHandler);

  // Запуск цикла рендеринга
  animate();

  isInitialized = true;
  console.log('Three.js инициализирован.');
}

// --- Цикл рендеринга ---
function animate() {
  if (controls) controls.update();
  if (renderer && scene && camera) {
    renderer.render(scene, camera);
  }
  animationFrameId = requestAnimationFrame(animate);
}

// --- Функция очистки ---
function cleanup() {
  if (animationFrameId !== null) {
    cancelAnimationFrame(animationFrameId);
    animationFrameId = null;
  }

  if (resizeHandler) {
    window.removeEventListener('resize', resizeHandler);
    resizeHandler = null;
  }

  if (renderer) {
    if (containerElement && renderer.domElement.parentNode) {
      containerElement.removeChild(renderer.domElement);
    }
    renderer.dispose();
    renderer = null;
  }

  if (controls) {
    controls.dispose();
    controls = null;
  }

  if (mesh3d) {
    if (scene) scene.remove(mesh3d);
    if (mesh3d.geometry) mesh3d.geometry.dispose();
    if (Array.isArray(mesh3d.material)) {
      mesh3d.material.forEach(m => m.dispose());
    } else if (mesh3d.material) {
      mesh3d.material.dispose();
    }
    mesh3d = null;
  }

  scene = null;
  camera = null;
  containerElement = null;
  isInitialized = false;
}

// --- Функция рендеринга модели ---
function renderMesh(modelData: MeshData) {
  console.log('renderMesh: Получены данные для рендеринга:', modelData);

  if (!scene) {
    console.warn('Сцена Three.js не инициализирована. Вызовите setupViewer сначала.');
    return;
  }

  // Удаляем предыдущую модель
  if (mesh3d) {
    scene.remove(mesh3d);
    if (mesh3d.geometry) mesh3d.geometry.dispose();
    if (Array.isArray(mesh3d.material)) {
      mesh3d.material.forEach(m => m.dispose());
    } else if (mesh3d.material) {
      mesh3d.material.dispose();
    }
    mesh3d = null;
  }

  if (!modelData.vertices || modelData.vertices.length === 0) {
    console.warn('Нет данных о вершинах для рендеринга.');
    return;
  }

  // Геометрия
  const geometry = new THREE.BufferGeometry();
  const vertices = new Float32Array(modelData.vertices);
  geometry.setAttribute('position', new THREE.BufferAttribute(vertices, 3));

  // Индексы треугольников
  if (modelData.triangles && modelData.triangles.length > 0) {
    const indices: number[] = [];
    for (const tri of modelData.triangles) {
      // Проверяем валидность индексов
      const v0 = tri.vertices[0];
      const v1 = tri.vertices[1];
      const v2 = tri.vertices[2];
      
      const maxIndex = vertices.length / 3 - 1;
      if (v0 >= 0 && v0 <= maxIndex && v1 >= 0 && v1 <= maxIndex && v2 >= 0 && v2 <= maxIndex) {
        indices.push(v0, v1, v2);
      } else {
        console.warn(`Некорректные индексы вершин: [${v0}, ${v1}, ${v2}], максимум: ${maxIndex}`);
      }
    }
    
    if (indices.length > 0) {
      geometry.setIndex(indices);
    } else {
      console.error('Нет валидных индексов для геометрии');
      return;
    }
  } else {
    console.warn('Нет треугольников для рендеринга.');
    return;
  }

  // Нормали
  if (modelData.normals && modelData.normals.length > 0) {
    // Проверяем, что количество нормалей соответствует вершинам
    if (modelData.normals.length === modelData.vertices.length) {
      const normals = new Float32Array(modelData.normals);
      geometry.setAttribute('normal', new THREE.BufferAttribute(normals, 3));
    } else {
      console.warn('Количество нормалей не соответствует количеству вершин, вычисляем нормали');
      geometry.computeVertexNormals();
    }
  } else {
    console.log('Вычисляем нормали автоматически');
    geometry.computeVertexNormals();
  }

  // Материалы
  let material: THREE.Material | THREE.Material[];
  
  if (modelData.materials && modelData.materials.length > 0) {
    // Если есть несколько материалов, создаем массив материалов
    if (modelData.materials.length > 1) {
      material = modelData.materials.map(mat => {
        return new THREE.MeshPhongMaterial({
          color: new THREE.Color(mat.diffuse.r, mat.diffuse.g, mat.diffuse.b),
          specular: new THREE.Color(mat.specular.r, mat.specular.g, mat.specular.b),
          shininess: mat.shininess || 30,
          side: THREE.DoubleSide,
          wireframe: false
        });
      });
    } else {
      // Один материал
      const mat = modelData.materials[0];
      material = new THREE.MeshPhongMaterial({
        color: new THREE.Color(mat.diffuse.r, mat.diffuse.g, mat.diffuse.b),
        specular: new THREE.Color(mat.specular.r, mat.specular.g, mat.specular.b),
        shininess: mat.shininess || 30,
        side: THREE.DoubleSide,
        wireframe: false
      });
    }
  } else {
    material = new THREE.MeshPhongMaterial({
      color: 0x3b82f6,
      side: THREE.DoubleSide,
      wireframe: false
    });
  }

  // Меш
  mesh3d = new THREE.Mesh(geometry, material);
  scene.add(mesh3d);

  // Центрируем и масштабируем
  geometry.computeBoundingBox();
  if (geometry.boundingBox) {
    const box = geometry.boundingBox;
    const center = box.getCenter(new THREE.Vector3());
    const size = box.getSize(new THREE.Vector3());
    const maxDim = Math.max(size.x, size.y, size.z);
    const radius = maxDim / 2;
    
    if (controls && camera) {
      controls.target.copy(center);
      camera.position.set(
        center.x,
        center.y,
        center.z + radius * 2.5
      );
      camera.lookAt(center);
      camera.near = radius / 100;
      camera.far = radius * 100;
      camera.updateProjectionMatrix();
      controls.update();
    }
  } else {
    // Fallback если boundingBox не вычислен
    if (controls && camera) {
      controls.target.set(0, 0, 0);
      camera.position.set(0, 0, 5);
      camera.updateProjectionMatrix();
      controls.update();
    }
  }

  console.log('Модель отрендерена в Three.js. Вершин:', vertices.length / 3, 'Треугольников:', modelData.triangles.length);
}

// --- Композабл ---
export function useModelViewer() {
  // Функция проверки готовности сцены
  function isSceneReady(): boolean {
    return scene !== null && camera !== null && renderer !== null && isInitialized;
  }

  // Функция установки сцены (принимает ref на HTMLElement)
  function setupViewer(containerRef: Ref<HTMLElement | null>) {
    onMounted(() => {
      // Используем setTimeout для гарантии, что DOM полностью отрендерен
      setTimeout(() => {
        if (containerRef.value) {
          containerElement = containerRef.value;
          console.log('useModelViewer: onMounted, инициализирую Three.js в контейнере:', containerRef.value);
          initThree(containerRef.value);
        } else {
          console.error('useModelViewer: onMounted, containerRef.value отсутствует!');
        }
      }, 100);
    });

    onUnmounted(() => {
      console.log('useModelViewer: onUnmounted, очищаю ресурсы Three.js');
      cleanup();
    });
  }

  // Возвращаем функции
  return {
    renderMesh,
    setupViewer,
    isSceneReady,
  };
}