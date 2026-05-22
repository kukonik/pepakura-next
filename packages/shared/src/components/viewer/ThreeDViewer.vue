<template>
  <div ref="containerRef" class="viewer-container">
    <canvas ref="canvasRef" class="viewer-canvas"></canvas>
    <div class="overlay-top"><slot name="top"></slot></div>
    <div class="overlay-bottom"><slot name="bottom"></slot></div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted, defineExpose } from "vue";
import * as THREE from "three";
import { OrbitControls } from "three/examples/jsm/controls/OrbitControls.js";
import { OBJLoader } from "three/examples/jsm/loaders/OBJLoader.js";
import { MTLLoader } from "three/examples/jsm/loaders/MTLLoader.js";
import { SimplifyModifier } from "three/examples/jsm/modifiers/SimplifyModifier.js";
import { GLTFLoader } from "three/examples/jsm/loaders/GLTFLoader.js";
import { FBXLoader } from "three/examples/jsm/loaders/FBXLoader.js";
import { STLLoader } from "three/examples/jsm/loaders/STLLoader.js";
import { PLYLoader } from "three/examples/jsm/loaders/PLYLoader.js";
import { ColladaLoader } from "three/examples/jsm/loaders/ColladaLoader.js";
import { TDSLoader } from "three/examples/jsm/loaders/TDSLoader.js";
import { useUnfoldState } from "../../composables/useUnfoldState";

const containerRef = ref<HTMLDivElement>();
const canvasRef = ref<HTMLCanvasElement>();

const unfoldState = useUnfoldState();

let scene: THREE.Scene;
let camera: THREE.PerspectiveCamera;
let renderer: THREE.WebGLRenderer;
let controls: OrbitControls;
let animationFrameId: number;

let ambientLight: THREE.AmbientLight;
let dirLight: THREE.DirectionalLight;
let helpersGroup: THREE.Group;
let currentModel: THREE.Object3D | null = null;

const seamMaterial = new THREE.LineBasicMaterial({
  color: 0xff0000,
  depthTest: false,
  transparent: true,
  opacity: 0.9,
  linewidth: 1,
});

let seamLines: THREE.LineSegments | null = null;

let fileMap: Map<string, File> = new Map();
let objectUrls: string[] = [];

const modelMeta = ref({
  filename: "",
  scale: 1.0,
});

onMounted(() => {
  initThree();
  window.addEventListener("resize", onWindowResize);
  const canvas = canvasRef.value;
  if (canvas) canvas.addEventListener("dblclick", onCanvasDoubleClick);
  if (containerRef.value) {
    const resizeObserver = new ResizeObserver(() => onWindowResize());
    resizeObserver.observe(containerRef.value);
    (containerRef.value as any)._resizeObserver = resizeObserver;
  }
});

onUnmounted(() => {
  const canvas = canvasRef.value;
  if (canvas) canvas.removeEventListener("dblclick", onCanvasDoubleClick);
  cancelAnimationFrame(animationFrameId);
  window.removeEventListener("resize", onWindowResize);
  if (containerRef.value && (containerRef.value as any)._resizeObserver) {
    (containerRef.value as any)._resizeObserver.disconnect();
  }
  disposeScene();
  cleanupObjectUrls();
});

function initThree() {
  if (!containerRef.value || !canvasRef.value) return;
  const width = containerRef.value.clientWidth;
  const height = containerRef.value.clientHeight;

  scene = new THREE.Scene();
  scene.background = new THREE.Color(0x1a1a1a);

  camera = new THREE.PerspectiveCamera(45, 1, 0.1, 10000);
  camera.position.set(5, 5, 5);

  renderer = new THREE.WebGLRenderer({
    canvas: canvasRef.value,
    antialias: true,
    alpha: false,
  });
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

  helpersGroup = new THREE.Group();
  const gridHelper = new THREE.GridHelper(20, 20, 0x555555, 0x333333);
  const axesHelper = new THREE.AxesHelper(2);
  helpersGroup.add(gridHelper);
  helpersGroup.add(axesHelper);
  scene.add(helpersGroup);

  animate();
  onWindowResize();
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
  if (width <= 0 || height <= 0) return;
  camera.aspect = width / height;
  camera.updateProjectionMatrix();
  renderer.setSize(width, height);
}

function buildFileMap(files: File[]) {
  fileMap.clear();
  files.forEach((f) => {
    fileMap.set(f.name, f);
  });
}

function cleanupObjectUrls() {
  objectUrls.forEach((url) => URL.revokeObjectURL(url));
  objectUrls = [];
}

function createManagerForFileMap(): THREE.LoadingManager {
  const manager = new THREE.LoadingManager();
  manager.setURLModifier((url: string) => {
    const normalized = url.split(/[/\\]/).pop() || url;
    const file = fileMap.get(normalized);
    if (file) {
      const blobUrl = URL.createObjectURL(file);
      objectUrls.push(blobUrl);
      return blobUrl;
    }
    return url;
  });
  return manager;
}

function getClosestPointOnSegment(
  p: THREE.Vector3,
  a: THREE.Vector3,
  b: THREE.Vector3
): THREE.Vector3 {
  const ab = new THREE.Vector3().subVectors(b, a);
  let t = p.clone().sub(a).dot(ab) / ab.dot(ab);
  t = Math.max(0, Math.min(1, t));
  return a.clone().add(ab.multiplyScalar(t));
}

function getClosestEdge(
  point: THREE.Vector3,
  vA: THREE.Vector3,
  vB: THREE.Vector3,
  vC: THREE.Vector3
): [number, number] {
  const dAB = point.distanceToSquared(getClosestPointOnSegment(point, vA, vB));
  const dBC = point.distanceToSquared(getClosestPointOnSegment(point, vB, vC));
  const dCA = point.distanceToSquared(getClosestPointOnSegment(point, vC, vA));
  if (dAB < dBC && dAB < dCA) return [0, 1];
  if (dBC < dAB && dBC < dCA) return [1, 2];
  return [2, 0];
}

function onCanvasDoubleClick(event: MouseEvent) {
  if (!currentModel || !camera || !renderer) return;
  const rect = canvasRef.value!.getBoundingClientRect();
  const mouse = new THREE.Vector2();
  mouse.x = ((event.clientX - rect.left) / rect.width) * 2 - 1;
  mouse.y = -((event.clientY - rect.top) / rect.height) * 2 + 1;
  const raycaster = new THREE.Raycaster();
  raycaster.setFromCamera(mouse, camera);

  const roots = currentModel.children.length
    ? currentModel.children
    : [currentModel];
  const intersects = raycaster.intersectObjects(roots, true);

  if (intersects.length === 0) return;

  const intersect = intersects[0];
  const faceIndex = intersect.faceIndex;
  if (!(intersect.object instanceof THREE.Mesh) || faceIndex === undefined) {
    return;
  }

  const geometry = intersect.object.geometry;
  const idxAttr = geometry.index;
  let v1: number, v2: number, v3: number;

  if (idxAttr) {
    const i0 = faceIndex * 3;
    v1 = idxAttr.getX(i0);
    v2 = idxAttr.getX(i0 + 1);
    v3 = idxAttr.getX(i0 + 2);
  } else {
    const i0 = faceIndex * 3;
    v1 = i0;
    v2 = i0 + 1;
    v3 = i0 + 2;
  }

  const posAttr = geometry.attributes.position;
  const p1 = new THREE.Vector3();
  const p2 = new THREE.Vector3();
  const p3 = new THREE.Vector3();

  p1.fromBufferAttribute(posAttr, v1);
  p2.fromBufferAttribute(posAttr, v2);
  p3.fromBufferAttribute(posAttr, v3);

  intersect.object.updateMatrixWorld(true);
  p1.applyMatrix4(intersect.object.matrixWorld);
  p2.applyMatrix4(intersect.object.matrixWorld);
  p3.applyMatrix4(intersect.object.matrixWorld);

  const edgeIndices = getClosestEdge(intersect.point, p1, p2, p3);
  const vertices = [v1, v2, v3];
  const idxA = vertices[edgeIndices[0]];
  const idxB = vertices[edgeIndices[1]];

  unfoldState.toggleEdge(idxA, idxB);
  updateSeamVisualization(intersect.object);
}

function updateSeamVisualization(targetMesh: THREE.Mesh) {
  if (seamLines) {
    scene.remove(seamLines);
    seamLines.geometry.dispose();
    seamLines = null;
  }

  targetMesh.updateMatrixWorld(true);

  const points: THREE.Vector3[] = [];
  const geometry = targetMesh.geometry;
  const posAttr = geometry.attributes.position;
  const box = new THREE.Box3().setFromObject(targetMesh);
  const size = new THREE.Vector3();
  box.getSize(size);
  const maxDim = Math.max(size.x, size.y, size.z);
  const offset = maxDim * 0.002;

  const cameraDir = new THREE.Vector3();
  if (camera) {
    camera.getWorldDirection(cameraDir);
    cameraDir.negate();
  }

  unfoldState.cutEdges.forEach((key) => {
    const [v1Idx, v2Idx] = key.split("_").map(Number);
    const p1 = new THREE.Vector3();
    const p2 = new THREE.Vector3();
    p1.fromBufferAttribute(posAttr, v1Idx);
    p2.fromBufferAttribute(posAttr, v2Idx);
    p1.applyMatrix4(targetMesh.matrixWorld);
    p2.applyMatrix4(targetMesh.matrixWorld);
    p1.addScaledVector(cameraDir, offset);
    p2.addScaledVector(cameraDir, offset);
    points.push(p1, p2);
  });

  if (points.length > 0) {
    const lineGeo = new THREE.BufferGeometry().setFromPoints(points);
    seamLines = new THREE.LineSegments(lineGeo, seamMaterial);
    scene.add(seamLines);
  }
}

// --- MESH + TEXTURE SERIALIZATION ---

function extractMeshData(mesh: THREE.Mesh): any {
  const geo = mesh.geometry;
  const pos = geo.attributes.position;
  const idx = geo.index;

  const processMaterial = (m: any) => {
    const data: any = {
      color: m?.color?.getHexString ? m.color.getHexString() : undefined,
      metalness: m?.metalness,
      roughness: m?.roughness,
    };

    if (m instanceof THREE.MeshStandardMaterial && m.map && m.map.image) {
      try {
        const img: any = m.map.image;
        if (img instanceof HTMLImageElement && typeof img.src === "string") {
          if (img.src.startsWith("data:") || img.src.startsWith("blob:")) {
            data.texture = img.src;
          }
        } else if (img instanceof HTMLCanvasElement) {
          data.texture = img.toDataURL("image/png");
        }
      } catch (e) {
        console.warn("Texture extraction failed", e);
      }
    }

    return data;
  };

  let matData: any = null;
  if (Array.isArray(mesh.material)) {
    matData = mesh.material.map(processMaterial);
  } else {
    matData = processMaterial(mesh.material);
  }

  return {
    position: Array.from(pos.array),
    index: idx ? Array.from(idx.array) : null,
    uv: geo.attributes.uv ? Array.from(geo.attributes.uv.array) : null,
    materials: matData,
    matrix: mesh.matrix.elements,
  };
}

function createSingleMesh(
  data: any,
  texLoader: THREE.TextureLoader
): THREE.Mesh {
  const geo = new THREE.BufferGeometry();
  const pos = new Float32Array(data.position);
  geo.setAttribute("position", new THREE.BufferAttribute(pos, 3));
  if (data.index) geo.setIndex(data.index);
  if (data.uv) {
    geo.setAttribute(
      "uv",
      new THREE.BufferAttribute(new Float32Array(data.uv), 2)
    );
  }
  geo.computeVertexNormals();

  const makeMaterial = (m: any) => {
    const mat = new THREE.MeshStandardMaterial({
      color: m?.color ? parseInt(m.color, 16) : 0x606060,
      metalness: m?.metalness ?? 0.5,
      roughness: m?.roughness ?? 0.5,
      side: THREE.DoubleSide,
    });

    if (m?.texture && typeof m.texture === "string") {
      texLoader.load(
        m.texture,
        (tex) => {
          tex.colorSpace = THREE.SRGBColorSpace;
          mat.map = tex;
          mat.needsUpdate = true;
        },
        undefined,
        (err) => {
          console.warn("Texture load failed", m.texture, err);
        }
      );
    }

    return mat;
  };

  let material: any;
  if (Array.isArray(data.materials)) {
    material = data.materials.map(makeMaterial);
  } else {
    material = makeMaterial(data.materials || {});
  }

  const mesh = new THREE.Mesh(geo, material);
  if (data.matrix) {
    mesh.matrix.fromArray(data.matrix);
    mesh.matrixAutoUpdate = false;
    mesh.updateMatrixWorld();
  } else {
    mesh.matrixAutoUpdate = true;
  }
  return mesh;
}

function createMeshFromData(
  data: any,
  texLoader: THREE.TextureLoader
): THREE.Object3D {
  if (Array.isArray(data)) {
    const group = new THREE.Group();
    data.forEach((item) => {
      const mesh = createSingleMesh(item, texLoader);
      group.add(mesh);
    });
    return group;
  }
  return createSingleMesh(data, texLoader);
}function fitCameraToModel(object: THREE.Object3D) {
  const box = new THREE.Box3().setFromObject(object);
  const size = new THREE.Vector3();
  box.getSize(size);

  if (
    !Number.isFinite(size.x) ||
    !Number.isFinite(size.y) ||
    !Number.isFinite(size.z)
  ) {
    return;
  }

  const maxDim = Math.max(size.x, size.y, size.z);
  const clampedDim = Math.min(Math.max(maxDim, 0.001), 10000);
  const distance = clampedDim * 3.0;

  camera.position.set(distance, distance * 0.8, distance);
  camera.lookAt(0, 0, 0);
  controls.target.set(0, 0, 0);
  controls.update();
  renderer.render(scene, camera);
}

function addModelToScene(object: THREE.Object3D) {
  currentModel = object;
  const box = new THREE.Box3().setFromObject(object);
  const center = new THREE.Vector3();
  const size = new THREE.Vector3();
  box.getCenter(center);
  box.getSize(size);

  if (
    !Number.isFinite(size.x) ||
    !Number.isFinite(size.y) ||
    !Number.isFinite(size.z)
  ) {
    size.set(1, 1, 1);
  }

  object.position.sub(center);
  object.position.y += size.y / 2;

  object.traverse((child: any) => {
    if (child instanceof THREE.Mesh) {
      child.geometry.computeVertexNormals();
      if (Array.isArray(child.material)) {
        child.material.forEach((m) => (m.side = THREE.DoubleSide));
      } else {
        child.material.side = THREE.DoubleSide;
      }
    }
  });

  scene.add(object);

  const maxDimRaw = Math.max(size.x, size.y, size.z);
  if (!Number.isFinite(maxDimRaw) || maxDimRaw <= 0) {
    controls.update();
    renderer.render(scene, camera);
    return;
  }

  const maxDim = Math.min(Math.max(maxDimRaw, 0.001), 10000);
  const distance = maxDim * 3.0;
  camera.position.set(distance, distance * 0.8, distance);
  camera.lookAt(0, 0, 0);
  controls.target.set(0, 0, 0);
  modelMeta.value.scale = 1.0;

  unfoldState.clearCuts();
  controls.update();
  renderer.render(scene, camera);
}

function clearModel() {
  if (currentModel) {
    scene.remove(currentModel);
    (currentModel as any).traverse((child: any) => {
      if (child instanceof THREE.Mesh) {
        child.geometry.dispose();
        if (Array.isArray(child.material)) {
          child.material.forEach((m) => m.dispose());
        } else {
          child.material.dispose();
        }
      }
    });
    currentModel = null;
  }
  if (seamLines) {
    scene.remove(seamLines);
    seamLines.geometry.dispose();
    seamLines = null;
  }
}

function disposeScene() {
  clearModel();
  if (helpersGroup) scene.remove(helpersGroup);
  if (renderer) renderer.dispose();
  if (controls) controls.dispose();
}

// --- SAVE / LOAD PNX ---

async function savePNX() {
  if (!currentModel) {
    alert("Нет модели.");
    return;
  }

  const meshes: any[] = [];
  currentModel.traverse((child: any) => {
    if (child instanceof THREE.Mesh) {
      meshes.push(extractMeshData(child));
    }
  });

  if (meshes.length === 0) {
    alert("Не найдено геометрии для сохранения.");
    return;
  }

  const state = {
    version: "3.0",
    timestamp: new Date().toISOString(),
    modelFilename: modelMeta.value.filename,
    camera: {
      position: camera.position.toArray(),
      target: controls.target.toArray(),
    },
    model: { scale: currentModel.scale.x },
    lights: { ambient: ambientLight.intensity, dir: dirLight.intensity },
    seams: Array.from(unfoldState.cutEdges),
    meshes,
  };

  const blob = new Blob([JSON.stringify(state, null, 2)], {
    type: "application/json",
  });

  const types = [
    {
      description: "Pepakura Project",
      accept: { "application/json": [".pnx"] },
    },
  ];

  const suggestedName =
    (modelMeta.value.filename || "pepakura").replace(/\.[^/.]+$/, "") +
    ".pnx";

  if ("showSaveFilePicker" in window) {
    try {
      const handle = await (window as any).showSaveFilePicker({
        suggestedName,
        types,
      });
      const writable = await handle.createWritable();
      await writable.write(blob);
      await writable.close();
      alert("Сохранено!");
      return;
    } catch (e: any) {
      if (e?.name !== "AbortError") {
        console.error("Save Picker failed", e);
      }
    }
  }

  const url = URL.createObjectURL(blob);
  const a = document.createElement("a");
  a.href = url;
  a.download = suggestedName;
  a.click();
  URL.revokeObjectURL(url);
}

function loadPNX(file: File) {
  const reader = new FileReader();
  const texLoader = new THREE.TextureLoader();

  reader.onload = (e) => {
    try {
      const content = e.target?.result as string;
      const state = JSON.parse(content);

      if (state.meshes && Array.isArray(state.meshes) && state.meshes.length > 0) {
        clearModel();
        const object = createMeshFromData(state.meshes, texLoader);
        addModelToScene(object);

        setTimeout(() => {
          unfoldState.clearCuts();
          if (state.seams && Array.isArray(state.seams)) {
            state.seams.forEach((s: string) => unfoldState.cutEdges.add(s));

            let target: THREE.Mesh | null = null;
            if (currentModel) {
              if (currentModel.children.length > 0) {
                const first = currentModel.children[0];
                if (first instanceof THREE.Mesh) target = first;
              } else if (currentModel instanceof THREE.Mesh) {
                target = currentModel;
              }
            }
            if (target) updateSeamVisualization(target);
          }

          if (state.camera && Array.isArray(state.camera.position)) {
            camera.position.fromArray(state.camera.position);
          }
          if (state.camera && Array.isArray(state.camera.target)) {
            controls.target.fromArray(state.camera.target);
          }
          if (state.model && state.model.scale) {
            updateModelScale(state.model.scale);
          }
          if (state.lights) {
            updateLights(state.lights.ambient, state.lights.dir);
          }

          if (currentModel) fitCameraToModel(currentModel);

          controls.update();
          renderer.render(scene, camera);
          alert("PNX загружен!");
        }, 200);
      } else if (state.meshData) {
        clearModel();
        const mesh = createMeshFromData(state.meshData, texLoader);
        addModelToScene(mesh);
        if (currentModel instanceof THREE.Mesh) {
          updateSeamVisualization(currentModel);
        }
      }

      if (!currentModel) {
        alert("PNX пуст или поврежден");
        return;
      }
    } catch (err) {
      alert("Ошибка чтения PNX");
      console.error(err);
    }
  };

  reader.readAsText(file);
}

// --- VIEWER CONTROLS ---

function toggleWireframe() {
  if (!currentModel) return;

  let current = false;

  if (currentModel.children.length > 0) {
    const	first = currentModel.children[0];
    if (first instanceof THREE.Mesh) {
      const mat = first.material;
      if (Array.isArray(mat)) {
        current = !!mat[0].wireframe;
      } else {
        current = !!(mat as any).wireframe;
      }
    }
  } else if (currentModel instanceof THREE.Mesh) {
    const mat = currentModel.material;
    if (Array.isArray(mat)) {
      current = !!mat[0].wireframe;
    } else {
      current = !!(mat as any).wireframe;
    }
  }

  const newValue = !current;

  (currentModel as any).traverse((child: any) => {
    if (child instanceof THREE.Mesh) {
      if (Array.isArray(child.material)) {
        child.material.forEach((m) => (m.wireframe = newValue));
      } else {
        child.material.wireframe = newValue;
      }
    }
  });
}

function toggleAutoRotate() {
  if (!controls) return;
  controls.autoRotate = !controls.autoRotate;
}

async function loadModelFromFiles(files: FileList | File[]) {
  if (!scene) return;
  clearModel();
  cleanupObjectUrls();
  const fileArray = Array.from(files);
  buildFileMap(fileArray);

  const pnxFile = fileArray.find((f) =>
    f.name.toLowerCase().endsWith(".pnx")
  );
  if (pnxFile) {
    loadPNX(pnxFile);
    return;
  }

  const blendFile = fileArray.find((f) =>
    f.name.toLowerCase().endsWith(".blend")
  );
  const maxFile = fileArray.find((f) =>
    f.name.toLowerCase().endsWith(".max")
  );
  const pdoFile = fileArray.find((f) =>
    f.name.toLowerCase().endsWith(".pdo")
  );
  if (blendFile || maxFile || pdoFile) {
    const name = blendFile?.name || maxFile?.name || pdoFile?.name;
    alert(
      `Файл "${name}" (Blender/3dsMax/PDO) не поддерживается.\nЭкспортируйте в OBJ.`
    );
    return;
  }

  const objFile = fileArray.find((f) => f.name.toLowerCase().endsWith(".obj"));
  const mtlFile = fileArray.find((f) => f.name.toLowerCase().endsWith(".mtl"));
  const gltfFile = fileArray.find((f) =>
    f.name.toLowerCase().endsWith(".gltf") ||
    f.name.toLowerCase().endsWith(".glb")
  );
  const fbxFile = fileArray.find((f) => f.name.toLowerCase().endsWith(".fbx"));
  const stlFile = fileArray.find((f) => f.name.toLowerCase().endsWith(".stl"));
  const plyFile = fileArray.find((f) => f.name.toLowerCase().endsWith(".ply"));
  const daeFile = fileArray.find((f) => f.name.toLowerCase().endsWith(".dae"));
  const tdsFile = fileArray.find((f) => f.name.toLowerCase().endsWith(".3ds"));

  if (
    !objFile &&
    !gltfFile &&
    !fbxFile &&
    !stlFile &&
    !plyFile &&
    !daeFile &&
    !tdsFile
  ) {
    alert("Формат не поддерживается (OBJ, GLB, FBX, STL, PLY, DAE, 3DS)");
    return;
  }

  const manager = createManagerForFileMap();
  let object: THREE.Object3D | null = null;

  try {
    if (objFile) {
      const textDecoder = new TextDecoder();
      let materials: THREE.MaterialCreator | null = null;
      if (mtlFile) {
        const mtlText = textDecoder.decode(await mtlFile.arrayBuffer());
        const mtlLoader = new MTLLoader(manager);
        materials = mtlLoader.parse(mtlText);
        materials.preload();
      }
      const objLoader = new OBJLoader(manager);
      if (materials) objLoader.setMaterials(materials);
      const objText = textDecoder.decode(await objFile.arrayBuffer());
      object = objLoader.parse(objText);
      modelMeta.value.filename = objFile.name;
    } else if (gltfFile) {
      const gltfLoader = new GLTFLoader(manager);
      const blobUrl = URL.createObjectURL(gltfFile);
      object = await new Promise<THREE.Object3D>((resolve, reject) => {
        gltfLoader.load(
          blobUrl,
          (gltf) => resolve(gltf.scene),
          undefined,
          reject
        );
      });
      URL.revokeObjectURL(blobUrl);
      modelMeta.value.filename = gltfFile.name;
    } else if (fbxFile) {
      const fbxLoader = new FBXLoader(manager);
      const arrayBuffer = await fbxFile.arrayBuffer();
      object = fbxLoader.parse(arrayBuffer, fbxFile.name);
      modelMeta.value.filename = fbxFile.name;
    } else if (stlFile) {
      const stlLoader = new STLLoader();
      const arrayBuffer = await stlFile.arrayBuffer();
      const geometry = stlLoader.parse(arrayBuffer);
      const material = new THREE.MeshStandardMaterial({
        color: 0x606060,
        metalness: 0.5,
        roughness: 0.5,
      });
      const mesh = new THREE.Mesh(geometry, material);
      mesh.rotation.x = -Math.PI / 2;
      object = mesh;
      modelMeta.value.filename = stlFile.name;
    } else if (plyFile) {
      const plyLoader = new PLYLoader();
      const arrayBuffer = await plyFile.arrayBuffer();
      const geometry = plyLoader.parse(arrayBuffer);
      geometry.computeVertexNormals();
      const material = new THREE.MeshStandardMaterial({
        color: 0x606060,
        metalness: 0.5,
        roughness: 0.5,
      });
      const mesh = new THREE.Mesh(geometry, material);
      object = mesh;
      modelMeta.value.filename = plyFile.name;
    } else if (daeFile) {
      const daeLoader = new ColladaLoader(manager);
      const text = await daeFile.text();
      const collada = daeLoader.parse(text);
      object = collada.scene;
      modelMeta.value.filename = daeFile.name;
    } else if (tdsFile) {
      const tdsLoader = new TDSLoader(manager);
      const arrayBuffer = await tdsFile.arrayBuffer();
      object = tdsLoader.parse(arrayBuffer);
      (object as any).rotation.x = -Math.PI / 2;
      modelMeta.value.filename = tdsFile.name;
    }

    if (object) addModelToScene(object);
  } catch (e) {
    console.error(e);
    alert("Ошибка загрузки: " + e);
  }
}

function loadObjFromFiles(files: FileList | File[]) {
  loadModelFromFiles(files);
}

async function loadObjFromUrl(objUrl: string, mtlUrl: string | null = null) {
  if (!scene) return;
  clearModel();
  cleanupObjectUrls();
  try {
    let materials: THREE.MaterialCreator | null = null;
    if (mtlUrl) {
      const mtlResp = await fetch(mtlUrl);
      const mtlText = await mtlResp.text();
      const mtlLoader = new MTLLoader();
      materials = mtlLoader.parse(mtlText);
      materials.preload();
    }
    const objResp = await fetch(objUrl);
    const objText = await objResp.text();
    const objLoader = new OBJLoader();
    if (materials) objLoader.setMaterials(materials);
    const object = objLoader.parse(objText);
    addModelToScene(object);
  } catch (e) {
    console.error(e);
    alert("Ошибка URL");
  }
}

function resetCamera() {
  if (currentModel) fitCameraToModel(currentModel);
  else {
    camera.position.set(5, 5, 5);
    camera.lookAt(0, 0, 0);
    controls.target.set(0, 0, 0);
    controls.update();
    renderer.render(scene, camera);
  }
}

function resetFiles() {
  clearModel();
  unfoldState.clearCuts();
  controls.update();
  renderer.render(scene, camera);
}

function updateLights(ambient: number, dir: number) {
  if (ambientLight) ambientLight.intensity = ambient;
  if (dirLight) dirLight.intensity = dir;
}

function updateModelScale(scale: number) {
  if (!currentModel) return;
  currentModel.scale.set(scale, scale, scale);
  modelMeta.value.scale = scale;

  currentModel.updateMatrixWorld(true);

  let target: THREE.Mesh | null = null;
  if (currentModel.children.length > 0) {
    const first = currentModel.children[0];
    if (first instanceof THREE.Mesh) target = first;
  } else if (currentModel instanceof THREE.Mesh) {
    target = currentModel;
  }
  if (target) updateSeamVisualization(target);
}

function simplifyModel(factor: number) {
  if (!currentModel) return { total: 0, removed: 0 };
  let totalVerts = 0;
  let removedVerts = 0;

  (currentModel as any).traverse((child: any) => {
    if (child instanceof THREE.Mesh) {
      const oldGeo = child.geometry;
      totalVerts += oldGeo.attributes.position.count;
      const targetCount = Math.floor(oldGeo.attributes.position.count * factor);
      if (targetCount < oldGeo.attributes.position.count) {
        try {
          const modifier = new SimplifyModifier();
          child.geometry = modifier.modify(oldGeo, targetCount);
          removedVerts += oldGeo.attributes.position.count - targetCount;
        } catch (e) {
          console.warn("Simplify failed", e);
        }
      }
    }
  });

  return { total: totalVerts, removed: removedVerts };
}

function loadTestCube() {
  if (!scene) return;
  clearModel();
  const geometry = new THREE.BoxGeometry(2, 2, 2);
  const material = new THREE.MeshStandardMaterial({ color: 0x00ff00 });
  const cube = new THREE.Mesh(geometry, material);
  cube.position.set(0, 1, 0);
  scene.add(cube);
  currentModel = cube;
  controls.update();
  renderer.render(scene, camera);
}

defineExpose({
  loadModelFromFiles,
  loadObjFromFiles,
  loadObjFromUrl,
  resetCamera,
  resetFiles,
  updateLights,
  updateModelScale,
  simplifyModel,
  savePNX,
  loadPNX,
  loadTestCube,
  toggleWireframe,
  toggleAutoRotate,
});
</script>

<style scoped>
.viewer-container {
  position: relative;
  width: 100%;
  height: 100%;
  overflow: hidden;
  background-color: #1a1a1a;
}
.viewer-canvas {
  display: block;
  width: 100%;
  height: 100%;
  outline: none;
}
.overlay-top {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  padding: 1rem;
  pointer-events: none;
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
}
.overlay-bottom {
  position: absolute;
  bottom: 0;
  left: 0;
  width: 100%;
  padding: 1rem;
  pointer-events: none;
  display: flex;
  justify-content: center;
  align-items: flex-end;
  gap: 1rem;
}
.overlay-top > *,
.overlay-bottom > * {
  pointer-events: auto;
}
</style>





