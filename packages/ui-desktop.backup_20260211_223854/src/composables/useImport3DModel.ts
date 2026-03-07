// pepakura-next/ui-desktop/src/composables/useImport3DModel.ts
import { ref } from 'vue'
import { useProjectStore } from '@/stores/project'

export function useImport3DModel() {
  const project = useProjectStore()
  const isLoading = ref(false)
  const lastError = ref<string | null>(null)

  /**
   * Унифицированный импорт 3D‑модели.
   * Сейчас: демо‑режим, берём OBJ из pepakura-next/ui-desktop/public/models/model.obj
   * (в рантайме это /models/model.obj).
   */
  async function importFromFiles(_files: FileList | File[]) {
    isLoading.value = true
    lastError.value = null

    try {
      const objUrl = 'models/model.obj' // /public/models/model.obj

      project.setThreeD({
        sourcePath: objUrl,
        workingPath: objUrl,
        mtlPath: null,
        format: 'obj+mtl',
        status: 'ready',
        faces: 4460,
        parts: 3,
        lastError: null
      })
      project.recomputeUnfoldEstimates()
    } catch (e) {
      console.error('Import 3D demo model failed', e)
      project.markThreeDError('Не удалось подготовить демо‑модель.')
      lastError.value = 'Не удалось подготовить демо‑модель.'
    } finally {
      isLoading.value = false
    }
  }

  function reset() {
    project.resetThreeD()
  }

  return {
    isLoading,
    lastError,
    importFromFiles,
    reset
  }
}
