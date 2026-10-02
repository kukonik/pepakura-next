<template>
  <div ref="containerRef" style="width: 100%; height: 100%; overflow: hidden; background-color: #1e1e1e;"></div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted, watch } from 'vue';
import * as THREE from 'three';
import { OBJLoader } from 'three/examples/jsm/loaders/OBJLoader.js';
import { MTLLoader } from 'three/examples/jsm/loaders/MTLLoader.js';
import { OrbitControls } from 'three/examples/jsm/controls/OrbitControls.js';

interface Props {
  meshData: string | null;
  mtlData: string | null;
  textureMap: Record<string, string> | null;
  textureMappingMode?: 'strict' | 'heuristic';
}

const props = withDefaults(defineProps<Props>(), {
  meshData: null,
  mtlData: null,
  textureMap: () => null,
  textureMappingMode: 'heuristic'
});

const containerRef = ref<HTMLDivElement | null>(null);

let scene: THREE.Scene;
let camera: THREE.PerspectiveCamera;
let renderer: THREE.WebGLRenderer;
let controls: OrbitControls;
let animationFrameId: number;
let currentModel: THREE.Group | null = null;

const init = () => {
  if (!containerRef.value) return;

  scene = new THREE.Scene();
  scene.background = new THREE.Color(0x1e1e1e);

  const aspect = containerRef.value.clientWidth / containerRef.value.clientHeight;
  camera = new THREE.PerspectiveCamera(45, aspect, 0.1, 1000);
  camera.position.set(0, 5, 15);

  renderer = new THREE.WebGLRenderer({ antialias: true });
  renderer.setSize(containerRef.value.clientWidth, containerRef.value.clientHeight);
  renderer.setPixelRatio(window.devicePixelRatio);
  renderer.outputColorSpace = THREE.SRGBColorSpace;
  renderer.toneMapping = THREE.ACESFilmicToneMapping;
  renderer.toneMappingExposure = 1.0;
  containerRef.value.appendChild(renderer.domElement);

  controls = new OrbitControls(camera, renderer.domElement);
  controls.enableDamping = true;
  controls.dampingFactor = 0.05;

  const ambientLight = new THREE.AmbientLight(0xffffff, 0.8);
  scene.add(ambientLight);

  const dirLight1 = new THREE.DirectionalLight(0xffffff, 1.0);
  dirLight1.position.set(5, 10, 7);
  scene.add(dirLight1);

  const dirLight2 = new THREE.DirectionalLight(0xffffff, 0.4);
  dirLight2.position.set(-5, -5, -5);
  scene.add(dirLight2);

  const gridHelper = new THREE.GridHelper(20, 20, 0x888888, 0x444444);
  scene.add(gridHelper);

  const axesHelper = new THREE.AxesHelper(5);
  scene.add(axesHelper);

  animate();
};

const animate = () => {
  animationFrameId = requestAnimationFrame(animate);
  controls.update();
  renderer.render(scene, camera);
};

const disposeModel = (obj: THREE.Object3D) => {
  obj.traverse((child) => {
    if ((child as THREE.Mesh).isMesh) {
      const mesh = child as THREE.Mesh;
      mesh.geometry.dispose();
      if (Array.isArray(mesh.material)) {
        mesh.material.forEach(mat => mat.dispose());
      } else {
        mesh.material.dispose();
      }
    }
  });
};
const loadModel = async (objText: string, mtlText: string | null, texMap: Record<string, string> | null) => {
  if (currentModel) {
    scene.remove(currentModel);
    disposeModel(currentModel);
    currentModel = null;
  }

  let parsedObj: THREE.Group;

  if (mtlText) {
    const mtlLoader = new MTLLoader();
    const materials = mtlLoader.parse(mtlText, '');
    materials.preload();
    parsedObj = new OBJLoader().setMaterials(materials).parse(objText);
  } else {
    parsedObj = new OBJLoader().parse(objText);
  }

  // Collect all texture loading promises
  const texturePromises: Promise<void>[] = [];

  if (texMap && Object.keys(texMap).length > 0) {
    const texKeys = Object.keys(texMap);
    const isHeuristic = props.textureMappingMode === 'heuristic';

    parsedObj.traverse((child) => {
      if ((child as THREE.Mesh).isMesh) {
        const mesh = child as THREE.Mesh;
        const mat = mesh.material as THREE.MeshPhongMaterial;

        if (!mesh.geometry.attributes.uv) return;

        let targetUrl: string | null = null;

        if (mat.name) {
          if (texMap[mat.name]) {
            targetUrl = texMap[mat.name];
          }

          if (!targetUrl && isHeuristic) {
            const nameTokens = mat.name.toLowerCase().split(/[^a-z0-9]+/).filter(t => t.length >= 3);
            for (const key of texKeys) {
              const keyLower = key.toLowerCase();
              for (const token of nameTokens) {
                if (keyLower === token || keyLower.includes(token) || token.includes(keyLower)) {
                  targetUrl = texMap[key];
                  break;
                }
              }
              if (targetUrl) break;
            }
          }
        }

        if (!targetUrl && isHeuristic && texKeys.length === 1 && !mat.map) {
          targetUrl = texMap[texKeys[0]];
        }

        if (targetUrl) {
          // Create promise for this texture
          const promise = new Promise<void>((resolve) => {
            const img = new Image();
            img.onload = () => {
              const tex = new THREE.Texture(img);
              tex.colorSpace = THREE.SRGBColorSpace;
              tex.needsUpdate = true;

              // UV normalization
              const uvs = mesh.geometry.attributes.uv;
              let maxU = -Infinity, maxV = -Infinity;
              for (let i = 0; i < uvs.count; i++) {
                const u = uvs.getX(i);
                const v = uvs.getY(i);
                if (u > maxU) maxU = u;
                if (v > maxV) maxV = v;
              }

              if (maxU > 1.0 || maxV > 1.0) {
                const texW = img.width;
                const texH = img.height;
                const C = maxU > 0 ? texW / maxU : 1;

                for (let i = 0; i < uvs.count; i++) {
                  const u = uvs.getX(i);
                  const v = uvs.getY(i);
                  const uNorm = u * (C / texW);
                  const vNorm = v * (C / texH);
                  uvs.setXY(i, uNorm, vNorm);
                }
                uvs.needsUpdate = true;
              }

              mat.map = tex;
              mat.color.set(0xffffff);

              if (mat.name?.toLowerCase().includes('eye')) {
                mat.transparent = true;
                mat.alphaTest = 0.1;
              }

              mat.needsUpdate = true;
              resolve();
            };
            img.onerror = () => resolve(); // Resolve even on error to not block
            img.src = targetUrl!;
          });
          texturePromises.push(promise);
        }
      }
    });
  }

  // Wait for all textures to load before adding model to scene
  await Promise.all(texturePromises);

  const box = new THREE.Box3().setFromObject(parsedObj);
  const center = box.getCenter(new THREE.Vector3());
  const size = box.getSize(new THREE.Vector3());
  const maxDim = Math.max(size.x, size.y, size.z);
  const scale = maxDim > 0 ? 10 / maxDim : 1;

  parsedObj.scale.setScalar(scale);
  parsedObj.position.x -= center.x * scale;
  parsedObj.position.z -= center.z * scale;
  parsedObj.position.y -= box.min.y * scale;

  scene.add(parsedObj);
  currentModel = parsedObj;

  controls.target.set(0, (size.y * scale) / 2, 0);
  camera.position.set(0, (size.y * scale) / 2, 15);
  controls.update();
};;

const handleResize = () => {
  if (!containerRef.value) return;
  const width = containerRef.value.clientWidth;
  const height = containerRef.value.clientHeight;
  camera.aspect = width / height;
  camera.updateProjectionMatrix();
  renderer.setSize(width, height);
};

watch([() => props.meshData, () => props.mtlData, () => props.textureMap], () => { if (props.meshData) { loadModel(props.meshData, props.mtlData, props.textureMap); } });

onMounted(() => {
  init();
  window.addEventListener('resize', handleResize);
  if (props.meshData) {
    loadModel(props.meshData, props.mtlData, props.textureMap);
  }
});

onUnmounted(() => {
  window.removeEventListener('resize', handleResize);
  if (animationFrameId) cancelAnimationFrame(animationFrameId);
  if (currentModel) disposeModel(currentModel);
  if (renderer) renderer.dispose();
});
</script>


