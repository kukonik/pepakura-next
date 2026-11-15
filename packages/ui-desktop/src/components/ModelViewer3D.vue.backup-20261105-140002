<template>
  <div class="viewer-container" ref="containerRef">
    <div v-if="error" class="error-overlay">{{ error }}</div>
    <div v-if="isLoading" class="loading-overlay">Парсинг OBJ...</div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch, onMounted, onUnmounted } from 'vue';
import * as THREE from 'three';
import { OBJLoader } from 'three/examples/jsm/loaders/OBJLoader.js';
import { OrbitControls } from 'three/examples/jsm/controls/OrbitControls.js';

const props = defineProps<{ meshData: string }>();
const containerRef = ref<HTMLDivElement | null>(null);
const error = ref('');
const isLoading = ref(false);

let scene: THREE.Scene, camera: THREE.PerspectiveCamera, renderer: THREE.WebGLRenderer, controls: OrbitControls;
let currentModel: THREE.Group | null = null;
let animationFrameId = 0;

const sanitizeObj = (text: string) => text.replace(/^V /gm, 'v ').replace(/^F /gm, 'f ');

onMounted(() => {
  if (!containerRef.value) return;
  scene = new THREE.Scene(); scene.background = new THREE.Color(0x1e1e1e);
  camera = new THREE.PerspectiveCamera(45, containerRef.value.clientWidth / containerRef.value.clientHeight, 0.1, 1000); camera.position.z = 5;
  renderer = new THREE.WebGLRenderer({ antialias: true }); renderer.setSize(containerRef.value.clientWidth, containerRef.value.clientHeight);
  containerRef.value.appendChild(renderer.domElement);
  controls = new OrbitControls(camera, renderer.domElement);
  scene.add(new THREE.AmbientLight(0xffffff, 0.6));
  scene.add(new THREE.DirectionalLight(0xffffff, 0.8).translateZ(5));
  const animate = () => { animationFrameId = requestAnimationFrame(animate); controls.update(); renderer.render(scene, camera); };
  animate();
});

watch(() => props.meshData, (newData) => {
  if (!renderer || !newData) return;
  isLoading.value = true; error.value = '';
  if (currentModel) {
    scene.remove(currentModel);
    currentModel.traverse((child) => { if ((child as THREE.Mesh).isMesh) { child.geometry.dispose(); if (child.material instanceof THREE.Material) child.material.dispose(); } });
    currentModel = null;
  }
  try {
    const sanitized = sanitizeObj(newData);
    const obj = new OBJLoader().parse(sanitized);
    obj.traverse((child) => { if ((child as THREE.Mesh).isMesh) { (child as THREE.Mesh).material = new THREE.MeshStandardMaterial({ color: 0x007acc }); } });
    currentModel = obj; scene.add(currentModel);
    const box = new THREE.Box3().setFromObject(currentModel); const center = box.getCenter(new THREE.Vector3()); const size = box.getSize(new THREE.Vector3());
    currentModel.position.sub(center); currentModel.scale.multiplyScalar(3 / Math.max(size.x, size.y, size.z));
  } catch (e: any) { error.value = 'Ошибка формата OBJ: ' + e.message; } finally { isLoading.value = false; }
}, { flush: 'post' });

onUnmounted(() => { cancelAnimationFrame(animationFrameId); renderer?.dispose(); });
</script>

<style scoped>
.viewer-container { width: 100%; height: 100%; position: relative; background: #1e1e1e; }
.error-overlay { position: absolute; top: 50%; left: 50%; transform: translate(-50%, -50%); color: red; background: rgba(0,0,0,0.8); padding: 10px; }
.loading-overlay { position: absolute; top: 10px; left: 10px; color: #fff; font-family: monospace; }
</style>
