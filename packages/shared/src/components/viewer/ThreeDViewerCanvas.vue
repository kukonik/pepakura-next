<script setup lang="ts">
import { onMounted, onBeforeUnmount, ref, computed } from "vue";
import * as THREE from "three";
import { OBJLoader } from "three/examples/jsm/loaders/OBJLoader.js";
import { MTLLoader } from "three/examples/jsm/loaders/MTLLoader.js";
import { OrbitControls } from "three/examples/jsm/controls/OrbitControls.js";
import type { LocaleCode } from "../../i18n/messages";
import { useI18nShared } from "../../i18n/useI18nShared";

const props = defineProps<{
  locale: { value: LocaleCode };
}>();

const { t } = useI18nShared(props.locale);

const containerRef = ref<HTMLDivElement | null>(null);
const canvasRef = ref<HTMLCanvasElement | null>(null);

let renderer: THREE.WebGLRenderer | null = null;
let scene: THREE.Scene | null = null;
let camera: THREE.PerspectiveCamera | null = null;
let controls: OrbitControls | null = null;
let animationId: number | null = null;
let model: THREE.Object3D | null = null;

const isFullscreen = computed(() => {
  if (typeof document === "undefined") return false;
  const el = document.fullscreenElement as HTMLElement | null;
  return !!el && containerRef.value === el;
});

// ВРЕМЕННО: прототип загружает только встроенные модели из кода/проекта
// Внешние файлы пользователя (drag&drop, диалог выбора) пока не поддерживаются

function initScene() {
  if (!containerRef.value || !canvasRef.value) return;

  const width = containerRef.value.clientWidth || 1;
  const height = containerRef.value.clientHeight || 1;

  renderer = new THREE.WebGLRenderer({
    canvas: canvasRef.value,
    antialias: true,
  });
  renderer.setSize(width, height);
  renderer.setPixelRatio(window.devicePixelRatio || 1);
  renderer.setClearColor(0x020617, 1);

  scene = new THREE.Scene();
  scene.background = new THREE.Color(0x020617);

  camera = new THREE.PerspectiveCamera(45, width / height, 0.1, 1000);
  camera.position.set(0, 1.5, 3);

  const light = new THREE.DirectionalLight(0xffffff, 1);
  light.position.set(3, 5, 2);
  scene.add(light);

  const ambient = new THREE.AmbientLight(0xffffff, 0.35);
  scene.add(ambient);

  const grid = new THREE.GridHelper(4, 8, 0x444444, 0x222222);
  grid.position.y = -0.5;
  scene.add(grid);

  const axes = new THREE.AxesHelper(1.2);
  scene.add(axes);

  controls = new OrbitControls(camera, canvasRef.value);
  controls.enableDamping = true;
  controls.dampingFactor = 0.08;
  controls.minDistance = 0.8;
  controls.maxDistance = 10;
  controls.target.set(0, 0, 0);

  animate();
}

function clearModel() {
  if (scene && model) {
    scene.remove(model);
    model.traverse((obj) => {
      const mesh = obj as THREE.Mesh;
      if (mesh.geometry) mesh.geometry.dispose();
      if (Array.isArray(mesh.material)) {
        mesh.material.forEach((m) => m.dispose());
      } else if (mesh.material) {
        mesh.material.dispose();
      }
    });
  }
  model = null;
}

function animate() {
  if (!renderer || !scene || !camera) return;

  animationId = window.requestAnimationFrame(animate);

  if (controls) {
    controls.update();
  }

  renderer.render(scene, camera);
}

function handleResize() {
  if (!renderer || !camera || !containerRef.value) return;
  const width = containerRef.value.clientWidth || 1;
  const height = containerRef.value.clientHeight || 1;
  camera.aspect = width / height;
  camera.updateProjectionMatrix();
  renderer.setSize(width, height);
}

function toggleFullscreen() {
  if (!containerRef.value) return;
  if (!document.fullscreenElement) {
    containerRef.value.requestFullscreen?.();
  } else {
    document.exitFullscreen?.();
  }
}

onMounted(() => {
  initScene();
  handleResize();
  window.addEventListener("resize", handleResize);
});

onBeforeUnmount(() => {
  if (animationId != null) {
    cancelAnimationFrame(animationId);
  }
  window.removeEventListener("resize", handleResize);
  clearModel();
  if (renderer) {
    renderer.dispose();
  }
  renderer = null;
  scene = null;
  camera = null;
  if (controls) {
    controls.dispose();
  }
  controls = null;
});
</script>

<template>
  <div ref="containerRef" class="viewer-root">
    <div class="viewer-canvas-wrapper">
      <canvas ref="canvasRef" class="viewer-canvas"></canvas>

      <div class="viewer-overlay">
        <div class="overlay-toolbar">
          <button type="button" class="neon-button neon-small" @click="toggleFullscreen">
            {{ isFullscreen ? t("viewer.exitFullscreen") : t("viewer.fullscreen") }}
          </button>
        </div>

        <div class="overlay-warning">
          <p class="warning-title">Предупреждение</p>
          <p class="warning-text">
            Сейчас просмотрщик открывает только встроенные примеры моделей из папки программы.
            Загрузка своих файлов 3D‑моделей пока недоступна.
          </p>
        </div>

        <div class="overlay-info">
          <span>{{ t("viewer.orbitHint") }}</span>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.viewer-root {
  width: 100%;
  height: 100%;
  position: relative;
  background-color: #020617;
}

.viewer-canvas-wrapper {
  position: relative;
  width: 100%;
  height: 100%;
}

.viewer-canvas {
  width: 100%;
  height: 100%;
  display: block;
}

.viewer-overlay {
  position: absolute;
  inset: 0;
  pointer-events: none;
}

/* Кнопка fullscreen как неоновый оверлей */
.overlay-toolbar {
  position: absolute;
  top: 10px;
  right: 10px;
  pointer-events: auto;
}

.neon-button {
  position: relative;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: 6px;
  padding: 4px 10px;
  font-size: 11px;
  border-radius: 999px;
  border: 1px solid rgba(56, 189, 248, 0.7);
  color: #e0f2fe;
  background: radial-gradient(circle at 0 0, rgba(59, 130, 246, 0.3), transparent),
    radial-gradient(circle at 100% 100%, rgba(45, 212, 191, 0.2), transparent),
    rgba(15, 23, 42, 0.9);
  cursor: pointer;
  text-shadow: 0 0 4px rgba(56, 189, 248, 0.9);
  box-shadow:
    0 0 8px rgba(56, 189, 248, 0.6),
    0 0 16px rgba(45, 212, 191, 0.4),
    inset 0 0 4px rgba(15, 23, 42, 0.9);
  transition:
    box-shadow 0.18s ease-out,
    transform 0.18s ease-out,
    background 0.18s ease-out;
  pointer-events: auto;
}

.neon-button::before {
  content: "";
  position: absolute;
  inset: -2px;
  border-radius: inherit;
  background: radial-gradient(circle, rgba(56, 189, 248, 0.25), transparent 70%);
  opacity: 0;
  transition: opacity 0.18s ease-out;
  z-index: -1;
}

.neon-button:hover {
  transform: translateY(-1px);
  box-shadow:
    0 0 12px rgba(56, 189, 248, 0.9),
    0 0 24px rgba(45, 212, 191, 0.8),
    inset 0 0 4px rgba(15, 23, 42, 1);
}

.neon-button:hover::before {
  opacity: 1;
}

.neon-small {
  padding-inline: 8px;
}

/* Простое понятное предупреждение */
.overlay-warning {
  position: absolute;
  left: 50%;
  bottom: 18px;
  transform: translateX(-50%);
  max-width: 420px;
  padding: 8px 10px;
  border-radius: 8px;
  background-color: rgba(15, 23, 42, 0.95);
  border: 1px solid rgba(148, 163, 184, 0.8);
  color: #e5e7eb;
  font-size: 11px;
  pointer-events: auto;
}

.warning-title {
  margin: 0 0 2px 0;
  font-weight: 600;
  font-size: 11px;
}

.warning-text {
  margin: 0;
  font-size: 11px;
}

/* Подсказка камеры */
.overlay-info {
  position: absolute;
  left: 10px;
  top: 10px;
  pointer-events: auto;
  font-size: 11px;
  padding: 4px 8px;
  border-radius: 4px;
  border: 1px solid #1e293b;
  background-color: rgba(15, 23, 42, 0.85);
  color: #e5e7eb;
}
</style>
