import { ref } from 'vue'
import { useProjectStore } from '@/stores/project'

export function useImport3DModel() {
  const project = useProjectStore()
  const isLoading = ref(false)
  const lastError = ref<string | null>(null)

  async function importFromFiles(_files: FileList | File[]) {
    // На данном этапе игнорируем реальные файлы и берём демо‑модель из public.
    // public/models/model.obj -> /models/model.obj
    isLoading.value = true
    lastError.value = null

    try {
      project.setThreeD({
        sourcePath: 'models/model.obj',
        workingPath: 'models/model.obj',
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
