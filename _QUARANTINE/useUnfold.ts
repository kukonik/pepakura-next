import { usePlatform } from '@pepakura/shared/composables/usePlatform';
import type { Mesh, UnfoldedMesh, UnfoldConfig, SvgExportConfig } from '@/types';

/**
 * Composable for unfold-related core commands.
 * Uses Platform Bridge to work on both Desktop (Tauri) and Web (WASM).
 */
export function useUnfold() {
  const { invoke, invokeWithResult } = usePlatform();

  const unfoldMesh = async (
    meshId: number,
    config: UnfoldConfig
  ): Promise<UnfoldedMesh> => {
    return await invoke<UnfoldedMesh>('unfold_mesh', { meshId, config });
  };

  const exportSvg = async (
    unfoldedId: number,
    path: string,
    config: SvgExportConfig
  ): Promise<void> => {
    return await invoke<void>('export_svg', { unfoldedId, path, config });
  };

  const importModel = async (path: string, format: string): Promise<Mesh> => {
    return await invoke<Mesh>('import_model', { path, format });
  };

  const createProject = async (name: string): Promise<string> => {
    return await invoke<string>('create_project', { name });
  };

  return {
    unfoldMesh,
    exportSvg,
    importModel,
    createProject,
  };
}