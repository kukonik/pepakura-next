// pepakura-next/packages/ui-desktop/src/stores/undoRedoStore.ts
import { defineStore } from 'pinia'
import { PartOverride } from '@/../../shared/src/types/nesting'
import { useProjectStore } from './projectStore'

// Тип для действия отмены/повтора
export interface UndoAction {
  type: 'partOverride' | 'resetOverrides'
  partId?: number
  previousOverride?: PartOverride
  previousOverrides?: Record<number, PartOverride>
  newOverride?: PartOverride
}

export interface UndoRedoState {
  history: UndoAction[]
  currentIndex: number
  maxSize: number
}

export const useUndoRedoStore = defineStore('undoRedoStore', {
  state: (): UndoRedoState => ({
    history: [],
    currentIndex: -1,
    maxSize: 50 // Максимальное количество действий в истории
  }),

  getters: {
    canUndo: (state) => state.currentIndex >= 0,
    canRedo: (state) => state.currentIndex < state.history.length - 1,
    historyLength: (state) => state.history.length
  },

  actions: {
    // Добавить действие в историю
    addPartOverrideAction(partId: number, previousOverride: PartOverride | undefined, newOverride: PartOverride) {
      const projectStore = useProjectStore()
      
      // Удаляем все действия после текущего индекса (если есть)
      if (this.currentIndex < this.history.length - 1) {
        this.history = this.history.slice(0, this.currentIndex + 1)
      }
      
      // Создаем действие
      const action: UndoAction = {
        type: 'partOverride',
        partId,
        previousOverride,
        newOverride
      }
      
      // Добавляем действие в историю
      this.history.push(action)
      
      // Ограничиваем размер истории
      if (this.history.length > this.maxSize) {
        this.history.shift()
        this.currentIndex = Math.max(0, this.currentIndex - 1)
      } else {
        this.currentIndex++
      }
    },

    // Добавить действие сброса всех переопределений
    addResetOverridesAction(previousOverrides: Record<number, PartOverride>) {
      // Удаляем все действия после текущего индекса (если есть)
      if (this.currentIndex < this.history.length - 1) {
        this.history = this.history.slice(0, this.currentIndex + 1)
      }
      
      // Создаем действие
      const action: UndoAction = {
        type: 'resetOverrides',
        previousOverrides
      }
      
      // Добавляем действие в историю
      this.history.push(action)
      
      // Ограничиваем размер истории
      if (this.history.length > this.maxSize) {
        this.history.shift()
        this.currentIndex = Math.max(0, this.currentIndex - 1)
      } else {
        this.currentIndex++
      }
    },

    // Отменить последнее действие
    undo() {
      if (!this.canUndo) return
      
      const projectStore = useProjectStore()
      const action = this.history[this.currentIndex]
      
      switch (action.type) {
        case 'partOverride':
          // Восстанавливаем предыдущее состояние переопределения
          if (action.previousOverride) {
            projectStore.setPartOverride(action.previousOverride)
          } else if (action.partId !== undefined) {
            // Если не было предыдущего переопределения, удаляем текущее
            projectStore.removePartOverride(action.partId)
          }
          break
          
        case 'resetOverrides':
          // Восстанавливаем все предыдущие переопределения
          if (action.previousOverrides) {
            projectStore.partOverrides = { ...action.previousOverrides }
          } else {
            projectStore.resetPartOverrides()
          }
          break
      }
      
      // Перемещаем указатель назад
      this.currentIndex--
    },

    // Повторить последнее отмененное действие
    redo() {
      if (!this.canRedo) return
      
      const projectStore = useProjectStore()
      // Перемещаем указатель вперед перед выполнением действия
      this.currentIndex++
      const action = this.history[this.currentIndex]
      
      switch (action.type) {
        case 'partOverride':
          // Применяем новое переопределение
          if (action.newOverride) {
            projectStore.setPartOverride(action.newOverride)
          } else if (action.partId !== undefined) {
            // Если новое переопределение отсутствует, удаляем текущее
            projectStore.removePartOverride(action.partId)
          }
          break
          
        case 'resetOverrides':
          // Сбрасываем все переопределения
          projectStore.resetPartOverrides()
          break
      }
    },

    // Очистить историю
    clearHistory() {
      this.history = []
      this.currentIndex = -1
    }
  }
})