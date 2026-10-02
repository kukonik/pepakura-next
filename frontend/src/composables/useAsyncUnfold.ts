/**
 * Composable для асинхронной развёртки с использованием Web Worker.
 * 
 * @example
 * ```typescript
 * const { isProcessing, progress, result, error, runUnfold, cancel } = useAsyncUnfold();
 * 
 * await runUnfold(meshData, config);
 * ```
 */

import { ref, readonly, computed } from 'vue';

// Типы данных
export interface MeshData {
  name: string;
  vertices: Array<{ id: number; position: number[] }>;
  faces: Array<{ vertices: number[] }>;
}

export interface UnfoldConfig {
  algorithm: 'mds' | 'lscm';
  max_iterations: number;
  tolerance: number;
  preserve_detail: boolean;
}

export interface UnfoldResult {
  vertices_2d: number[];
  faces: any[];
  metadata: string;
}

export interface UnfoldProgress {
  progress: number;
  message: string;
}

// Типы сообщений воркера
type WorkerRequest = 
  | { type: 'UNFOLD_REQUEST'; payload: { mesh: MeshData; config: UnfoldConfig } }
  | { type: 'TERMINATE' };

type WorkerResponse = 
  | { type: 'UNFOLD_SUCCESS'; payload: UnfoldResult }
  | { type: 'UNFOLD_ERROR'; payload: string }
  | { type: 'UNFOLD_PROGRESS'; payload: UnfoldProgress };

export function useAsyncUnfold() {
  // Реактивные переменные состояния
  const isProcessing = ref(false);
  const progress = ref<number>(0);
  const progressMessage = ref<string>('');
  const result = ref<UnfoldResult | null>(null);
  const error = ref<string | null>(null);

  // Воркер
  let worker: Worker | null = null;
  let resolvePromise: ((value: UnfoldResult) => void) | null = null;
  let rejectPromise: ((reason: Error) => void) | null = null;

  // Вычисляемое: готова ли развёртка
  const isReady = computed(() => result.value !== null && !isProcessing.value);

  // Вычисляемое: есть ли ошибка
  const hasError = computed(() => error.value !== null);

  /**
   * Инициализирует Web Worker.
   */
  function initWorker(): Worker {
    if (worker) {
      return worker;
    }

    // Создаём воркер через Vite worker loader
    // ?worker - директива Vite для загрузки как Web Worker
    // ?url - возвращает URL воркера вместо инстанса
    const workerUrl = new URL('../workers/unfold.worker.ts', import.meta.url);
    worker = new Worker(workerUrl, { type: 'module' });

    // Обработчик сообщений от воркера
    worker.onmessage = (event: MessageEvent<WorkerResponse>) => {
      const { type, payload } = event.data;

      switch (type) {
        case 'UNFOLD_SUCCESS':
          progress.value = 100;
          progressMessage.value = 'Развёртка завершена';
          result.value = payload;
          error.value = null;
          isProcessing.value = false;
          
          if (resolvePromise) {
            resolvePromise(payload);
          }
          break;

        case 'UNFOLD_ERROR':
          error.value = payload;
          isProcessing.value = false;
          progress.value = 0;
          
          if (rejectPromise) {
            rejectPromise(new Error(payload));
          }
          break;

        case 'UNFOLD_PROGRESS':
          progress.value = payload.progress;
          progressMessage.value = payload.message;
          break;
      }
    };

    // Обработчик ошибок воркера
    worker.onerror = (errorEvent: ErrorEvent) => {
      const errorMessage = `Ошибка воркера: ${errorEvent.message}`;
      error.value = errorMessage;
      isProcessing.value = false;
      
      if (rejectPromise) {
        rejectPromise(new Error(errorMessage));
      }
    };

    return worker;
  }

  /**
   * Запускает развёртку в отдельном потоке.
   * 
   * @param mesh - Данные 3D меша
   * @param config - Конфигурация развёртки
   * @returns Promise с результатом развёртки
   */
  async function runUnfold(
    mesh: MeshData,
    config: UnfoldConfig = {
      algorithm: 'mds',
      max_iterations: 100,
      tolerance: 1e-6,
      preserve_detail: true,
    }
  ): Promise<UnfoldResult> {
    // Сбрасываем предыдущее состояние
    reset();

    // Инициализируем воркер
    worker = initWorker();

    isProcessing.value = true;
    progress.value = 0;
    progressMessage.value = 'Запуск развёртки...';

    // Отправляем запрос воркеру
    const request: WorkerRequest = {
      type: 'UNFOLD_REQUEST',
      payload: { mesh, config },
    };

    worker.postMessage(request);

    // Возвращаем Promise для ожидания результата
    return new Promise<UnfoldResult>((resolve, reject) => {
      resolvePromise = resolve;
      rejectPromise = reject;
    });
  }

  /**
   * Отменяет текущую развёртку и завершает воркер.
   */
  function cancel(): void {
    if (worker) {
      // Отправляем сигнал завершения
      worker.postMessage({ type: 'TERMINATE' } as WorkerRequest);
      
      // Завершаем воркер
      worker.terminate();
      worker = null;
    }

    // Отклоняем Promise если он ещё активен
    if (rejectPromise) {
      rejectPromise(new Error('Unfold cancelled by user'));
      rejectPromise = null;
    }

    isProcessing.value = false;
    progressMessage.value = 'Отменено';
  }

  /**
   * Сбрасывает состояние composable.
   */
  function reset(): void {
    isProcessing.value = false;
    progress.value = 0;
    progressMessage.value = '';
    result.value = null;
    error.value = null;
    resolvePromise = null;
    rejectPromise = null;
  }

  /**
   * Очищает ресурсы (вызывать при unmount компонента).
   */
  function dispose(): void {
    cancel();
  }

  return {
    // Состояние
    isProcessing: readonly(isProcessing),
    progress: readonly(progress),
    progressMessage: readonly(progressMessage),
    result: readonly(result),
    error: readonly(error),
    isReady,
    hasError,

    // Методы
    runUnfold,
    cancel,
    reset,
    dispose,
  };
}

export default useAsyncUnfold;
