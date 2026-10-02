import { ref, computed, onUnmounted } from 'vue';

// Типы сообщений воркера
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

type WorkerMessage = 
  | UnfoldRequest
  | UnfoldSuccess
  | UnfoldError
  | { type: 'WORKER_READY' };

/**
 * Композиция для асинхронной развёртки меша через Web Worker.
 * Предоставляет реактивные переменные состояния и функцию запуска.
 */
export function useAsyncUnfold() {
  const worker = ref<Worker | null>(null);
  const isProcessing = ref(false);
  const progress = ref(0);
  const result = ref<any>(null);
  const error = ref<string | null>(null);
  const isReady = ref(false);

  // ID текущего запроса
  const currentRequestId = ref<string>('');
  // Ссылки на резолверы текущего промиса
  let currentResolve: ((value: any) => void) | null = null;
  let currentReject: ((reason?: any) => void) | null = null;
  // Интервал прогресса
  let progressInterval: NodeJS.Timeout | null = null;

  // Инициализация воркера
  const initWorker = () => {
    if (worker.value) return;

    try {
      // Создаём воркер из файла
      worker.value = new Worker(new URL('../workers/unfold.worker.ts', import.meta.url), {
        type: 'module'
      });

      worker.value.onmessage = (event: MessageEvent<WorkerMessage>) => {
        const message = event.data;
        switch (message.type) {
          case 'WORKER_READY':
            isReady.value = true;
            console.log('[useAsyncUnfold] Worker готов');
            break;
          case 'UNFOLD_SUCCESS':
            if (message.id === currentRequestId.value) {
              clearProgressInterval();
              result.value = message.result;
              error.value = null;
              isProcessing.value = false;
              progress.value = 100;
              if (currentResolve) {
                currentResolve(message.result);
                cleanupPromise();
              }
            }
            break;
          case 'UNFOLD_ERROR':
            if (message.id === currentRequestId.value) {
              clearProgressInterval();
              error.value = message.error;
              isProcessing.value = false;
              progress.value = 0;
              if (currentReject) {
                currentReject(new Error(message.error));
                cleanupPromise();
              }
            }
            break;
        }
      };

      worker.value.onerror = (err) => {
        console.error('[useAsyncUnfold] Ошибка воркера:', err);
        clearProgressInterval();
        error.value = err.message;
        isProcessing.value = false;
        if (currentReject) {
          currentReject(err);
          cleanupPromise();
        }
      };
    } catch (err) {
      console.error('[useAsyncUnfold] Не удалось создать воркер:', err);
      error.value = err instanceof Error ? err.message : String(err);
    }
  };

  // Очистка интервала прогресса
  const clearProgressInterval = () => {
    if (progressInterval) {
      clearInterval(progressInterval);
      progressInterval = null;
    }
  };

  // Очистка ссылок на промис
  const cleanupPromise = () => {
    currentResolve = null;
    currentReject = null;
  };

  // Запуск развёртки
  const runUnfold = async (meshData: any, config?: UnfoldRequest['config']): Promise<any> => {
    if (!worker.value) {
      initWorker();
      // Ждём готовности воркера
      await new Promise<void>((resolve) => {
        const check = () => {
          if (isReady.value) {
            resolve();
          } else {
            setTimeout(check, 10);
          }
        };
        check();
      });
    }

    // Сброс состояния
    isProcessing.value = true;
    progress.value = 0;
    result.value = null;
    error.value = null;

    // Генерация уникального ID запроса
    const requestId = `unfold_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;
    currentRequestId.value = requestId;

    // Отправка запроса воркеру
    const request: UnfoldRequest = {
      type: 'UNFOLD_REQUEST',
      id: requestId,
      meshData,
      config,
    };
    worker.value!.postMessage(request);

    // Имитация прогресса (в реальности можно получать от воркера)
    clearProgressInterval();
    progressInterval = setInterval(() => {
      if (progress.value < 90) {
        progress.value += 10;
      }
    }, 200);

    // Возвращаем промис, который разрешится при успехе/ошибке
    return new Promise((resolve, reject) => {
      currentResolve = resolve;
      currentReject = reject;
    });
  };

  // Отмена выполнения
  const cancel = () => {
    if (worker.value && isProcessing.value) {
      // Отправляем сообщение о завершении воркеру
      worker.value.postMessage({ type: 'TERMINATE' });
      worker.value.terminate();
      worker.value = null;
      isProcessing.value = false;
      isReady.value = false;
      clearProgressInterval();
      progress.value = 0;
      error.value = 'Операция отменена пользователем';
      if (currentReject) {
        currentReject(new Error('Операция отменена пользователем'));
        cleanupPromise();
      }
    }
  };

  // Очистка при размонтировании
  onUnmounted(() => {
    if (worker.value) {
      worker.value.terminate();
      worker.value = null;
    }
    clearProgressInterval();
  });

  return {
    // Состояние
    isProcessing: computed(() => isProcessing.value),
    progress: computed(() => progress.value),
    result: computed(() => result.value),
    error: computed(() => error.value),
    isReady: computed(() => isReady.value),

    // Методы
    runUnfold,
    cancel,
    initWorker,
  };
}