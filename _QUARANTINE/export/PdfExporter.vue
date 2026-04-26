<template>
  <div class="pdf-export-dialog" @click.self="close">
    <div class="dialog-content">
      <h2>Экспорт в PDF</h2>

      <!-- Настройки экспорта -->
      <div class="export-settings">
        <!-- Размер страницы -->
        <div class="setting-group">
          <label>Размер страницы</label>
          <select v-model="settings.pageSize">
            <option value="A4">A4 (210 × 297 мм)</option>
            <option value="A3">A3 (297 × 420 мм)</option>
            <option value="A2">A2 (420 × 594 мм)</option>
            <option value="A1">A1 (594 × 841 мм)</option>
          </select>
        </div>

        <!-- Ориентация -->
        <div class="setting-group">
          <label>Ориентация</label>
          <div class="orientation-buttons">
            <button
              :class="{ active: settings.orientation === 'portrait' }"
              @click="settings.orientation = 'portrait'"
            >
              <span class="icon-portrait">📄</span>
              <span>Книжная</span>
            </button>
            <button
              :class="{ active: settings.orientation === 'landscape' }"
              @click="settings.orientation = 'landscape'"
            >
              <span class="icon-landscape">📄</span>
              <span>Альбомная</span>
            </button>
          </div>
        </div>

        <!-- Масштаб -->
        <div class="setting-group">
          <label>
            Масштаб
            <span class="hint">(0 = авто)</span>
          </label>
          <input
            type="number"
            v-model.number="settings.scale"
            step="0.1"
            min="0"
            max="10"
          />
        </div>

        <!-- Слои -->
        <div class="setting-group">
          <label>Слои</label>
          <div class="checkbox-group">
            <label class="checkbox-label">
              <input type="checkbox" v-model="settings.showCutLines" />
              <span class="color-indicator cut"></span>
              Линии реза
            </label>
            <label class="checkbox-label">
              <input type="checkbox" v-model="settings.showFoldLines" />
              <span class="color-indicator fold"></span>
              Линии сгиба
            </label>
            <label class="checkbox-label">
              <input type="checkbox" v-model="settings.showPartNumbers" />
              <span class="color-indicator number"></span>
              Номера деталей
            </label>
          </div>
        </div>

        <!-- Предпросмотр -->
        <div class="preview-section" v-if="unfolded">
          <label>Предпросмотр</label>
          <div class="preview-container">
            <div
              class="preview-page"
              :class="{ landscape: settings.orientation === 'landscape' }"
            >
              <svg viewBox="0 0 210 297" preserveAspectRatio="xMidYMid meet">
                <rect
                  x="0"
                  y="0"
                  :width="settings.orientation === 'landscape' ? 297 : 210"
                  :height="settings.orientation === 'landscape' ? 210 : 297"
                  fill="white"
                  stroke="#ccc"
                  stroke-width="0.5"
                />
                <!-- Упрощённая визуализация развёртки -->
                <g transform="translate(105, 148.5) scale(0.5)">
                  <path
                    v-for="(face, idx) in unfolded.faces"
                    :key="idx"
                    :d="getFacePath(face)"
                    fill="none"
                    stroke="#ff0000"
                    stroke-width="0.5"
                  />
                </g>
              </svg>
            </div>
          </div>
        </div>
      </div>

      <!-- Кнопки действий -->
      <div class="dialog-actions">
        <button class="btn-secondary" @click="close">Отмена</button>
        <button
          class="btn-primary"
          @click="exportToPdf"
          :disabled="isExporting"
        >
          {{ isExporting ? 'Экспорт...' : 'Экспортировать' }}
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { save } from '@tauri-apps/plugin-dialog'

interface UnfoldedFace {
  face_id: number
  vertices_2d: number[][]
  center: number[]
  bounds: { x: number, y: number, width: number, height: number }
}

interface UnfoldedMesh {
  vertices_2d: number[][]
  faces: UnfoldedFace[]
  source_mesh: any
  metadata: any
}

interface PdfSettings {
  pageSize: 'A4' | 'A3' | 'A2' | 'A1'
  orientation: 'portrait' | 'landscape'
  scale: number
  showCutLines: boolean
  showFoldLines: boolean
  showPartNumbers: boolean
}

const props = defineProps<{
  unfolded: UnfoldedMesh | null
}>()

const emit = defineEmits<{
  close: []
  exported: [path: string]
}>()

const isExporting = ref(false)

const settings = reactive<PdfSettings>({
  pageSize: 'A4',
  orientation: 'portrait',
  scale: 0, // 0 = авто
  showCutLines: true,
  showFoldLines: true,
  showPartNumbers: true,
})

const close = () => {
  emit('close')
}

const getFacePath = (face: UnfoldedFace): string => {
  if (face.vertices_2d.length < 2) return ''
  const [x, y] = face.vertices_2d[0]
  let path = `M ${x} ${y}`
  for (let i = 1; i < face.vertices_2d.length; i++) {
    const [vx, vy] = face.vertices_2d[i]
    path += ` L ${vx} ${vy}`
  }
  path += ' Z'
  return path
}

const exportToPdf = async () => {
  if (!props.unfolded) {
    alert('Нет данных для экспорта')
    return
  }

  isExporting.value = true

  try {
    // Предлагаем пользователю выбрать путь для сохранения
    const filePath = await save({
      filters: [{ name: 'PDF', extensions: ['pdf'] }],
      defaultPath: 'pepakura-export.pdf',
    })

    if (!filePath) {
      // Пользователь отменил выбор
      isExporting.value = false
      return
    }

    // Вызываем Tauri команду для экспорта
    const resultPath = await invoke<string>('export_unfold_pdf', {
      unfolded: props.unfolded,
      outputPath: filePath,
      pageSize: settings.pageSize,
      scale: settings.scale,
      showFoldLines: settings.showFoldLines,
      showCutLines: settings.showCutLines,
      showPartNumbers: settings.showPartNumbers,
      orientation: settings.orientation,
    })

    emit('exported', resultPath)
    close()
  } catch (error) {
    console.error('PDF export error:', error)
    alert(`Ошибка экспорта: ${error instanceof Error ? error.message : String(error)}`)
  } finally {
    isExporting.value = false
  }
}
</script>

<style scoped>
.pdf-export-dialog {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.dialog-content {
  background: var(--bg-primary, #fff);
  border-radius: 12px;
  padding: 24px;
  max-width: 600px;
  width: 90%;
  max-height: 90vh;
  overflow-y: auto;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.2);
}

h2 {
  margin: 0 0 20px;
  font-size: 20px;
  color: var(--text-primary, #333);
}

.export-settings {
  display: flex;
  flex-direction: column;
  gap: 20px;
  margin-bottom: 24px;
}

.setting-group {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.setting-group label {
  font-weight: 600;
  font-size: 14px;
  color: var(--text-primary, #333);
}

.setting-group select,
.setting-group input[type="number"] {
  padding: 8px 12px;
  border: 1px solid var(--border-color, #ddd);
  border-radius: 6px;
  font-size: 14px;
  background: var(--bg-secondary, #f9f9f9);
  color: var(--text-primary, #333);
}

.hint {
  font-weight: normal;
  font-size: 12px;
  color: var(--text-secondary, #999);
  margin-left: 4px;
}

.orientation-buttons {
  display: flex;
  gap: 12px;
}

.orientation-buttons button {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 8px;
  padding: 12px;
  border: 2px solid var(--border-color, #ddd);
  border-radius: 8px;
  background: var(--bg-secondary, #f9f9f9);
  cursor: pointer;
  transition: all 0.2s;
}

.orientation-buttons button.active {
  border-color: var(--accent-color, #4a9eff);
  background: var(--accent-light, #e8f4ff);
}

.orientation-buttons button:hover {
  border-color: var(--accent-color, #4a9eff);
}

.icon-portrait,
.icon-landscape {
  font-size: 24px;
}

.icon-landscape {
  transform: rotate(90deg);
  display: inline-block;
}

.checkbox-group {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.checkbox-label {
  display: flex;
  align-items: center;
  gap: 8px;
  cursor: pointer;
  font-weight: normal;
}

.checkbox-label input[type="checkbox"] {
  width: 18px;
  height: 18px;
  cursor: pointer;
}

.color-indicator {
  width: 16px;
  height: 16px;
  border-radius: 3px;
  display: inline-block;
}

.color-indicator.cut {
  background: #ff0000;
}

.color-indicator.fold {
  background: #0000ff;
}

.color-indicator.number {
  background: #000000;
}

.preview-section {
  margin-top: 10px;
}

.preview-container {
  display: flex;
  justify-content: center;
  padding: 16px;
  background: var(--bg-secondary, #f5f5f5);
  border-radius: 8px;
}

.preview-page {
  width: 150px;
  height: 212px;
  background: white;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  border: 1px solid #ddd;
}

.preview-page.landscape {
  width: 212px;
  height: 150px;
}

.preview-page svg {
  width: 100%;
  height: 100%;
}

.dialog-actions {
  display: flex;
  justify-content: flex-end;
  gap: 12px;
  padding-top: 16px;
  border-top: 1px solid var(--border-color, #eee);
}

.btn-secondary,
.btn-primary {
  padding: 10px 20px;
  border-radius: 6px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
}

.btn-secondary {
  background: var(--bg-secondary, #f5f5f5);
  border: 1px solid var(--border-color, #ddd);
  color: var(--text-primary, #333);
}

.btn-secondary:hover {
  background: var(--bg-tertiary, #e5e5e5);
}

.btn-primary {
  background: var(--accent-color, #4a9eff);
  border: 1px solid var(--accent-color, #4a9eff);
  color: white;
}

.btn-primary:hover:not(:disabled) {
  background: var(--accent-hover, #3a8eef);
}

.btn-primary:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}
</style>
