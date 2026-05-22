# ============================================================================
# FINAL FIX: Separation of Grid from Model (Fixes Reset/Clear)
# ============================================================================

 $ProjectRoot = "D:\Dev\pepakura-next"
 $TargetFile = "$ProjectRoot\packages\shared\src\components\viewer\ThreeDViewer.vue"

Write-Host "=== Final Fix: Separating Helpers ===" -ForegroundColor Cyan

 $Content = @'
<template>
  <div ref="containerRef" class="viewer-container">
    <canvas ref="canvasRef" class="viewer-canvas"></canvas>
    <div class="overlay-top"><slot name="top"></slot></div>
    <div class="overlay-bottom"><slot name="bottom"></slot></div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted, defineExpose } from 'vue';
import * as THREE from 'three';
import { OrbitControls } from 'three/examples/jsm/controls/OrbitControls.js';
import { OBJLoader } from 'three/examples/jsm/loaders/OBJLoader.js';
import { MTLLoader } from 'three/examples/jsm/loaders/MTLLoader.js';
import { SimplifyModifier } from 'three/examples/jsm/modifiers/SimplifyModifier.js';

const containerRef = ref<HTMLDivElement>();
const canvasRef = ref<HTMLCanvasElement>();

let scene: THREE.Scene;
let camera: THREE.PerspectiveCamera;
let renderer: THREE.WebGLRenderer;
let controls: OrbitControls;
let animationFrameId: number;
let ambientLight: THREE.AmbientLight;
let dirLight: THREE.DirectionalLight;
let currentModel: THREE.Group | null = null;

// Группа для сетки и осей (отделена от модели)
let helpersGroup: THREE.Group;

onMounted(() => {
  initThree();
  window.addEventListener('resize', onWindowResize);
  if (containerRef.value) {
    const resizeObserver = new ResizeObserver(() => onWindowResize());
    resizeObserver.observe(containerRef.value);
    (containerRef.value as any)._resizeObserver = resizeObserver;
  }
});

onUnmounted(() => {
  cancelAnimationFrame(animationFrameId);
  window.removeEventListener('resize', onWindowResize);
  if (containerRef.value && (containerRef.value as any)._resizeObserver) {
    (containerRef.value as any)._resizeObserver.disconnect();
  }
  disposeScene();
});

function initThree() {
  if (!containerRef.value || !canvasRef.value) return;
  const width = containerRef.value.clientWidth;
  const height = containerRef.value.clientHeight;

  scene = new THREE.Scene();
  scene.background = new THREE.Color(0x1a1a1a);

  camera = new THREE.PerspectiveCamera(45, width / height, 0.1, 1000);
  camera.position.set(5, 5, 5);

  renderer = new THREE.WebGLRenderer({ canvas: canvasRef.value, antialias: true, alpha: false });
  renderer.setSize(width, height);
  renderer.setPixelRatio(Math.min(window.devicePixelRatio, 2));
  renderer.shadowMap.enabled = true;

  controls = new OrbitControls(camera, renderer.domElement);
  controls.enableDamping = true;
  controls.dampingFactor = 0.05;
  controls.screenSpacePanning = false;

  ambientLight = new THREE.AmbientLight(0xffffff, 1.0);
  scene.add(ambientLight);

  dirLight = new THREE.DirectionalLight(0xffffff, 1.0);
  dirLight.position.set(5, 10, 7);
  scene.add(dirLight);

  // Создаем отдельную группу для Grid и Axes
  helpersGroup = new THREE.Group();
  const gridHelper = new THREE.GridHelper(20, 20, 0x555555, 0x333333);
  const axesHelper = new THREE.AxesHelper(2);
  helpersGroup.add(gridHelper);
  helpersGroup.add(axesHelper);
  scene.add(helpersGroup);

  animate();
}

function animate() {
  animationFrameId = requestAnimationFrame(animate);
  controls.update();
  renderer.render(scene, camera);
}

function onWindowResize() {
  if (!containerRef.value || !camera || !renderer) return;
  const width = containerRef.value.clientWidth;
  const height = containerRef.value.clientHeight;
  camera.aspect = width / height;
  camera.updateProjectionMatrix();
  renderer.setSize(width, height);
}

// --- Public Methods ---

async function loadObjFromFiles(files: FileList | File[]) {
  if (!scene) return;
  clearModel();
  const fileArray = Array.from(files);
  const objFile = fileArray.find(f => f.name.toLowerCase().endsWith('.obj'));
  const mtlFile = fileArray.find(f => f.name.toLowerCase().endsWith('.mtl'));

  if (!objFile) { alert('OBJ файл не выбран'); return; }

  const textDecoder = new TextDecoder();
  try {
    let materials = null;
    if (mtlFile) {
      const mtlText = textDecoder.decode(await mtlFile.arrayBuffer());
      const cleanMtl = mtlText.replace(/(map_Kd|map_Ks|map_Ns|map_d|bump|disp)\s+([^\s\r\n]+)/gi, '$1 .');
      const mtlLoader = new MTLLoader();
      materials = mtlLoader.parse(cleanMtl);
      materials.preload();
    }
    const objLoader = new OBJLoader();
    if (materials) objLoader.setMaterials(materials);
    const objText = textDecoder.decode(await objFile.arrayBuffer());
    const object = objLoader.parse(objText);
    addModelToScene(object);
  } catch (e) {
    console.error("LOAD ERROR", e);
    alert("Ошибка загрузки: " + e);
  }
}

async function loadObjFromUrl(objUrl: string, mtlUrl: string | null = null) {
  if (!scene) return;
  clearModel();
  try {
    let materials = null;
    if (mtlUrl) {
      const mtlResp = await fetch(mtlUrl);
      const mtlText = await mtlResp.text();
      const cleanMtl = mtlText.replace(/(map_Kd|map_Ks|map_Ns|map_d|bump|disp)\s+([^\s\r\n]+)/gi, '$1 .');
      const mtlLoader = new MTLLoader();
      materials = mtlLoader.parse(cleanMtl);
      materials.preload();
    }
    const objResp = await fetch(objUrl);
    const objText = await objResp.text();
    const objLoader = new OBJLoader();
    if (materials) objLoader.setMaterials(materials);
    const object = objLoader.parse(objText);
    addModelToScene(object);
  } catch (e) { console.error(e); alert('Ошибка загрузки URL'); }
}

function addModelToScene(object: THREE.Group) {
  currentModel = object;
  const box = new THREE.Box3().setFromObject(object);
  const center = box.getCenter(new THREE.Vector3());
  const size = box.getSize(new THREE.Vector3());

  object.position.x += (object.position.x - center.x);
  object.position.y += (object.position.y - center.y);
  object.position.z += (object.position.z - center.z);
  object.position.y += size.y / 2;

  object.traverse((child) => {
    if (child instanceof THREE.Mesh) {
      child.geometry.computeVertexNormals();
      if (Array.isArray(child.material)) child.material.forEach(m => m.side = THREE.DoubleSide);
      else child.material.side = THREE.DoubleSide;
    }
  });

  scene.add(object);

  const maxDim = Math.max(size.x, size.y, size.z);
  if (maxDim === 0) return;
  const fov = camera.fov * (Math.PI / 180);
  let cameraZ = Math.abs(maxDim / 2 * Math.tan(fov * 2));
  cameraZ *= 2.5;
  camera.position.set(cameraZ, cameraZ * 0.8, cameraZ);
  camera.lookAt(0, 0, 0);
  controls.target.set(0, 0, 0);
  
  controls.update();
  renderer.render(scene, camera);
}

function clearModel() {
  // Удаляем ТОЛЬКО модель. helpersGroup нетрогаем!
  if (currentModel) {
    scene.remove(currentModel);
    currentModel.traverse((child) => {
      if (child instanceof THREE.Mesh) {
        child.geometry.dispose();
        if (Array.isArray(child.material)) child.material.forEach(m => m.dispose());
        else child.material.dispose();
      }
    });
    currentModel = null;
  }
}

function disposeScene() {
  clearModel();
  if (helpersGroup) scene.remove(helpersGroup);
  if (renderer) renderer.dispose();
  if (controls) controls.dispose();
}

function resetCamera() {
  if (!camera || !controls || !renderer) { return; }
  camera.position.set(5, 5, 5);
  camera.lookAt(0, 0, 0);
  controls.target.set(0, 0, 0);
  controls.update();
  renderer.render(scene, camera);
}

function resetFiles() {
  clearModel();
  resetCamera();
}

function updateLights(ambient: number, dir: number) {
  if (ambientLight) ambientLight.intensity = ambient;
  if (dirLight) dirLight.intensity = dir;
}

function updateModelScale(scale: number) {
  if (currentModel) {
    currentModel.scale.set(scale, scale, scale);
    const box = new THREE.Box3().setFromObject(currentModel);
    const size = box.getSize(new THREE.Vector3());
    currentModel.position.y = size.y / 2;
  }
}

function simplifyModel(factor: number) {
  if (!currentModel) return { total: 0, removed: 0 };
  let totalVerts = 0; let removedVerts = 0;
  currentModel.traverse((child) => {
    if (child instanceof THREE.Mesh) {
      const oldGeo = child.geometry;
      totalVerts += oldGeo.attributes.position.count;
      const targetCount = Math.floor(oldGeo.attributes.position.count * factor);
      if (targetCount < oldGeo.attributes.position.count) {
        try {
          const modifier = new SimplifyModifier();
          child.geometry = modifier.modify(oldGeo, targetCount);
          removedVerts += (oldGeo.attributes.position.count - targetCount);
        } catch (e) {}
      }
    }
  });
  return { total: totalVerts, removed: removedVerts };
}

function loadTestCube() {
  if (!scene) return;
  clearModel();
  const geometry = new THREE.BoxGeometry(2, 2, 2);
  const material = new THREE.MeshBasicMaterial({ color: 0x00ff00, wireframe: true });
  const cube = new THREE.Mesh(geometry, material);
  cube.position.set(0, 1, 0);
  scene.add(cube);
  currentModel = cube;
  controls.update();
  renderer.render(scene, camera);
}

defineExpose({
  loadObjFromFiles,
  loadObjFromUrl,
  resetCamera,
  resetFiles,
  updateLights,
  updateModelScale,
  simplifyModel,
  loadTestCube
});
</script>

<style scoped>
.viewer-container { position: relative; width: 100%; height: 100%; overflow: hidden; background-color: #1a1a1a; }
.viewer-canvas { display: block; width: 100%; height: 100%; outline: none; }
.overlay-top { position: absolute; top: 0; left: 0; width: 100%; padding: 1rem; pointer-events: none; display: flex; justify-content: space-between; align-items: flex-start; }
.overlay-bottom { position: absolute; bottom: 0; left: 0; width: 100%; padding: 1rem; pointer-events: none; display: flex; justify-content: center; align-items: flex-end; gap: 1rem; }
.overlay-top > *, .overlay-bottom > * { pointer-events: auto; }
</style>
'@

Set-Content -Path $TargetFile -Value $Content -Encoding UTF8

Write-Host "   [OK] File updated." -ForegroundColor Green
Write-Host "Теперь запусти браузер и перезагрузи страницу (Ctrl+Shift+R)." -ForegroundColor Yellow