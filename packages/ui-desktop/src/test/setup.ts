import { vi } from 'vitest';
import { config } from '@vue/test-utils';

// Мокаем глобальные объекты и API
Object.defineProperty(window, 'matchMedia', {
  writable: true,
  value: vi.fn().mockImplementation(query => ({
    matches: false,
    media: query,
    onchange: null,
    addListener: vi.fn(),
    removeListener: vi.fn(),
    addEventListener: vi.fn(),
    removeEventListener: vi.fn(),
    dispatchEvent: vi.fn(),
  })),
});

Object.defineProperty(window, 'localStorage', {
  value: {
    getItem: vi.fn(),
    setItem: vi.fn(),
    removeItem: vi.fn(),
    clear: vi.fn(),
  },
  writable: true,
});

Object.defineProperty(window, 'sessionStorage', {
  value: {
    getItem: vi.fn(),
    setItem: vi.fn(),
    removeItem: vi.fn(),
    clear: vi.fn(),
  },
  writable: true,
});

// Мокаем Tauri API
vi.mock('@tauri-apps/api', () => ({
  invoke: vi.fn(),
  event: {
    listen: vi.fn(),
    emit: vi.fn(),
  },
  window: {
    getCurrent: vi.fn().mockReturnValue({
      setTitle: vi.fn(),
    }),
  },
  dialog: {
    open: vi.fn(),
    save: vi.fn(),
  },
  fs: {
    readTextFile: vi.fn(),
    writeTextFile: vi.fn(),
  },
}));

// Мокаем Tauri runtime
vi.mock('@tauri-apps/api/tauri', () => ({
  invoke: vi.fn(),
}));

// Настраиваем Vue Test Utils
config.global.mocks = {
  $t: (key: string) => key,
  $router: {
    push: vi.fn(),
    replace: vi.fn(),
    go: vi.fn(),
  },
  $route: {
    params: {},
    query: {},
  },
};

config.global.provide = {
  // Добавляем провайдеры для зависимостей
};

config.global.components = {
  // Регистрируем глобальные компоненты
};

// Глобальные настройки для тестов
beforeEach(() => {
  // Очищаем все моки перед каждым тестом
  vi.clearAllMocks();
  
  // Сбрасываем состояние хранилищ
  // Это нужно делать в каждом конкретном тесте, 
  // но здесь можно установить общие значения по умолчанию
});

afterEach(() => {
  // Очистка после каждого теста
  vi.restoreAllMocks();
});

// Глобальные хелперы для тестов
global.console = {
  ...console,
  log: vi.fn(),
  error: vi.fn(),
  warn: vi.fn(),
  info: vi.fn(),
  debug: vi.fn(),
};

// Мокаем performance API
Object.defineProperty(global, 'performance', {
  writable: true,
  value: {
    now: vi.fn().mockReturnValue(1000),
    mark: vi.fn(),
    measure: vi.fn(),
    getEntriesByName: vi.fn().mockReturnValue([]),
    getEntriesByType: vi.fn().mockReturnValue([]),
    clearMarks: vi.fn(),
    clearMeasures: vi.fn(),
  },
});

// Мокаем URL.createObjectURL
Object.defineProperty(URL, 'createObjectURL', {
  writable: true,
  value: vi.fn().mockReturnValue('blob:test'),
});

Object.defineProperty(URL, 'revokeObjectURL', {
  writable: true,
  value: vi.fn(),
});

// Мокаем FileReader
class MockFileReader {
  onload: (() => void) | null = null;
  onerror: (() => void) | null = null;
  result: string | ArrayBuffer | null = null;
  
  readAsText(blob: Blob) {
    this.result = 'mock file content';
    if (this.onload) {
      this.onload();
    }
  }
  
  readAsDataURL(blob: Blob) {
    this.result = 'data:text/plain;base64,bW9jayBmaWxlIGNvbnRlbnQ=';
    if (this.onload) {
      this.onload();
    }
  }
}

Object.defineProperty(window, 'FileReader', {
  writable: true,
  value: MockFileReader,
});

// Мокаем Image
Object.defineProperty(window, 'Image', {
  writable: true,
  value: class MockImage {
    onload: (() => void) | null = null;
    onerror: (() => void) | null = null;
    src = '';
    
    constructor() {
      setTimeout(() => {
        if (this.onload) {
          this.onload();
        }
      }, 0);
    }
  },
});

// Мокаем fetch API
Object.defineProperty(window, 'fetch', {
  writable: true,
  value: vi.fn().mockResolvedValue({
    ok: true,
    status: 200,
    json: vi.fn().mockResolvedValue({}),
    text: vi.fn().mockResolvedValue(''),
    blob: vi.fn().mockResolvedValue(new Blob()),
  }),
});

// Мокаем ResizeObserver
Object.defineProperty(window, 'ResizeObserver', {
  writable: true,
  value: vi.fn().mockImplementation(() => ({
    observe: vi.fn(),
    unobserve: vi.fn(),
    disconnect: vi.fn(),
  })),
});

// Мокаем IntersectionObserver
Object.defineProperty(window, 'IntersectionObserver', {
  writable: true,
  value: vi.fn().mockImplementation(() => ({
    observe: vi.fn(),
    unobserve: vi.fn(),
    disconnect: vi.fn(),
  })),
});

// Мокаем requestAnimationFrame
Object.defineProperty(window, 'requestAnimationFrame', {
  writable: true,
  value: vi.fn().mockImplementation(callback => {
    return setTimeout(callback, 0);
  }),
});

Object.defineProperty(window, 'cancelAnimationFrame', {
  writable: true,
  value: vi.fn().mockImplementation(id => {
    clearTimeout(id);
  }),
});

// Мокаем crypto
Object.defineProperty(window, 'crypto', {
  writable: true,
  value: {
    getRandomValues: vi.fn().mockImplementation(array => {
      for (let i = 0; i < array.length; i++) {
        array[i] = Math.floor(Math.random() * 256);
      }
      return array;
    }),
    subtle: {
      digest: vi.fn().mockResolvedValue(new ArrayBuffer(32)),
    },
  },
});

export {};