// D:\Dev\pepakura-next\tools\scene_snapshot.ts

import * as fs from "fs";
import {
  loadObjModel,
  setCamera,
  setModelMatrix,
  setLights,
  applySeams,
  applyMeshData,
  getCurrentModelPath,
  getCurrentCamera,
  getCurrentModelMatrix,
  getCurrentLights,
  getCurrentSeams,
  getCurrentMeshPositions,
  getCurrentMeshUVs,
  getCurrentMaterials
} from "D:\\Dev\\pepakura-next\\src\\modules\\renderer-3d\\sceneRuntime";

interface Camera {
  position: [number, number, number];
  target: [number, number, number];
}

interface Lights {
  ambient: number;
  dir: number;
}

interface Material {
  color: string;
}

interface Snapshot {
  version: string;
  timestamp: number;
  modelPath: string;
  camera: Camera;
  modelScale: number;
  lights: Lights;
  matrix: number[];
  seams: number[];
  meshesPositions: number[];
  meshesUVs: number[];
  materials: Material[];
}

function serializeSnapshot(snapshot: Snapshot): string {
  let result = "";
  result += "version " + snapshot.version + "\n";
  result += "timestamp " + snapshot.timestamp + "\n";
  result += "modelPath " + snapshot.modelPath + "\n";
  result += "camera position " + snapshot.camera.position.join(" ") + "\n";
  result += "camera target " + snapshot.camera.target.join(" ") + "\n";
  result += "model scale " + snapshot.modelScale + "\n";
  result += "lights ambient " + snapshot.lights.ambient + "\n";
  result += "lights dir " + snapshot.lights.dir + "\n";
  result += "matrix " + snapshot.matrix.join(" ") + "\n";
  result += "seams " + snapshot.seams.join(" ") + "\n";
  result += "meshes position " + snapshot.meshesPositions.join(" ") + "\n";
  result += "meshes uv " + snapshot.meshesUVs.join(" ") + "\n";
  result +=
    "materials color " +
    (snapshot.materials[0] && snapshot.materials[0].color
      ? snapshot.materials[0].color
      : "ffffff") +
    "\n";
  return result;
}

function parseSnapshot(text: string): Snapshot {
  const lines = text.trim().split("\n");
  const snapshot: Snapshot = {
    version: "",
    timestamp: 0,
    modelPath: "",
    camera: {
      position: [0, 0, 0],
      target: [0, 0, 0],
    },
    modelScale: 1.0,
    lights: {
      ambient: 0.5,
      dir: 0.5,
    },
    matrix: new Array(16).fill(0),
    seams: [],
    meshesPositions: [],
    meshesUVs: [],
    materials: [{ color: "ffffff" }],
  };

  for (const rawLine of lines) {
    const line = rawLine.trim();
    if (line.length === 0) continue;

    if (line.startsWith("version ")) {
      snapshot.version = line.substring("version ".length).trim();
      continue;
    }

    if (line.startsWith("timestamp ")) {
      const v = line.substring("timestamp ".length).trim();
      snapshot.timestamp = v ? parseFloat(v) : 0;
      continue;
    }

    if (line.startsWith("modelPath ")) {
      snapshot.modelPath = line.substring("modelPath ".length).trim();
      continue;
    }

    if (line.startsWith("camera position ")) {
      const rest = line.substring("camera position ".length).trim();
      const parts = rest.split(/\s+/);
      snapshot.camera.position = [
        parseFloat(parts[0] || "0"),
        parseFloat(parts[1] || "0"),
        parseFloat(parts[2] || "0"),
      ];
      continue;
    }

    if (line.startsWith("camera target ")) {
      const rest = line.substring("camera target ".length).trim();
      const parts = rest.split(/\s+/);
      snapshot.camera.target = [
        parseFloat(parts[0] || "0"),
        parseFloat(parts[1] || "0"),
        parseFloat(parts[2] || "0"),
      ];
      continue;
    }

    if (line.startsWith("model scale ")) {
      const v = line.substring("model scale ".length).trim();
      snapshot.modelScale = v ? parseFloat(v) : 1.0;
      continue;
    }

    if (line.startsWith("lights ambient ")) {
      const v = line.substring("lights ambient ".length).trim();
      snapshot.lights.ambient = v ? parseFloat(v) : 0.5;
      continue;
    }

    if (line.startsWith("lights dir ")) {
      const v = line.substring("lights dir ".length).trim();
      snapshot.lights.dir = v ? parseFloat(v) : 0.5;
      continue;
    }

    if (line.startsWith("matrix ")) {
      const rest = line.substring("matrix ".length).trim();
      snapshot.matrix = rest.length
        ? rest.split(/\s+/).map((v) => parseFloat(v))
        : [];
      continue;
    }

    if (line.startsWith("seams ")) {
      const rest = line.substring("seams ".length).trim();
      snapshot.seams = rest.length
        ? rest.split(/\s+/).map((v) => parseInt(v, 10))
        : [];
      continue;
    }

    if (line.startsWith("meshes position ")) {
      const rest = line.substring("meshes position ".length).trim();
      snapshot.meshesPositions = rest.length
        ? rest.split(/\s+/).map((v) => parseFloat(v))
        : [];
      continue;
    }

    if (line.startsWith("meshes uv ")) {
      const rest = line.substring("meshes uv ".length).trim();
      snapshot.meshesUVs = rest.length
        ? rest.split(/\s+/).map((v) => parseFloat(v))
        : [];
      continue;
    }

    if (line.startsWith("materials color ")) {
      const v = line.substring("materials color ".length).trim();
      snapshot.materials[0].color = v || "ffffff";
      continue;
    }
  }

  return snapshot;
}

function saveSceneSnapshot(modelPathArg: string): void {
  const runtimeModelPath = modelPathArg || getCurrentModelPath();
  const currentCamera = getCurrentCamera();
  const currentMatrix = getCurrentModelMatrix();
  const currentLights = getCurrentLights();
  const currentSeams = getCurrentSeams();
  const currentPositions = getCurrentMeshPositions();
  const currentUVs = getCurrentMeshUVs();
  const currentMaterials = getCurrentMaterials();

  const snapshot: Snapshot = {
    version: "1.0",
    timestamp: Date.now(),
    modelPath: runtimeModelPath,
    camera: {
      position: currentCamera.position,
      target: currentCamera.target,
    },
    modelScale: 1.0,
    lights: {
      ambient: currentLights.ambient,
      dir: currentLights.dir,
    },
    matrix: currentMatrix,
    seams: currentSeams,
    meshesPositions: currentPositions,
    meshesUVs: currentUVs,
    materials:
      currentMaterials.length > 0 ? currentMaterials : [{ color: "ffffff" }],
  };

  const snapshotDir = "D:\\Dev\\pepakura-next\\snapshots";
  if (!fs.existsSync(snapshotDir)) {
    fs.mkdirSync(snapshotDir, { recursive: true });
  }

  const snapshotText = serializeSnapshot(snapshot);
  const snapshotPath = "D:\\Dev\\pepakura-next\\snapshots\\scene.snapshot";
  fs.writeFileSync(snapshotPath, snapshotText, { encoding: "utf8" });
  console.log("Снапшот сохранён: " + snapshotPath);
}

function loadSceneSnapshot(): void {
  const snapshotPath = "D:\\Dev\\pepakura-next\\snapshots\\scene.snapshot";

  if (!fs.existsSync(snapshotPath)) {
    console.error("Снапшот не найден: " + snapshotPath);
    return;
  }

  const snapshotText = fs.readFileSync(snapshotPath, "utf8");
  const snapshot = parseSnapshot(snapshotText);

  loadObjModel(snapshot.modelPath);
  setCamera(snapshot.camera.position, snapshot.camera.target);
  setModelMatrix(snapshot.matrix);
  setLights(snapshot.lights.ambient, snapshot.lights.dir);
  applySeams(snapshot.seams);
  applyMeshData(snapshot.meshesPositions, snapshot.meshesUVs);

  console.log("Снапшот загружен из: " + snapshotPath);
  console.log("Модель: " + snapshot.modelPath);
}

function main(): void {
  const args = process.argv.slice(2);

  if (args.length === 0) {
    console.log("Использование:");
    console.log(
      "  node D:\\Dev\\pepakura-next\\tools\\scene_snapshot.js save D:\\Dev\\pepakura-next\\models\\head.obj"
    );
    console.log(
      "  node D:\\Dev\\pepakura-next\\tools\\scene_snapshot.js load"
    );
    return;
  }

  const command = args[0];

  if (command === "save") {
    if (args.length !== 2) {
      console.error("Ошибка: укажите путь к модели");
      return;
    }
    const modelPathArg = args[1];
    if (!modelPathArg || !modelPathArg.startsWith("D:\\")) {
      console.error(
        "Ошибка: ожидается абсолютный путь к модели, начинающийся с D:\\"
      );
      return;
    }
    saveSceneSnapshot(modelPathArg);
  } else if (command === "load") {
    loadSceneSnapshot();
  } else {
    console.error("Неизвестная команда: " + command);
  }
}

main();
