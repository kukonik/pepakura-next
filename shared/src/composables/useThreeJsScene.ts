// shared/src/composables/useThreeJsScene.ts
import { ref, onMounted, onUnmounted, shallowRef } from 'vue';
import * as THREE from 'three';
import { OrbitControls } from 'three/examples/jsm/controls/OrbitControls.js';
import type { MeshData } from '../types/model';

export function useThreeJsScene(containerId: string = 'viewerContainer') {
  const containerRef = ref<HTMLElement | null>(null);
  let scene: THREE.Scene | null = null;
  let camera: THREE.PerspectiveCamera | null = null;
  let renderer: THREE.WebGLRenderer | null = null;
  let controls: OrbitControls | null = null;
  let mesh3d: THREE.Mesh | null = null; // Храним ссылку на отображаемую модель

  // Инициализация сцены
  const initScene = () => {
    const container = document.getElementById(containerId);
    if (!container) {
      console.error('Контейнер Three.js не найден:', containerId);
      return;
    }
    containerRef.value = container;

    // Убираем placeholder, если он ещё виден
    const placeholder = container.querySelector('.viewer-placeholder');
    if (placeholder) {
      (placeholder as HTMLElement).style.display = 'none';
    }

    const rect = container.getBoundingClientRect();
    const width = rect.width;
    const height = rect.height;

    // Создаём сцену
    scene = new THREE.Scene();
    scene.background = new THREE.Color(0x0b1120); // Цвет фона как в CSS

    // Создаём камеру
    camera = new THREE.PerspectiveCamera(75, width / height, 0.1, 1000);
    camera.position.z = 5;

    // Создаём рендерер
    renderer = new THREE.WebGLRenderer({ antialias: true, alpha: true });
    renderer.setSize(width, height);
    renderer.setPixelRatio(window.devicePixelRatio);
    container.appendChild(renderer.domElement);

    // Создаём controls
    if (camera && renderer) {
      controls = new OrbitControls(camera, renderer.domElement);
      controls.enableDamping = true;
      controls.dampingFactor = 0.05;
    }

    // Освещение
    const ambientLight = new THREE.AmbientLight(0xffffff, 0.6);
    scene.add(ambientLight);
    const directionalLight = new THREE.DirectionalLight(0xffffff, 0.8);
    directionalLight.position.set(10, 10, 5);
    scene.add(directionalLight);

    // Запускаем цикл рендеринга
    animate();
  };

  // Цикл рендеринга
  const animate = () => {
    if (controls) controls.update();
    if (renderer && scene && camera) {
      renderer.render(scene, camera);
    }
    requestAnimationFrame(animate);
  };

  // Рендеринг модели
  const renderMesh = (meshData: MeshData) => {
    if (!scene) {
      console.warn('Сцена Three.js не инициализирована, инициализирую...');
      initScene();
      if (!scene) {
        console.error('Не удалось инициализировать сцену для рендеринга.');
        return;
      }
    }

    // Удаляем предыдущую модель, если она была
    if (mesh3d) {
      scene.remove(mesh3d);
      if (mesh3d.geometry) mesh3d.geometry.dispose();
      if (mesh3d.material) {
        if (Array.isArray(mesh3d.material)) {
          mesh3d.material.forEach(m => m.dispose());
        } else {
          mesh3d.material.dispose();
        }
      }
      mesh3d = null;
    }

    if (!meshData.vertices || meshData.vertices.length === 0) {
      console.warn('Нет данных о вершинах для рендеринга.');
      return;
    }

    // Создаём геометрию
    const geometry = new THREE.BufferGeometry();
    const vertices = new Float32Array(meshData.vertices);
    geometry.setAttribute('position', new THREE.BufferAttribute(vertices, 3));

    if (meshData.triangles && meshData.triangles.length > 0) {
      const indices = [];
      for (const tri of meshData.triangles) {
        indices.push(...tri.vertices);
      }
      geometry.setIndex(indices);
    }

    if (!meshData.normals || meshData.normals.length === 0) {
      geometry.computeVertexNormals();
    } else {
      const normals = new Float32Array(meshData.normals);
      geometry.setAttribute('normal', new THREE.BufferAttribute(normals, 3));
    }

    // Создаём материал
    let material: THREE.Material;
    if (meshData.materials && meshData.materials.length > 0) {
      const m = meshData.materials[0]; // Берём первый материал
      material = new THREE.MeshPhongMaterial({
        color: new THREE.Color(m.diffuse.r, m.diffuse.g, m.diffuse.b),
        specular: new THREE.Color(m.specular.r, m.specular.g, m.specular.b),
        shininess: m.shininess,
        side: THREE.DoubleSide,
      });
    } else {
      material = new THREE.MeshPhongMaterial({ color: 0x3b82f6, side: THREE.DoubleSide });
    }

    // Создаём меш и добавляем на сцену
    mesh3d = new THREE.Mesh(geometry, material);
    scene.add(mesh3d);

    // Центрируем и масштабируем камеру
    geometry.computeBoundingSphere();
    if (geometry.boundingSphere) {
      const center = geometry.boundingSphere.center;
      const radius = geometry.boundingSphere.radius || 1;
      if (controls && camera) {
        controls.target.copy(center);
        camera.position.copy(center);
        camera.position.z = center.z + radius * 3;
        camera.near = radius / 100;
        camera.far = radius * 100;
        camera.updateProjectionMatrix();
        controls.update();
      }
    }
  };

  // Обработчик изменения размера окна
  const resizeHandler = () => {
    if (!renderer || !camera || !containerRef.value) return;
    const rect = containerRef.value.getBoundingClientRect();
    const width = rect.width;
    const height = rect.height;
    camera.aspect = width / height;
    camera.updateProjectionMatrix();
    renderer.setSize(width, height);
    renderer.setPixelRatio(Math.min(window.devicePixelRatio, 2));
  };

  // Жизненный цикл
  onMounted(() => {
    initScene();
    window.addEventListener('resize', resizeHandler);
  });

  onUnmounted(() => {
    window.removeEventListener('resize', resizeHandler);
    if (renderer) renderer.dispose();
    if (controls) controls.dispose();
    // Очистка mesh3d и scene будет выполнена в renderMesh при новой загрузке
    scene = null;
    camera = null;
    renderer = null;
    controls = null;
    mesh3d = null;
  });

  return {
    renderMesh,
    // Если потребуется доступ к сцене/камере извне
    // scene: () => scene,
    // camera: () => camera,
  };
}
