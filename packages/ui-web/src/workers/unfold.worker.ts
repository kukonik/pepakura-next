/// <reference lib="webworker" />

declare const self: DedicatedWorkerGlobalScope;

// Типы сообщений
type UnfoldRequest = {
  type: 'UNFOLD_REQUEST';
  id: string;
  meshData: any;
  config?: {
    algorithm?: 'mds' | 'lscm';
    maxIterations?: number;
    tolerance?: number;
    preserveDetail?: boolean;
  };
};

type UnfoldSuccess = {
  type: 'UNFOLD_SUCCESS';
  id: string;
  result: {
    vertices2d: number[];
    faces: any[];
    metadata: string;
  };
};

type UnfoldError = {
  type: 'UNFOLD_ERROR';
  id: string;
  error: string;
};

type WorkerMessage = UnfoldRequest | { type: 'TERMINATE' };

// Загрузка WASM модуля
let wasmModule: typeof import('pepakura_core_wasm') | null = null;
let isInitializing = false;
let initPromise: Promise<void> | null = null;

async function initWasm(): Promise<void> {
  if (wasmModule) return;
  if (isInitializing && initPromise) {
    await initPromise;
    return;
  }
  isInitializing = true;
  initPromise = (async () => {
    try {
      // Динамический импорт WASM модуля
      wasmModule = await import('pepakura_core_wasm');
      await wasmModule.init();
      console.log('[Worker] WASM модуль загружен');
    } catch (error) {
      console.error('[Worker] Ошибка загрузки WASM:', error);
      throw error;
    } finally {
      isInitializing = false;
    }
  })();
  await initPromise;
}

// Обработка развёртки
async function handleUnfold(request: UnfoldRequest): Promise<void> {
  const { id, meshData, config } = request;
  try {
    await initWasm();
    if (!wasmModule) {
      throw new Error('WASM модуль не загружен');
    }

    // Конвертируем meshData в формат, ожидаемый WASM
    const meshWasm = new wasmModule.MeshWasm(
      meshData.name || 'Mesh',
      meshData.vertices.map((v: any, idx: number) =>
        new wasmModule.VertexWasm(idx, v.position || [v.x, v.y, v.z])
      ),
      meshData.faces.map((f: any) =>
        new wasmModule.FaceWasm(f.vertices || f)
      )
    );

    const configWasm = new wasmModule.UnfoldConfigWasm(
      config?.algorithm,
      config?.maxIterations,
      config?.tolerance,
      config?.preserveDetail
    );

    const result = wasmModule.unfold_mesh(meshWasm, configWasm);

    const successMessage: UnfoldSuccess = {
      type: 'UNFOLD_SUCCESS',
      id,
      result: {
        vertices2d: result.vertices_2d(),
        faces: result.faces().map((f: any) => {
          const obj = JSON.parse(JSON.stringify(f));
          return obj;
        }),
        metadata: result.metadata(),
      },
    };
    self.postMessage(successMessage);
  } catch (error) {
    const errorMessage: UnfoldError = {
      type: 'UNFOLD_ERROR',
      id,
      error: error instanceof Error ? error.message : String(error),
    };
    self.postMessage(errorMessage);
  }
}

// Обработчик сообщений
self.addEventListener('message', async (event: MessageEvent<WorkerMessage>) => {
  const message = event.data;
  if (message.type === 'UNFOLD_REQUEST') {
    await handleUnfold(message);
  } else if (message.type === 'TERMINATE') {
    self.close();
  }
});

// Уведомляем о готовности
self.postMessage({ type: 'WORKER_READY' });

console.log('[Worker] Unfold worker запущен');