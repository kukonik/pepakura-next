// Экспорт классов загрузчиков
export { OBJLoader as OriginalOBJLoader } from './loaders/OBJLoader';
export { MTLLoader as OriginalMTLLoader } from 'three/examples/jsm/loaders/MTLLoader.js';
// Остальные загрузчики добавим позже
// export { STLLoader as OriginalSTLLoader } from './loaders/STLLLoader.js';
// export { GLTFLoader as OriginalGLTFLoader } from './loaders/GLTFLoader.js';

// Типы для работы (будут расшираться)
export enum FormatType { OBJ, MTL, STL, GLTF, PLY };

// Вспомогательная функция для создания менеджера проектов (заготовка под Phase 2)
export const createProjectManager = async () => {
  // stub
}
