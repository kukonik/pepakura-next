import { invoke } from '@tauri-apps/api/core';
import type { PlatformBridge } from './PlatformBridge';
import type { BridgeResult } from './types';
import type {
  ModelData,
  MeshData,
  UnfoldConfig,
  UnfoldResult,
  ParseResult,
  ObjGeometry
} from '../types/core';

export class TauriBridge implements PlatformBridge {
  async init(): Promise<void> {
    console.log('[TauriBridge] Initialized');
  }

  isReady(): boolean {
    return true;
  }

  // Core
  async loadModel(path: string): Promise<BridgeResult<ModelData>> {
    return await invoke('load_model', { path });
  }

  async unfoldMesh(mesh: MeshData, config?: UnfoldConfig): Promise<BridgeResult<UnfoldResult>> {
    // пока заглушка, можно заменить на реальный вызов
    return await invoke('unfold_3d_model', { objData: '' });
  }

  async parseMockObj(objString: string): Promise<BridgeResult<ParseResult>> {
    return await invoke('parse_mock_obj');
  }

  async loadRealObj(objString: string): Promise<BridgeResult<ObjGeometry>> {
    return await invoke('load_real_obj');
  }

  // AI
  async generateFromText(prompt: string): Promise<BridgeResult<string>> {
    return await invoke('ai_generate_from_text', { text: prompt });
  }

  async generateFromImage(imagePath: string): Promise<BridgeResult<string>> {
    return await invoke('ai_generate_from_image', { img: [] });
  }

  // Экспорт
  async exportToSvg(unfoldId: number): Promise<BridgeResult<string>> {
    return await invoke('export_svg');
  }

  async exportToPdf(unfoldId: number, format: string): Promise<BridgeResult<Uint8Array>> {
    return await invoke('export_unfold_pdf');
  }

  // Утилиты
  async invoke<T>(command: string, args?: Record<string, unknown>): Promise<T> {
    return await invoke(command, args);
  }

  async invokeWithResult<T>(command: string, args?: Record<string, unknown>): Promise<BridgeResult<T>> {
    const data = await invoke(command, args);
    return { success: true, data } as BridgeResult<T>;
  }

  // Метод для развертки LSCM (основной сейчас)
  async unfold_lscm(meshData: string, config: string): Promise<any> {
    return await invoke('unfold_lscm', { meshData, config });
  }
}

export function createTauriBridge(): PlatformBridge {
  return new TauriBridge();
}
