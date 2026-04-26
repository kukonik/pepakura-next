/**
 * Композиция для управления Command Palette
 */

import { ref, onMounted, onUnmounted } from 'vue'
import { hotkeysService } from '@/services/hotkeys.service'

export function useCommandPalette() {
  const isOpen = ref(false)

  // Открытие палитры команд
  const open = () => {
    isOpen.value = true
    console.log('Command Palette открыта')
    
    // Временно отключаем глобальные горячие клавиши при открытой палитре
    hotkeysService.disable()
  }

  // Закрытие палитры команд
  const close = () => {
    isOpen.value = false
    console.log('Command Palette закрыта')
    
    // Включаем горячие клавиши обратно
    hotkeysService.enable()
  }

  // Переключение состояния
  const toggle = () => {
    if (isOpen.value) {
      close()
    } else {
      open()
    }
  }

  // Обработка глобальных горячих клавиш
  const handleGlobalKeydown = (event: KeyboardEvent) => {
    // Ctrl+K или Cmd+K для открытия палитры
    if ((event.ctrlKey || event.metaKey) && event.key === 'k') {
      event.preventDefault()
      open()
    }

    // Escape для закрытия палитры
    if (event.key === 'Escape' && isOpen.value) {
      event.preventDefault()
      close()
    }
  }

  // Обработка события открытия палитры
  const handleOpenCommandPalette = () => {
    open()
  }

  // Регистрация глобальных обработчиков
  onMounted(() => {
    window.addEventListener('keydown', handleGlobalKeydown)
    window.addEventListener('open-command-palette', handleOpenCommandPalette)
  })

  onUnmounted(() => {
    window.removeEventListener('keydown', handleGlobalKeydown)
    window.removeEventListener('open-command-palette', handleOpenCommandPalette)
  })

  return {
    isOpen,
    open,
    close,
    toggle
  }
}