import { ref } from 'vue'
import { usePlatform } from '@pepakura/shared/composables/usePlatform'

export interface PdfExportSettings {
  pageSize: 'A4' | 'A3' | 'A2' | 'A1'
  orientation: 'portrait' | 'landscape'
  scale: number
  showFoldLines: boolean
  showCutLines: boolean
  showPartNumbers: boolean
}

export interface UnfoldedFace {
  face_id: number
  vertices_2d: number[][]
  center: number[]
  bounds: { x: number, y: number, width: number, height: number }
}

export interface UnfoldedMesh {
  vertices_2d: number[][]
  faces: UnfoldedFace[]
  source_mesh: any
  metadata: any
}

export function usePdfExport() {
  const isExporting = ref(false)
  const error = ref<string | null>(null)
  const { getBridge } = usePlatform()

  const defaultSettings: PdfExportSettings = {
    pageSize: 'A4',
    orientation: 'portrait',
    scale: 0,
    showFoldLines: true,
    showCutLines: true,
    showPartNumbers: true,
  }

  const exportPdf = async (
    unfolded: UnfoldedMesh,
    settings: Partial<PdfExportSettings> = {}
  ): Promise<string | null> => {
    isExporting.value = true
    error.value = null

    const finalSettings = { ...defaultSettings, ...settings }
    const bridge = getBridge()

    try {
      // Используем bridge для сохранения файла через диалог
      const data = await bridge.exportPDF(unfolded, finalSettings)
      // Предлагаем пользователю выбрать путь для сохранения
      const success = await bridge.saveFileDialog(data, 'pepakura-export.pdf')
      if (!success) {
        isExporting.value = false
        return null
      }
      // Возвращаем путь? В Web нет пути, но можно вернуть идентификатор.
      // Для простоты возвращаем пустую строку.
      return ''
    } catch (e) {
      error.value = e instanceof Error ? e.message : String(e)
      console.error('PDF export error:', error.value)
      throw error.value
    } finally {
      isExporting.value = false
    }
  }

  const exportPdfBytes = async (
    unfolded: UnfoldedMesh,
    settings: Partial<PdfExportSettings> = {}
  ): Promise<Uint8Array | null> => {
    isExporting.value = true
    error.value = null

    const finalSettings = { ...defaultSettings, ...settings }
    const bridge = getBridge()

    try {
      // Получаем PDF как bytes
      const bytes = await bridge.exportPDF(unfolded, finalSettings)
      return bytes
    } catch (e) {
      error.value = e instanceof Error ? e.message : String(e)
      console.error('PDF export error:', error.value)
      throw error.value
    } finally {
      isExporting.value = false
    }
  }

  return {
    isExporting,
    error,
    exportPdf,
    exportPdfBytes,
    defaultSettings,
  }
}