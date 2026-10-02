/**
 * Web Worker для выполнения тяжелых геометрических расчетов развёртки.
 * 
 * Сообщения:
 * - UNFOLD_REQUEST: { type: 'UNFOLD_REQUEST', payload: { mesh: MeshData, config: UnfoldConfig } }
 * - TERMINATE: { type: 'TERMINATE' }
 * 
 * Ответы:
 * - UNFOLD_SUCCESS: { type: 'UNFOLD_SUCCESS', payload: UnfoldedMeshWasm }
 * - UNFOLD_ERROR: { type: 'UNFOLD_ERROR', payload: string }
 * - UNFOLD_PROGRESS: { type: 'UNFOLD_PROGRESS', payload: { progress: number, message: string } }
 */

// Импортируем WASM модуль
// Путь будет разрешён через Vite alias @pepakura_wasm
import init from '@pepakura_wasm/pepakura_wasm.js';
import * as wasm from '@pepakura_wasm/pepakura_wasm.js';

// Типы для сообщений
interface UnfoldRequest {
  type: 'UNFOLD_REQUEST';
  payload: {
    mesh: {
      name: string;
      vertices: Array<{ id: number; position: number[] }>;
      faces: Array<{ vertices: number[] }>;
    };
    config: {
      algorithm: string;
      max_iterations: number;
      tolerance: number;
      preserve_detail: boolean;
    };
  };
}

interface TerminateRequest {
  type: 'TERMINATE';
}

type WorkerMessage = UnfoldRequest | TerminateRequest;

// Типы для ответов
interface UnfoldSuccess {
  type: 'UNFOLD_SUCCESS';
  payload: {
    vertices_2d: number[];
    faces: any[];
    metadata: string;
  };
}

interface UnfoldError {
  type: 'UNFOLD_ERROR';
  payload: string;
}

interface UnfoldProgress {
  type: 'UNFOLD_PROGRESS';
  payload: {
    progress: number;
    message: string;
  };
}

type WorkerResponse = UnfoldSuccess | UnfoldError | UnfoldProgress;

// Флаг инициализации WASM
let wasmInitialized = false;

/**
 * Инициализирует WASM модуль.
 */
async function initializeWasm(): Promise<void> {
  if (wasmInitialized) return;
  
  self.postMessage({
    type: 'UNFOLD_PROGRESS',
    payload: { progress: 0, message: 'Инициализация WASM модуля...' }
  } as WorkerResponse);

  try {
    await init();
    wasmInitialized = true;
    
    self.postMessage({
      type: 'UNFOLD_PROGRESS',
      payload: { progress: 10, message: `WASM готов` }
    } as WorkerResponse);
  } catch (error) {
    throw new Error(`Failed to initialize WASM: ${error instanceof Error ? error.message : String(error)}`);
  }
}

/**
 * Обрабатывает запрос на развёртку.
 */
async function handleUnfoldRequest(request: UnfoldRequest): Promise<void> {
  const { mesh, config } = request.payload;
  
  try {
    // Отправляем прогресс
    self.postMessage({
      type: 'UNFOLD_PROGRESS',
      payload: { progress: 20, message: 'Подготовка данных...' }
    } as WorkerResponse);

    // Конвертируем данные в WASM-совместимый формат
    const vertices = mesh.vertices.map(
      v => new wasm.VertexWasm(v.id, v.position)
    );
    
    const faces = mesh.faces.map(
      f => new wasm.FaceWasm(f.vertices)
    );
    
    const meshWasm = new wasm.MeshWasm(
      mesh.name,
      vertices,
      faces
    );

    self.postMessage({
      type: 'UNFOLD_PROGRESS',
      payload: { progress: 40, message: 'Выполнение развёртки...' }
    } as WorkerResponse);

    // Создаём конфигурацию
    const configWasm = new wasm.UnfoldConfigWasm(
      config.algorithm,
      config.max_iterations,
      config.tolerance,
      config.preserve_detail
    );

    // Выполняем развёртку (тяжелая операция)
    const result = wasm.unfold_mesh(meshWasm, configWasm);

    self.postMessage({
      type: 'UNFOLD_PROGRESS',
      payload: { progress: 90, message: 'Обработка результата...' }
    } as WorkerResponse);

    // Отправляем результат
    self.postMessage({
      type: 'UNFOLD_SUCCESS',
      payload: {
        vertices_2d: result.vertices_2d(),
        faces: result.faces(),
        metadata: result.metadata()
      }
    } as WorkerResponse);

    self.postMessage({
      type: 'UNFOLD_PROGRESS',
      payload: { progress: 100, message: 'Готово!' }
    } as WorkerResponse);

  } catch (error) {
    const errorMessage = error instanceof Error ? error.message : String(error);
    
    self.postMessage({
      type: 'UNFOLD_ERROR',
      payload: `Ошибка развёртки: ${errorMessage}`
    } as WorkerResponse);
  }
}

/**
 * Обработчик входящих сообщений.
 */
self.onmessage = async (event: MessageEvent<WorkerMessage>) => {
  const { type } = event.data;

  switch (type) {
    case 'UNFOLD_REQUEST':
      try {
        await initializeWasm();
        await handleUnfoldRequest(event.data);
      } catch (error) {
        self.postMessage({
          type: 'UNFOLD_ERROR',
          payload: `Критическая ошибка: ${error instanceof Error ? error.message : String(error)}`
        } as WorkerResponse);
      }
      break;

    case 'TERMINATE':
      // Воркер будет завершён извне через terminate()
      self.postMessage({
        type: 'UNFOLD_PROGRESS',
        payload: { progress: 0, message: 'Завершение работы...' }
      } as WorkerResponse);
      break;

    default:
      self.postMessage({
        type: 'UNFOLD_ERROR',
        payload: `Неизвестный тип сообщения: ${type}`
      } as WorkerResponse);
  }
};

// Сообщаем о готовности воркера
self.postMessage({
  type: 'UNFOLD_PROGRESS',
  payload: { progress: 0, message: 'Воркер готов к работе' }
} as WorkerResponse);
