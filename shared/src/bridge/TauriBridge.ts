import type { PlatformBridge } from './PlatformBridge';
import type { ParseResult, ObjGeometry } from '../types/core';
import type { UnfoldResult } from './types';

export class TauriBridge implements PlatformBridge {
  private initialized = false;

  isReady(): boolean {
    return this.initialized;
  }

  async initialize(): Promise<void> {
    this.initialized = true;
  }

  async invoke<T>(cmd: string, args?: Record<string, unknown>): Promise<T> {
    const { invoke } = await import('@tauri-apps/api/core');
    return invoke<T>(cmd, args);
  }

  async parseMockObj(objString: string): Promise<ParseResult> {
    return this.invoke<ParseResult>('parse_mock_obj', { objString });
  }

  async loadRealObj(objString: string): Promise<ObjGeometry> {
    return this.invoke<ObjGeometry>('load_real_obj', { objString });
  }

  async startMockUnfold(totalFaces: number): Promise<string> {
    return this.invoke<string>('start_mock_unfold', { totalFaces });
  }

  async unfold_lscm(meshJson: string, optionsJson: string): Promise<UnfoldResult> {
    return this.invoke<UnfoldResult>('unfold_lscm', { meshJson, optionsJson });
  }

  async onUnfoldProgress(callback: (percent: number, msg: string) => void): Promise<() => void> {
    const { listen } = await import('@tauri-apps/api/event');
    const unlisten = await listen<{ percent: number; message: string }>(
      'unfold-progress',
      (event) => {
        callback(event.payload.percent, event.payload.message);
      }
    );
    return unlisten;
  }
}

// ФАБРИКА, которую запрашивает usePlatform
export function createTauriBridge(): PlatformBridge {
  return new TauriBridge();
}