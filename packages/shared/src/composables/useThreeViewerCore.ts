import * as THREE from 'three';
import { OBJLoader } from 'three/examples/jsm/loaders/OBJLoader.js';
import { MTLLoader } from 'three/examples/jsm/loaders/MTLLoader.js';
import { SimplifyModifier } from 'three/examples/jsm/modifiers/SimplifyModifier.js';

export interface ViewerRefs {
  scene: THREE.Scene;
  camera: THREE.PerspectiveCamera;
  renderer?: THREE.WebGLRenderer;
  controls?: THREE.OrbitControls;
  container?: HTMLElement;
}

export interface ViewerCallbacks {
  onLoadStart?: () => void;
  onLoadProgress?: (progress: ProgressEvent) => void;
  onLoadError?: (err: unknown) => void;
  onLoadFinished?: (object: THREE.Group) => void;
  onLog?: (msg: string) => void;
}

export function useThreeViewerCore(refs: ViewerRefs, callbacks: ViewerCallbacks = {}) {
  const { scene, camera, renderer, controls, container } = refs;
  const { onLog, onLoadStart, onLoadProgress, onLoadError, onLoadFinished } = callbacks;

  let objectUrls: string[] = [];
  let currentModelGroup: THREE.Group | null = null;

  // --- 1. File & Texture Management ---

  const cleanupObjectUrls = () => {
    objectUrls.forEach(url => URL.revokeObjectURL(url));
    objectUrls = [];
  };

  const createLoadingManager = (fileMap: Map<string, File>): THREE.LoadingManager => {
    const manager = new THREE.LoadingManager();

    manager.setURLModifier((url) => {
      // Normalize URL (remove paths, keep filename)
      const fileName = url.split(/(\\|\/)/g).pop() || url;
      
      if (fileMap.has(fileName)) {
        const file = fileMap.get(fileName);
        if (file) {
          const blobUrl = URL.createObjectURL(file);
          objectUrls.push(blobUrl);
          onLog?.(`[Core] Mapped ${fileName} to Blob`);
          return blobUrl;
        }
      }
      
      // Return original if not found in map (might be relative path or data URI)
      return url;
    });

    manager.onStart = (url, itemsLoaded, itemsTotal) => {
      onLog?.(`[Core] Started loading: ${url}`);
      onLoadStart?.();
    };

    manager.onProgress = (url, loaded, total) => {
      if (onLoadProgress) {
        onLoadProgress({ loaded, total } as any); // Simplified structure
      }
    };

    manager.onError = (url) => {
      onLog?.(`[Core] Error loading: ${url}`);
      onLoadError?.(new Error(`Failed to load ${url}`));
    };

    return manager;
  };

  // --- 2. Model Loading ---

  const clearModel = () => {
    if (currentModelGroup) {
      scene.remove(currentModelGroup);
      
      // Deep traversal for disposing
      currentModelGroup.traverse((child) => {
        if (child instanceof THREE.Mesh) {
          if (child.geometry) child.geometry.dispose();
          if (child.material) {
            if (Array.isArray(child.material)) {
              child.material.forEach(m => disposeMaterial(m));
            } else {
              disposeMaterial(child.material);
            }
          }
        }
      });
      
      currentModelGroup = null;
    }
    cleanupObjectUrls();
    onLog?.('[Core] Scene cleared');
  };

  const disposeMaterial = (material: THREE.Material) => {
    if ('map' in material && material.map) material.map.dispose();
    if ('normalMap' in material && material.normalMap) material.normalMap.dispose();
    material.dispose();
  };

  const loadObjFromFiles = async (files: FileList | File[]) => {
    clearModel();
    
    // 1. Populate FileMap
    const fileMap = new Map<string, File>();
    let objFile: File | null = null;
    let mtlFile: File | null = null;

    Array.from(files).forEach(file => {
      fileMap.set(file.name, file);
      const ext = file.name.split('.').pop()?.toLowerCase();
      if (ext === 'obj') objFile = file;
      if (ext === 'mtl') mtlFile = file;
    });

    if (!objFile) {
      onLog?.('[Core] No OBJ file found');
      onLoadError?.('No OBJ file provided');
      return;
    }

    // 2. Create Manager with FileMap
    const manager = createLoadingManager(fileMap);

    try {
      // 3. Load Materials if exists
      let materials: THREE.Material[] | null = null;
      if (mtlFile) {
        onLog?.('[Core] Parsing MTL...');
        const mtlLoader = new MTLLoader(manager);
        const mtlText = await mtlFile.text();
        const materialsCreator = mtlLoader.parse(mtlText);
        materialsCreator.preload();
        materials = materialsCreator.materials;
      }

      // 4. Load Object
      onLog?.('[Core] Parsing OBJ...');
      const objLoader = new OBJLoader(manager);
      if (materials) {
        objLoader.setMaterials(materials);
      }

      const objText = await objFile.text();
      const object = objLoader.parse(objText);

      // 5. Add to Scene
      scene.add(object);
      currentModelGroup = object;

      onLog?.('[Core] Model added to scene');
      onLoadFinished?.(object);

      // 6. Fit Camera
      if (object) {
        fitCameraToModel(object);
      }

    } catch (err) {
      onLog?.(`[Core] Exception: ${err}`);
      onLoadError?.(err);
      clearModel();
    }
  };

  const loadObjFromUrl = async (objUrl: string, mtlUrl?: string | null) => {
    clearModel();
    onLog?.('[Core] Loading from URL...');
    
    const manager = new THREE.LoadingManager();
    // Note: URL loading doesn't use fileMap logic, simple resolution
    
    try {
      let materials: THREE.Material[] | null = null;
      
      if (mtlUrl) {
        const mtlLoader = new MTLLoader(manager);
        const materialsCreator = await mtlLoader.loadAsync(mtlUrl);
        materialsCreator.preload();
        materials = materialsCreator.materials;
      }

      const objLoader = new OBJLoader(manager);
      if (materials) objLoader.setMaterials(materials);

      const object = await objLoader.loadAsync(objUrl);
      scene.add(object);
      currentModelGroup = object;
      
      onLog?.('[Core] URL Model loaded');
      onLoadFinished?.(object);
      
      fitCameraToModel(object);
    } catch (err) {
      onLog?.(`[Core] Error loading URL: ${err}`);
      onLoadError?.(err);
    }
  };

  // --- 3. Camera Logic ---

  const fitCameraToModel = (object: THREE.Object3D) => {
    const box = new THREE.Box3().setFromObject(object);
    const size = new THREE.Vector3();
    box.getSize(size);

    // Sanity check
    if (!Number.isFinite(size.x) || !Number.isFinite(size.y) || !Number.isFinite(size.z)) {
      onLog?.('[Core] Invalid model size, skipping camera fit');
      return;
    }

    const maxDim = Math.max(size.x, size.y, size.z);
    if (maxDim < 0.001 || maxDim > 10000) {
      onLog?.('[Core] Model size out of bounds, skipping camera fit');
      return;
    }

    const center = new THREE.Vector3();
    box.getCenter(center);

    const fov = camera.fov * (Math.PI / 180);
    let cameraZ = Math.abs(maxDim / 2 * Math.tan(fov * 2)); // Basic distance

    cameraZ *= 2.5; // Zoom out factor
    
    // Clamp camera Z
    cameraZ = Math.max(0.1, Math.min(cameraZ, 10000));

    const direction = new THREE.Vector3().subVectors(camera.position, center).normalize();
    const position = direction.multiplyScalar(cameraZ).add(center);

    camera.position.copy(position);
    camera.lookAt(center);
    if (controls) controls.target.copy(center);
    if (controls) controls.update();

    onLog?.(`[Core] Cam fitted. Z: ${cameraZ.toFixed(2)}`);
  };

  const resetCamera = () => {
    if (currentModelGroup) {
      fitCameraToModel(currentModelGroup);
    } else {
      camera.position.set(0, 0, 5);
      camera.lookAt(0, 0, 0);
      if (controls) controls.target.set(0, 0, 0);
      if (controls) controls.update();
      onLog?.('[Core] Camera reset to default');
    }
  };

  // --- 4. Utilities ---

  const updateLights = (ambientIntensity: number, dirIntensity: number) => {
    scene.traverse((obj) => {
      if (obj instanceof THREE.AmbientLight) {
        obj.intensity = ambientIntensity;
      }
      if (obj instanceof THREE.DirectionalLight) {
        obj.intensity = dirIntensity;
      }
    });
    onLog?.(`[Core] Lights updated`);
  };

  const updateModelScale = (scale: number) => {
    if (currentModelGroup) {
      currentModelGroup.scale.set(scale, scale, scale);
      // Recalculate fit or just update? Usually fits again or leaves to user.
      // Let's leave position manual after scaling to avoid jumping, 
      // or re-fit if desired. Here we just scale.
    }
  };

  const simplifyModel = (factor: number) => {
    if (!currentModelGroup) return;
    onLog?.(`[Core] Simplify factor ${factor} requested (client-side only)`);
    // Actual simplification logic requires traversing meshes and applying modifier.
    // This is a placeholder for the shared logic hook.
    // Implementation details depend on whether we want destructive editing or a clone.
    // For now, we just log as it requires complex geometry cloning logic to be safe.
  };

  const loadTestCube = () => {
    clearModel();
    const geometry = new THREE.BoxGeometry(1, 1, 1);
    const material = new THREE.MeshStandardMaterial({ color: 0x00ff00 });
    const cube = new THREE.Mesh(geometry, material);
    scene.add(cube);
    currentModelGroup = cube;
    onLog?.('[Core] Test cube loaded');
    fitCameraToModel(cube);
  };

  return {
    loadObjFromFiles,
    loadObjFromUrl,
    clearModel,
    resetCamera,
    updateLights,
    updateModelScale,
    simplifyModel,
    loadTestCube
  };
}
