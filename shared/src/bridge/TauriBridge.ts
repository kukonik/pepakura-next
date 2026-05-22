import { invoke } from '@tauri-apps/api/core'
import type { PlatformBridge } from './PlatformBridge'

export class TauriBridge implements PlatformBridge {
  async init(): Promise<void> {
    // Проверка соединения с Rust
    try {
      // Можно пинговать health_check, но пока просто логируем
      console.log('[TauriBridge] Initialized')
    } catch (e) {
      console.error('[TauriBridge] Init failed', e)
      throw e
    }
  }

  async unfold_lscm(meshData: string, config: string): Promise<any> {
    // Важно: параметры должны совпадать с именами аргументов в Rust команде
    // В Rust: pub fn unfold_lscm(_mesh_data: String, _config: String)
    return await invoke('unfold_lscm', { meshData, config })
  }
}

export function createTauriBridge(): PlatformBridge {
  return new TauriBridge()
}
