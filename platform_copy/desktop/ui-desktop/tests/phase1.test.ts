/**
 * Тесты для Phase 1 реализации.
 * 
 * Тестирует:
 * - PDF экспорт composable
 * - AI кэширование и стриминг
 * - 3D Viewer
 * - Редактор развёрток
 * - Персистентность
 */

import { describe, it, expect, beforeEach, vi } from 'vitest'
import { usePdfExport } from '@/composables/usePdfExport'
import { useAiStream } from '@/composables/useAiStream'
import { useUnfoldEditor } from '@/composables/useUnfoldEditor'
import { useInteractiveViewer3D } from '@/composables/useInteractiveViewer3D'
import { usePersistence } from '@/composables/usePersistence'
import { use3d2dLink } from '@/composables/use3d2dLink'

// ============================================================================
// PDF Export Tests
// ============================================================================

describe('usePdfExport', () => {
  it('should have default settings', () => {
    const { defaultSettings } = usePdfExport()
    
    expect(defaultSettings.pageSize).toBe('A4')
    expect(defaultSettings.orientation).toBe('portrait')
    expect(defaultSettings.scale).toBe(0)
    expect(defaultSettings.showFoldLines).toBe(true)
    expect(defaultSettings.showCutLines).toBe(true)
    expect(defaultSettings.showPartNumbers).toBe(true)
  })
  
  it('should merge custom settings with defaults', () => {
    const { exportPdf } = usePdfExport()
    
    // Mock invoke
    vi.mocked(invoke).mockResolvedValue('/path/to/file.pdf')
    
    const unfolded = {
      vertices_2d: [[0, 0], [1, 0], [1, 1]],
      faces: [{ face_id: 0, vertices_2d: [[0, 0], [1, 0], [1, 1]] }],
      source_mesh: null,
      metadata: null,
    }
    
    const result = exportPdf(unfolded, {
      pageSize: 'A3',
      orientation: 'landscape',
    })
    
    expect(result).toBeDefined()
  })
})

// ============================================================================
// AI Stream Tests
// ============================================================================

describe('useAiStream', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })
  
  it('should initialize with default state', () => {
    const { isStreaming, currentResponse, totalTokens, error, progress } = useAiStream()
    
    expect(isStreaming.value).toBe(false)
    expect(currentResponse.value).toBe('')
    expect(totalTokens.value).toBe(0)
    expect(error.value).toBe(null)
    expect(progress.value).toBe(0)
  })
  
  it('should stream chat response', async () => {
    const { streamChat, currentResponse, isStreaming } = useAiStream({
      onToken: vi.fn(),
      onComplete: vi.fn(),
    })
    
    // Mock invoke
    vi.mocked(invoke).mockResolvedValue(undefined)
    
    await streamChat('Test message')
    
    expect(invoke).toHaveBeenCalledWith('ai_chat_stream', expect.any(Object))
  })
  
  it('should handle streaming errors', async () => {
    const { streamChat, error } = useAiStream()
    
    vi.mocked(invoke).mockRejectedValue(new Error('AI unavailable'))
    
    await streamChat('Test')
    
    expect(error.value).toContain('AI unavailable')
  })
  
  it('should stop streaming', () => {
    const { streamChat, stopStreaming, isStreaming } = useAiStream()
    
    // Начать стриминг
    streamChat('Test')
    
    // Остановить
    stopStreaming()
    
    expect(isStreaming.value).toBe(false)
  })
  
  it('should get complete response', async () => {
    const { chatComplete } = useAiStream()
    
    vi.mocked(invoke).mockResolvedValue('Complete response')
    
    const result = await chatComplete('Test')
    
    expect(result).toBe('Complete response')
  })
})

// ============================================================================
// Unfold Editor Tests
// ============================================================================

describe('useUnfoldEditor', () => {
  it('should initialize with default settings', () => {
    const { settings } = useUnfoldEditor()
    
    expect(settings.gridSize).toBe(10)
    expect(settings.snapToGrid).toBe(true)
    expect(settings.snapToParts).toBe(false)
    expect(settings.showGrid).toBe(true)
  })
  
  it('should load parts', () => {
    const { loadParts, state } = useUnfoldEditor()
    
    const testParts = [
      {
        id: 'face-1',
        vertices: [[0, 0], [1, 0], [1, 1]],
        center: [0.5, 0.5] as [number, number],
        bounds: { x: 0, y: 0, width: 1, height: 1 },
        rotation: 0,
        flipped: false,
      },
    ]
    
    loadParts(testParts)
    
    expect(state.parts.length).toBe(1)
    expect(state.parts[0].id).toBe('face-1')
  })
  
  it('should select part', () => {
    const { selectPart, selectedParts, hasSelection } = useUnfoldEditor()
    
    selectPart('face-1')
    
    expect(hasSelection.value).toBe(true)
    expect(selectedParts.value.length).toBe(1)
  })
  
  it('should select multiple parts', () => {
    const { selectPart, selectedParts, isMultiSelect } = useUnfoldEditor()
    
    selectPart('face-1')
    selectPart('face-2', true)
    
    expect(isMultiSelect.value).toBe(true)
    expect(selectedParts.value.length).toBe(2)
  })
  
  it('should clear selection', () => {
    const { selectPart, clearSelection, hasSelection } = useUnfoldEditor()
    
    selectPart('face-1')
    clearSelection()
    
    expect(hasSelection.value).toBe(false)
  })
  
  it('should snap to grid', () => {
    const { snapToGrid } = useUnfoldEditor({ gridSize: 10 })
    
    expect(snapToGrid(23, 10)).toBe(20)
    expect(snapToGrid(27, 10)).toBe(30)
    expect(snapToGrid(25, 10)).toBe(20)
  })
  
  it('should rotate part', () => {
    const { loadParts, rotatePart, state } = useUnfoldEditor()
    
    loadParts([
      {
        id: 'face-1',
        vertices: [[0, 0], [1, 0], [0.5, 1]],
        center: [0.5, 0.5] as [number, number],
        bounds: { x: 0, y: 0, width: 1, height: 1 },
        rotation: 0,
        flipped: false,
      },
    ])
    
    rotatePart('face-1', 90)
    
    expect(state.parts[0].rotation).toBe(90)
  })
  
  it('should undo/redo', () => {
    const { loadParts, rotatePart, undo, redo, state } = useUnfoldEditor()
    
    loadParts([
      {
        id: 'face-1',
        vertices: [[0, 0], [1, 0], [0.5, 1]],
        center: [0.5, 0.5] as [number, number],
        bounds: { x: 0, y: 0, width: 1, height: 1 },
        rotation: 0,
        flipped: false,
      },
    ])
    
    rotatePart('face-1', 90)
    undo()
    
    expect(state.parts[0].rotation).toBe(0)
    
    redo()
    
    expect(state.parts[0].rotation).toBe(90)
  })
  
  it('should align parts', () => {
    const { loadParts, selectPart, alignParts, state } = useUnfoldEditor()
    
    loadParts([
      {
        id: 'face-1',
        vertices: [[0, 0], [1, 0], [0.5, 1]],
        center: [0.5, 0.5] as [number, number],
        bounds: { x: 0, y: 0, width: 1, height: 1 },
        rotation: 0,
        flipped: false,
      },
      {
        id: 'face-2',
        vertices: [[2, 0], [3, 0], [2.5, 1]],
        center: [2.5, 0.5] as [number, number],
        bounds: { x: 2, y: 0, width: 1, height: 1 },
        rotation: 0,
        flipped: false,
      },
    ])
    
    selectPart('face-1')
    selectPart('face-2', true)
    alignParts('center')
    
    // Части должны быть выровнены по центру
    expect(state.selectedPartIds.length).toBe(2)
  })
})

// ============================================================================
// 3D Viewer Tests
// ============================================================================

describe('useInteractiveViewer3D', () => {
  it('should initialize with default options', () => {
    const { isReady } = useInteractiveViewer3D()
    
    expect(isReady.value).toBe(false)
  })
  
  it('should calculate camera projection', () => {
    const { projectCameraTo2D } = useInteractiveViewer3D()
    
    const camera3d = {
      position: [3, 3, 3],
      target: [0, 0, 0],
    }
    
    const projection = projectCameraTo2D(camera3d)
    
    expect(projection.x).toBeGreaterThan(0)
    expect(projection.y).toBeGreaterThan(0)
    expect(projection.scale).toBeGreaterThan(0)
  })
})

// ============================================================================
// Persistence Tests
// ============================================================================

describe('usePersistence', () => {
  beforeEach(() => {
    vi.clearAllMocks()
  })
  
  it('should initialize with default state', () => {
    const { isSaving, isLoaded, lastSaveAt, error } = usePersistence()
    
    expect(isSaving.value).toBe(false)
    expect(isLoaded.value).toBe(false)
    expect(lastSaveAt.value).toBe(null)
    expect(error.value).toBe(null)
  })
  
  it('should save state', async () => {
    const { saveState, lastSaveAt } = usePersistence()
    
    vi.mocked(invoke).mockResolvedValue(undefined)
    
    const result = await saveState('test_key', { value: 42 })
    
    expect(result).toBe(true)
    expect(lastSaveAt.value).toBeInstanceOf(Date)
  })
  
  it('should load state', async () => {
    const { loadState } = usePersistence()
    
    vi.mocked(invoke).mockResolvedValue({ value: 42 })
    
    const result = await loadState('test_key')
    
    expect(result).toEqual({ value: 42 })
  })
  
  it('should handle save errors', async () => {
    const { saveState, error } = usePersistence()
    
    vi.mocked(invoke).mockRejectedValue(new Error('Save failed'))
    
    const result = await saveState('test_key', { value: 42 })
    
    expect(result).toBe(false)
    expect(error.value).toContain('Save failed')
  })
})

// ============================================================================
// 3D-2D Link Tests
// ============================================================================

describe('use3d2dLink', () => {
  it('should initialize with default state', () => {
    const { linkedFaceIndex, isSyncEnabled } = use3d2dLink()
    
    expect(linkedFaceIndex.value).toBe(null)
    expect(isSyncEnabled.value).toBe(true)
  })
  
  it('should highlight face in 2D', () => {
    const { highlightFaceIn2D, linkedFaceIndex } = use3d2dLink()
    
    const dispatchEventSpy = vi.spyOn(window, 'dispatchEvent')
    
    highlightFaceIn2D(5)
    
    expect(linkedFaceIndex.value).toBe(5)
    expect(dispatchEventSpy).toHaveBeenCalledWith(
      expect.any(CustomEvent)
    )
  })
  
  it('should clear highlight', () => {
    const { highlightFaceIn2D, clearHighlight, linkedFaceIndex } = use3d2dLink()
    
    highlightFaceIn2D(5)
    clearHighlight()
    
    expect(linkedFaceIndex.value).toBe(null)
  })
  
  it('should project 3D camera to 2D', () => {
    const { projectCameraTo2D } = use3d2dLink()
    
    const camera3d = {
      position: [3, 3, 3],
      target: [0, 0, 0],
    }
    
    const projection = projectCameraTo2D(camera3d)
    
    expect(projection.x).toBeDefined()
    expect(projection.y).toBeDefined()
    expect(projection.scale).toBeDefined()
  })
})

// ============================================================================
// Integration Tests
// ============================================================================

describe('Phase 1 Integration', () => {
  it('should complete full workflow', async () => {
    // 1. Загрузка данных
    const { loadParts, state } = useUnfoldEditor()
    loadParts([
      {
        id: 'face-1',
        vertices: [[0, 0], [1, 0], [1, 1]],
        center: [0.5, 0.5] as [number, number],
        bounds: { x: 0, y: 0, width: 1, height: 1 },
        rotation: 0,
        flipped: false,
      },
    ])
    
    // 2. Выделение
    const { selectPart } = useUnfoldEditor()
    selectPart('face-1')
    
    // 3. Поворот
    const { rotatePart } = useUnfoldEditor()
    rotatePart('face-1', 90)
    
    // 4. Сохранение
    const { saveState } = usePersistence()
    vi.mocked(invoke).mockResolvedValue(undefined)
    const saved = await saveState('unfold_state', state)
    
    expect(saved).toBe(true)
  })
})
