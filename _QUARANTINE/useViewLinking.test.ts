/**
 * Тесты для useViewLinking composable.
 */

import { describe, it, expect, beforeEach } from 'vitest'
import { useViewLinking } from '../useViewLinking'

describe('useViewLinking', () => {
  let linking: ReturnType<typeof useViewLinking>

  beforeEach(() => {
    linking = useViewLinking()
  })

  it('должен инициализироваться с null значениями', () => {
    expect(linking.selectedFace2D.value).toBeNull()
    expect(linking.selectedFace3D.value).toBeNull()
    expect(linking.hoveredFace2D.value).toBeNull()
    expect(linking.hoveredFace3D.value).toBeNull()
  })

  it('должен выделять грань в 2D', () => {
    linking.selectFace2D(5)
    expect(linking.selectedFace2D.value).toBe(5)
    expect(linking.selectedFace3D.value).toBe(5)
  })

  it('должен выделять грань в 3D', () => {
    linking.selectFace3D(3)
    expect(linking.selectedFace3D.value).toBe(3)
    expect(linking.selectedFace2D.value).toBe(3)
  })

  it('должен наводить грань в 2D', () => {
    linking.hoverFace2D(7)
    expect(linking.hoveredFace2D.value).toBe(7)
    expect(linking.hoveredFace3D.value).toBe(7)
  })

  it('должен наводить грань в 3D', () => {
    linking.hoverFace3D(9)
    expect(linking.hoveredFace3D.value).toBe(9)
    expect(linking.hoveredFace2D.value).toBe(9)
  })

  it('должен сбрасывать выделение', () => {
    linking.selectFace2D(5)
    linking.hoverFace3D(7)
    
    linking.clearSelection()
    
    expect(linking.selectedFace2D.value).toBeNull()
    expect(linking.selectedFace3D.value).toBeNull()
    expect(linking.hoveredFace2D.value).toBeNull()
    expect(linking.hoveredFace3D.value).toBeNull()
  })

  it('должен проверять выделение грани', () => {
    linking.selectFace2D(5)
    
    expect(linking.isFaceSelected(5)).toBe(true)
    expect(linking.isFaceSelected(3)).toBe(false)
  })

  it('должен проверять наведение грани', () => {
    linking.hoverFace3D(7)
    
    expect(linking.isFaceHovered(7)).toBe(true)
    expect(linking.isFaceHovered(5)).toBe(false)
  })

  it('должен синхронизировать выделение 2D → 3D', () => {
    linking.selectFace2D(10)
    expect(linking.selectedFace3D.value).toBe(10)
  })

  it('должен синхронизировать выделение 3D → 2D', () => {
    linking.selectFace3D(15)
    expect(linking.selectedFace2D.value).toBe(15)
  })

  it('должен обрабатывать null выделение', () => {
    linking.selectFace2D(5)
    linking.selectFace2D(null)
    
    expect(linking.selectedFace2D.value).toBeNull()
    expect(linking.selectedFace3D.value).toBeNull()
  })
})
