import type { PlatformBridge } from './PlatformBridge';
import type { ParseResult, ObjGeometry } from '../types/core';
import type { UnfoldResult } from './types';

export class WasmBridge implements PlatformBridge {
  private initialized = false;

  isReady(): boolean {
    return this.initialized;
  }

  async initialize(): Promise<void> {
    this.initialized = true;
  }

  async invoke<T>(_cmd: string, _args?: Record<string, unknown>): Promise<T> {
    throw new Error('WASM bridge is not implemented yet');
  }

  async parseMockObj(_objString: string): Promise<ParseResult> {
    throw new Error('WASM bridge is not implemented yet');
  }

  async loadRealObj(_objString: string): Promise<ObjGeometry> {
    throw new Error('WASM bridge is not implemented yet');
  }

  async startMockUnfold(_totalFaces: number): Promise<string> {
    throw new Error('WASM bridge is not implemented yet');
  }

  async unfold_lscm(_meshJson: string, _optionsJson: string): Promise<UnfoldResult> {
    throw new Error('WASM bridge is not implemented yet');
  }

  async onUnfoldProgress(_callback: (percent: number, msg: string) => void): Promise<() => void> {
    return () => {};
  }
}

// ФАБРИКА, которую запрашивает usePlatform
export function createWasmBridge(_config: unknown): PlatformBridge {
  return new WasmBridge();
}