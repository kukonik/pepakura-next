<template>
  <div class="viewer-container">
    <div
      v-if="!modelPath"
      class="viewer-placeholder"
      @dragover.prevent
      @drop.prevent="onDrop"
    >
      <h3 class="placeholder-title">Загрузите модель для начала работы</h3>
      <p class="placeholder-text">
        Перетащите файл сюда или используйте кнопки импорта.
        Поддерживаемые форматы: OBJ+MTL, STL, GLTF, GLB
      </p>
      <input
        ref="fileInput"
        type="file"
        multiple
        class="hidden-input"
        @change="onFileInput"
        accept=".obj,.mtl,.stl,.gltf,.glb"
      />
      <button type="button" class="placeholder-button" @click="fileInput?.click()">
        Выбрать файл
      </button>
      <button
        type="button"
        class="placeholder-button secondary"
        @click="onImportFromDialog"
      >
        Импортировать через диалог
      </button>
    </div>

    <div v-else class="viewer-loaded">
      <div class="viewer-toolbar">
        <span class="file-label">
          {{ shortName }}
          <span class="format-pill">{{ formatLabel }}</span>
        </span>
        <div class="toolbar-actions">
          <button class="toolbar-btn" @click="resetModel">Очистить</button>
          <button class="toolbar-btn" @click="fileInput?.click()">Заменить файл</button>
        </div>
      </div>

      <div class="viewer-scene-wrapper">
        <Scene3D
          class="viewer-canvas"
          :model-path="modelPath"
          :format="format"
          :mtl-path="mtlPath"
        />
      </div>

      <input
        ref="fileInput"
        type="file"
        multiple
        class="hidden-input"
        @change="onFileInput"
        accept=".obj,.mtl,.stl,.gltf,.glb"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, onBeforeUnmount, onMounted, ref } from 'vue'
import Scene3D from './Scene3D.vue'
import { useImport3DModel } from '@/composables/useImport3DModel'
import { useProjectStore } from '@/stores/project'

type SupportedFormat = 'obj' | 'obj+mtl' | 'stl' | 'gltf' | 'glb'

declare global {
  interface Window {
    __pep_handleDropFiles?: (files: FileList) => void
  }
}

const project = useProjectStore()

const modelPath = ref<string | null>(null)
const mtlPath = ref<string | null>(null)
const format = ref<SupportedFormat | null>(null)

const objectUrls = ref<string[]>([])
const fileInput = ref<HTMLInputElement | null>(null)

const { importFromDialog, error } = useImport3DModel()

const shortName = computed(() => {
  if (!modelPath.value) return ''
  try {
    const url = new URL(modelPath.value)
    const parts = url.pathname.split('/')
    return parts[parts.length - 1]
  } catch {
    return modelPath.value
  }
})

const formatLabel = computed(() => {
  switch (format.value) {
    case 'obj+mtl':
      return 'OBJ + MTL'
    case 'obj':
      return 'OBJ'
    case 'stl':
      return 'STL'
    case 'gltf':
      return 'GLTF'
    case 'glb':
      return 'GLB'
    default:
      return 'Неизвестный формат'
  }
})

function revokeAll() {
  objectUrls.value.forEach(u => URL.revokeObjectURL(u))
  objectUrls.value = []
}

function sameBase(a: File, b: File) {
  const na = a.name.toLowerCase()
  const nb = b.name.toLowerCase()
  return na.replace(/\.[^.]+$/, '') === nb.replace(/\.[^.]+$/, '')
}

function resetModel() {
  revokeAll()
  modelPath.value = null
  mtlPath.value = null
  format.value = null
  project.set3DModel({
    sourcePath: null,
    workingPath: null,
    format: null,
    faces: null,
    parts: null
  })
}

function syncToProject() {
  project.set3DModel({
    sourcePath: modelPath.value,
    workingPath: modelPath.value,
    format: (format.value as any) ?? null
  })
}

function handleFiles(files: FileList | File[]) {
  const list = Array.from(files)
  console.log('[ModelViewer] handleFiles:', list.map(f => f.name))

  resetModel()

  const objFile = list.find(f => f.name.toLowerCase().endsWith('.obj'))
  const mtlFile = list.find(
    f => f.name.toLowerCase().endsWith('.mtl') && objFile && sameBase(f, objFile)
  )
  const stlFile = list.find(f => f.name.toLowerCase().endsWith('.stl'))
  const gltfFile = list.find(f => f.name.toLowerCase().endsWith('.gltf'))
  const glbFile = list.find(f => f.name.toLowerCase().endsWith('.glb'))

  console.log('[ModelViewer] picked:', {
    objFile: objFile?.name,
    mtlFile: mtlFile?.name,
    stlFile: stlFile?.name,
    gltfFile: gltfFile?.name,
    glbFile: glbFile?.name
  })

  if (objFile && mtlFile) {
    const objUrl = URL.createObjectURL(objFile)
    const mtlUrl = URL.createObjectURL(mtlFile)
    objectUrls.value.push(objUrl, mtlUrl)
    modelPath.value = objUrl
    mtlPath.value = mtlUrl
    format.value = 'obj+mtl'
    syncToProject()
    return
  }

  if (stlFile) {
    const url = URL.createObjectURL(stlFile)
    objectUrls.value.push(url)
    modelPath.value = url
    format.value = 'stl'
    syncToProject()
    return
  }

  if (gltfFile) {
    const url = URL.createObjectURL(gltfFile)
    objectUrls.value.push(url)
    modelPath.value = url
    format.value = 'gltf'
    syncToProject()
    return
  }

  if (glbFile) {
    const url = URL.createObjectURL(glbFile)
    objectUrls.value.push(url)
    modelPath.value = url
    format.value = 'glb'
    syncToProject()
    return
  }

  if (objFile) {
    const url = URL.createObjectURL(objFile)
    objectUrls.value.push(url)
    modelPath.value = url
    format.value = 'obj'
    syncToProject()
    return
  }

  console.warn('[ModelViewer] Не удалось определить поддерживаемый формат файлов')
}

function onDrop(e: DragEvent) {
  console.log('ModelViewer onDrop', e, e.dataTransfer?.files)
  const files = e.dataTransfer?.files
  if (!files || files.length === 0) return
  handleFiles(files)
}

function onFileInput(e: Event) {
  const input = e.target as HTMLInputElement
  if (!input.files || input.files.length === 0) return
  handleFiles(input.files)
  input.value = ''
}

async function onImportFromDialog() {
  const result = await importFromDialog()
  if (!result) return

  const path = result.path
  modelPath.value = `file:///${path.replace(/\\/g, '/')}`

  let fmt: SupportedFormat | null = null
  if (result.format && result.format !== 'auto') {
    fmt = result.format as SupportedFormat
  } else {
    const lower = path.toLowerCase()
    if (lower.endsWith('.obj')) fmt = 'obj'
    else if (lower.endsWith('.stl')) fmt = 'stl'
    else if (lower.endsWith('.gltf')) fmt = 'gltf'
    else if (lower.endsWith('.glb')) fmt = 'glb'
  }

  format.value = fmt
  mtlPath.value = null

  syncToProject()

  if (!format.value) {
    console.error('Cannot detect format for path:', path)
  }

  if (error.value) {
    console.error('Import 3D (dialog) error:', error.value)
  }
}

onMounted(() => {
  window.__pep_handleDropFiles = (files: FileList) => {
    console.log('__pep_handleDropFiles', files)
    handleFiles(files)
  }
})

onBeforeUnmount(() => {
  resetModel()
  window.__pep_handleDropFiles = undefined
})
</script>

<style scoped>
.viewer-container {
  width: 100%;
  height: 100%;
}

.viewer-placeholder {
  width: 100%;
  height: 100%;
  border: 1px dashed #4b5563;
  border-radius: 0.5rem;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  color: #9ca3af;
  text-align: center;
  padding: 1.5rem;
  gap: 0.5rem;
}

.placeholder-title {
  font-size: 1.125rem;
}

.placeholder-text {
  font-size: 0.875rem;
}

.placeholder-button {
  padding: 0.5rem 1rem;
  border-radius: 0.375rem;
  border: none;
  background-color: #2563eb;
  color: #fff;
  cursor: pointer;
}

.placeholder-button.secondary {
  background-color: #4b5563;
}

.placeholder-button:hover {
  opacity: 0.9;
}

.hidden-input {
  display: none;
}

.viewer-loaded {
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
}

.viewer-toolbar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0.35rem 0.75rem;
  font-size: 0.8rem;
  color: #e5e7eb;
  background: rgba(15, 23, 42, 0.95);
  border-bottom: 1px solid rgba(148, 163, 184, 0.4);
}

.file-label {
  display: flex;
  align-items: center;
  gap: 0.4rem;
  max-width: 360px;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.format-pill {
  padding: 0.1rem 0.5rem;
  border-radius: 999px;
  background: rgba(59, 130, 246, 0.15);
  border: 1px solid rgba(59, 130, 246, 0.5);
  font-size: 0.7rem;
  color: #bfdbfe;
}

.toolbar-actions {
  display: flex;
  gap: 0.4rem;
}

.toolbar-btn {
  border-radius: 999px;
  border: 1px solid rgba(148, 163, 184, 0.6);
  background: rgba(15, 23, 42, 0.9);
  color: #e5e7eb;
  font-size: 0.75rem;
  padding: 0.25rem 0.7rem;
  cursor: pointer;
}

.toolbar-btn:hover {
  background: rgba(30, 64, 175, 0.9);
}

.viewer-scene-wrapper {
  flex: 1;
  min-height: 0;
}

.viewer-canvas {
  width: 100%;
  height: 100%;
}
</style>
