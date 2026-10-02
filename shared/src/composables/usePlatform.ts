import { ref, computed } from 'vue';
import type { PlatformBridge, CoreResult } from '../bridge';
import { createTauriBridge } from '../bridge/TauriBridge';
import { createWasmBridge } from '../bridge/WasmBridge';
import { setBridge } from '../bridge';

let bridge = ref<PlatformBridge | null>(null);
let initPromise: Promise<void> | null = null;
let isDesktopEnvironment = false;

export function usePlatform() {
  const isDesktop = computed(() => isDesktopEnvironment);
  const isWeb = computed(() => !isDesktopEnvironment);
  const isInitialized = computed(() => bridge.value !== null);

  const init = async (): Promise<void> => {
    if (bridge.value) {
      return;
    }
    if (initPromise) {
      await initPromise;
      return;
    }

    initPromise = (async () => {
      try {
        delete (window as Record<string, unknown>).__TAURI__;
        isDesktopEnvironment = typeof window !== 'undefined' && '__TAURI_INTERNALS__' in window;
        
        if (isDesktopEnvironment) {
          bridge.value = createTauriBridge();
          setBridge(bridge.value);
        } else {
          bridge.value = createWasmBridge(null);
          setBridge(bridge.value);
        }
      } catch (error) {
        console.error('[Platform] CRITICAL: Bridge init failed:', error);
        bridge.value = null;
        throw error;
      } finally {
        initPromise = null;
      }
    })();

    await initPromise;
  };

  const getBridge = (): PlatformBridge => {
    if (!bridge.value) {
      throw new Error('Platform bridge not initialized. Call init() first.');
    }
    return bridge.value;
  };

  const invoke = async <T>(cmd: string, args?: unknown): Promise<T> => {
    if (!bridge.value) {
      await init();
    }
    return getBridge().invoke<T>(cmd, args);
  };

  const invokeWithResult = async <T>(cmd: string, args?: unknown): Promise<CoreResult<T>> => {
    if (!bridge.value) {
      await init();
    }
    return getBridge().invokeWithResult<T>(cmd, args);
  };

  const detectHardware = (): HardwareProfile => {
    if (typeof navigator === 'undefined') {
      return 'medium';
    }
    interface NavigatorExtended extends Navigator {
      deviceMemory?: number;
    }
    const ram = (navigator as NavigatorExtended).deviceMemory || 4;
    const cores = navigator.hardwareConcurrency || 2;
    if (ram <= 4 && cores <= 2) {
      return 'low';
    } else if (ram <= 8 && cores <= 4) {
      return 'medium';
    } else {
      return 'high';
    }
  };

  return {
    bridge,
    isDesktop,
    isWeb: computed(() => !isDesktopEnvironment),
    isInitialized,
    init,
    getBridge,
    invoke,
    invokeWithResult,
    detectHardware,
  };
}

export type HardwareProfile = 'low' | 'medium' | 'high';
