/**
 * Сервис для управления горячими клавишами
 */

import { commandRegistry, executeCommand } from '@/commands/commands.registry'

export class HotkeysService {
  private isEnabled = true
  private registeredHandlers: Map<string, (event: KeyboardEvent) => void> = new Map()

  constructor() {
    this.setupGlobalHotkeys()
  }

  // Включение/выключение горячих клавиш
  enable() {
    this.isEnabled = true
  }

  disable() {
    this.isEnabled = false
  }

  // Регистрация глобальных горячих клавиш
  private setupGlobalHotkeys() {
    const handler = (event: KeyboardEvent) => {
      if (!this.isEnabled) return

      // Игнорируем ввод в полях ввода
      if (this.isInputElement(event.target as HTMLElement)) {
        return
      }

      // Проверяем комбинации клавиш
      this.handleKeyCombination(event)
    }

    window.addEventListener('keydown', handler)
    this.registeredHandlers.set('global', handler)
  }

  // Проверка, является ли элемент полем ввода
  private isInputElement(element: HTMLElement): boolean {
    if (!element) return false
    
    const tagName = element.tagName.toLowerCase()
    const inputTypes = ['input', 'textarea', 'select']
    
    if (inputTypes.includes(tagName)) {
      return true
    }

    // Проверка contenteditable
    if (element.isContentEditable) {
      return true
    }

    return false
  }

  // Обработка комбинаций клавиш
  private handleKeyCombination(event: KeyboardEvent) {
    const key = event.key.toLowerCase()
    const ctrl = event.ctrlKey || event.metaKey // Cmd на Mac
    const shift = event.shiftKey
    const alt = event.altKey

    // Command Palette: Ctrl+K или Cmd+K
    if (ctrl && key === 'k' && !shift && !alt) {
      event.preventDefault()
      this.triggerCommandPalette()
      return
    }

    // Быстрые команды (только когда палитра закрыта)
    if (!this.isCommandPaletteOpen()) {
      // Экспорт PDF: Ctrl+Shift+P
      if (ctrl && shift && key === 'p') {
        event.preventDefault()
        executeCommand('file.export.pdf')
        return
      }

      // Экспорт SVG: Ctrl+Shift+S
      if (ctrl && shift && key === 's') {
        event.preventDefault()
        executeCommand('file.export.svg')
        return
      }

      // Развертка модели: Ctrl+U
      if (ctrl && key === 'u' && !shift) {
        event.preventDefault()
        executeCommand('tools.unfold')
        return
      }

      // Генерация AI модели: Ctrl+G
      if (ctrl && key === 'g' && !shift) {
        event.preventDefault()
        executeCommand('tools.ai.generate')
        return
      }

      // Сохранение: Ctrl+S
      if (ctrl && key === 's' && !shift) {
        event.preventDefault()
        executeCommand('file.save')
        return
      }

      // Открытие: Ctrl+O
      if (ctrl && key === 'o' && !shift) {
        event.preventDefault()
        executeCommand('file.open')
        return
      }

      // Новая модель: Ctrl+N
      if (ctrl && key === 'n' && !shift) {
        event.preventDefault()
        executeCommand('file.new')
        return
      }

      // Отмена: Ctrl+Z
      if (ctrl && key === 'z' && !shift) {
        event.preventDefault()
        executeCommand('edit.undo')
        return
      }

      // Повтор: Ctrl+Y
      if (ctrl && key === 'y' && !shift) {
        event.preventDefault()
        executeCommand('edit.redo')
        return
      }

      // Переворот детали: F
      if (key === 'f' && !ctrl && !shift && !alt) {
        event.preventDefault()
        executeCommand('edit.flip')
        return
      }

      // Поворот детали: R
      if (key === 'r' && !ctrl && !shift && !alt) {
        event.preventDefault()
        executeCommand('edit.rotate')
        return
      }

      // Каркасный режим: W
      if (key === 'w' && !ctrl && !shift && !alt) {
        event.preventDefault()
        executeCommand('view.wireframe')
        return
      }

      // Текстуры: T
      if (key === 't' && !ctrl && !shift && !alt) {
        event.preventDefault()
        executeCommand('view.textures')
        return
      }

      // Оптимизация: O
      if (key === 'o' && !ctrl && !shift && !alt) {
        event.preventDefault()
        executeCommand('tools.optimize')
        return
      }

      // Документация: F1
      if (key === 'f1') {
        event.preventDefault()
        executeCommand('help.docs')
        return
      }

      // Горячие клавиши: Ctrl+Shift+K
      if (ctrl && shift && key === 'k') {
        event.preventDefault()
        executeCommand('settings.shortcuts')
        return
      }
    }
  }

  // Проверка, открыта ли палитра команд
  private isCommandPaletteOpen(): boolean {
    // В реальном приложении нужно проверять состояние через store или props
    // Здесь используем проверку по классу или атрибуту
    return document.querySelector('.command-palette-overlay') !== null
  }

  // Триггер для открытия палитры команд
  private triggerCommandPalette() {
    // Создаем событие для открытия палитры
    const event = new CustomEvent('open-command-palette')
    window.dispatchEvent(event)
  }

  // Регистрация пользовательской горячей клавиши
  registerHotkey(key: string, callback: () => void, options?: {
    ctrl?: boolean
    shift?: boolean
    alt?: boolean
    preventDefault?: boolean
  }) {
    const handler = (event: KeyboardEvent) => {
      if (!this.isEnabled) return
      
      const matches = 
        event.key.toLowerCase() === key.toLowerCase() &&
        (options?.ctrl === undefined || event.ctrlKey === options.ctrl || event.metaKey === options.ctrl) &&
        (options?.shift === undefined || event.shiftKey === options.shift) &&
        (options?.alt === undefined || event.altKey === options.alt)

      if (matches) {
        if (options?.preventDefault !== false) {
          event.preventDefault()
        }
        callback()
      }
    }

    window.addEventListener('keydown', handler)
    const id = `custom-${Date.now()}-${Math.random()}`
    this.registeredHandlers.set(id, handler)
    
    return () => {
      window.removeEventListener('keydown', handler)
      this.registeredHandlers.delete(id)
    }
  }

  // Очистка всех обработчиков
  cleanup() {
    this.registeredHandlers.forEach((handler, id) => {
      window.removeEventListener('keydown', handler)
    })
    this.registeredHandlers.clear()
  }
}

// Создаем глобальный экземпляр сервиса
export const hotkeysService = new HotkeysService()