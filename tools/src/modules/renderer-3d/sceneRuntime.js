"use strict";
// D:\Dev\pepakura-next\src\modules\renderer-3d\sceneRuntime.ts
Object.defineProperty(exports, "__esModule", { value: true });
exports.loadObjModel = loadObjModel;
exports.setCamera = setCamera;
exports.setModelMatrix = setModelMatrix;
exports.setLights = setLights;
exports.applySeams = applySeams;
exports.applyMeshData = applyMeshData;
exports.getCurrentModelPath = getCurrentModelPath;
exports.getCurrentCamera = getCurrentCamera;
exports.getCurrentModelMatrix = getCurrentModelMatrix;
exports.getCurrentLights = getCurrentLights;
exports.getCurrentSeams = getCurrentSeams;
exports.getCurrentMeshPositions = getCurrentMeshPositions;
exports.getCurrentMeshUVs = getCurrentMeshUVs;
exports.getCurrentMaterials = getCurrentMaterials;
let currentModelPath = "D:\\Dev\\pepakura-next\\models\\model.obj";
let currentCamera = {
    position: [1, 0, 0],
    target: [0, 0, 0]
};
let currentMatrix = [
    1, 0, 0, 0,
    0, 1, 0, 0,
    0, 0, 1, 0,
    0, 0, 0, 1
];
let currentLights = {
    ambient: 0.5,
    dir: 0.8
};
let currentSeams = [10, 20, 30];
let currentMeshPositions = [0, 0, 0, 1, 0, 0, 0, 1, 0, 0];
let currentMeshUVs = [0, 0, 1, 0, 0, 1, 1];
let currentMaterials = [{ color: "ffffff" }];
function loadObjModel(absolutePath) {
    currentModelPath = absolutePath;
    console.log("loadObjModel:", absolutePath);
}
function setCamera(position, target) {
    currentCamera = { position, target };
    console.log("setCamera:", "position=" + position.join(","), "target=" + target.join(","));
}
function setModelMatrix(matrix) {
    currentMatrix = matrix.slice();
    console.log("setModelMatrix: matrix length=" + matrix.length);
}
function setLights(ambient, dir) {
    currentLights = { ambient, dir };
    console.log("setLights: ambient=" + ambient + " dir=" + dir);
}
function applySeams(seams) {
    currentSeams = seams.slice();
    console.log("applySeams: seams count=" + seams.length);
}
function applyMeshData(positions, uvs) {
    currentMeshPositions = positions.slice();
    currentMeshUVs = uvs.slice();
    console.log("applyMeshData: positions=" + positions.length + " uvs=" + uvs.length);
}
function getCurrentModelPath() {
    return currentModelPath;
}
function getCurrentCamera() {
    return {
        position: currentCamera.position.slice(),
        target: currentCamera.target.slice()
    };
}
function getCurrentModelMatrix() {
    return currentMatrix.slice();
}
function getCurrentLights() {
    return { ...currentLights };
}
function getCurrentSeams() {
    return currentSeams.slice();
}
function getCurrentMeshPositions() {
    return currentMeshPositions.slice();
}
function getCurrentMeshUVs() {
    return currentMeshUVs.slice();
}
function getCurrentMaterials() {
    return currentMaterials.map(m => ({ ...m }));
}
