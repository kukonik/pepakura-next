<template>
  <div v-if="isOpen" class="command-palette-overlay" @click.self="close">
    <div class="command-palette">
      <div class="command-palette-header">
        <input
          ref="inputRef"
          v-model="searchQuery"
          class="command-palette-input"
          placeholder="Введите команду или начните печатать..."
          @keydown="handleKeydown"
          @input="handleSearch"
        />
        <div class="command-palette-hint">
          <kbd>↑</kbd><kbd>↓</kbd> для навигации, <kbd>Enter</kbd> для выбора, <kbd>Esc</kbd> для отмены
        </div>
      </div>

      <div v-if="filteredCommands.length > 0" class="command-palette-list">
        <div
          v-for="(command, index) in filteredCommands"
          :key="command.id"
          :class="['command-item', { 'selected': selectedIndex === index }]"
          @click="executeCommand(command)"
          @mouseenter="selectedIndex = index"
        >
          <div class="command-icon">{{ command.icon || '⚡' }}</div>
          <div class="command-content">
            <div class="command-title">{{ command.title }}</div>
            <div class="command-description">{{ command.description }}</div>
            <div class="command-meta">
              <span class="command-category">{{ getCategoryLabel(command.category) }}</span>
              <span v-if="command.hotkey" class="command-hotkey">{{ formatHotkey(command.hotkey) }}</span>
            </div>
          </div>
        </div>
      </div>

      <div v-else class="command-palette-empty">
        <div class="empty-icon">🔍</div>
        <div class="empty-text">Команды не найдены</div>
        <div class="empty-hint">Попробуйте другой запрос</div>
      </div>

      <div class="command-palette-footer">
        <div class="footer-stats">
          {{ filteredCommands.length }} из {{ totalCommands }} команд
        </div>
        <div class="footer-categories">
          <span v-for="category in categories" :key="category" class="category-tag">
            {{ getCategoryLabel(category) }}
          </span>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, onMounted, onUnmounted, watch, nextTick } from 'vue'
import { 
  commandRegistry, 
  searchCommands, 
  getAllCategories,
  executeCommand as executeCommandById 
} from '@/commands/commands.registry'

const props = defineProps({
  isOpen: {
    type: Boolean,
    required: true
  }
})

const emit = defineEmits(['close', 'command-executed'])

const searchQuery = ref('')
const selectedIndex = ref(0)
const inputRef = ref(null)
const allCommands = ref([])
const categories = ref([])

// Загружаем команды и категории
onMounted(() => {
  allCommands.value = commandRegistry.getAllCommands()
  categories.value = getAllCategories()
})

// Фильтруем команды по запросу
const filteredCommands = computed(() => {
  if (!searchQuery.value.trim()) {
    return allCommands.value
  }
  return searchCommands(searchQuery.value)
})

const totalCommands = computed(() => allCommands.value.length)

// Фокус на input при открытии
watch(() => props.isOpen, (isOpen) => {
  if (isOpen) {
    nextTick(() => {
      inputRef.value?.focus()
      searchQuery.value = ''
      selectedIndex.value = 0
    })
  }
})

// Обработка клавиш
const handleKeydown = (event) => {
  switch (event.key) {
    case 'Escape':
      event.preventDefault()
      close()
      break
    case 'ArrowDown':
      event.preventDefault()
      selectedIndex.value = Math.min(selectedIndex.value + 1, filteredCommands.value.length - 1)
      break
    case 'ArrowUp':
      event.preventDefault()
      selectedIndex.value = Math.max(selectedIndex.value - 1, 0)
      break
    case 'Enter':
      event.preventDefault()
      if (filteredCommands.value[selectedIndex.value]) {
        executeCommand(filteredCommands.value[selectedIndex.value])
      }
      break
    case 'Tab':
      event.preventDefault()
      // Циклическое переключение между командами
      if (event.shiftKey) {
        selectedIndex.value = selectedIndex.value > 0 ? selectedIndex.value - 1 : filteredCommands.value.length - 1
      } else {
        selectedIndex.value = selectedIndex.value < filteredCommands.value.length - 1 ? selectedIndex.value + 1 : 0
      }
      break
  }
}

// Выполнение команды
const executeCommand = (command) => {
  console.log(`Выполнение команды: ${command.title}`)
  command.action()
  emit('command-executed', command)
  close()
}

// Закрытие палитры
const close = () => {
  emit('close')
}

// Обработка поиска
const handleSearch = () => {
  selectedIndex.value = 0
}

// Глобальные горячие клавиши
const handleGlobalKeydown = (event) => {
  // Ctrl+K или Cmd+K для открытия палитры
  if ((event.ctrlKey || event.metaKey) && event.key === 'k') {
    event.preventDefault()
    if (!props.isOpen) {
      // Открываем палитру через emit
      // В реальном приложении это будет управляться родительским компонентом
      console.log('Ctrl+K pressed - открытие палитры команд')
    }
  }
}

// Регистрируем глобальные обработчики
onMounted(() => {
  window.addEventListener('keydown', handleGlobalKeydown)
})

onUnmounted(() => {
  window.removeEventListener('keydown', handleGlobalKeydown)
})

// Вспомогательные функции
const getCategoryLabel = (category) => {
  const labels = {
    'file': 'Файл',
    'edit': 'Редактирование',
    'view': 'Вид',
    'tools': 'Инструменты',
    'export': 'Экспорт',
    'settings': 'Настройки',
    'help': 'Помощь'
  }
  return labels[category] || category
}

const formatHotkey = (hotkey) => {
  return hotkey
    .replace('Ctrl', '⌃')
    .replace('Cmd', '⌘')
    .replace('Shift', '⇧')
    .replace('Alt', '⌥')
    .replace('+', ' ')
}
</script>

<style scoped>
.command-palette-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: flex-start;
  justify-content: center;
  padding-top: 100px;
  z-index: 9999;
  backdrop-filter: blur(2px);
}

.command-palette {
  width: 600px;
  max-width: 90vw;
  background: white;
  border-radius: 8px;
  box-shadow: 0 10px 30px rgba(0, 0, 0, 0.3);
  overflow: hidden;
  animation: slideDown 0.2s ease-out;
}

@keyframes slideDown {
  from {
    opacity: 0;
    transform: translateY(-20px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.command-palette-header {
  padding: 20px;
  border-bottom: 1px solid #e5e7eb;
  background: #f9fafb;
}

.command-palette-input {
  width: 100%;
  padding: 12px 16px;
  font-size: 16px;
  border: 2px solid #e5e7eb;
  border-radius: 6px;
  outline: none;
  transition: border-color 0.2s;
}

.command-palette-input:focus {
  border-color: #3b82f6;
  box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
}

.command-palette-hint {
  margin-top: 8px;
  font-size: 12px;
  color: #6b7280;
  display: flex;
  align-items: center;
  gap: 8px;
}

.command-palette-hint kbd {
  background: #e5e7eb;
  padding: 2px 6px;
  border-radius: 4px;
  font-size: 11px;
  font-family: monospace;
}

.command-palette-list {
  max-height: 400px;
  overflow-y: auto;
  padding: 8px 0;
}

.command-item {
  display: flex;
  align-items: center;
  padding: 12px 20px;
  cursor: pointer;
  transition: background-color 0.15s;
  border-left: 3px solid transparent;
}

.command-item:hover {
  background-color: #f3f4f6;
}

.command-item.selected {
  background-color: #eff6ff;
  border-left-color: #3b82f6;
}

.command-icon {
  font-size: 20px;
  margin-right: 16px;
  width: 32px;
  text-align: center;
}

.command-content {
  flex: 1;
}

.command-title {
  font-weight: 600;
  color: #111827;
  margin-bottom: 4px;
}

.command-description {
  font-size: 14px;
  color: #6b7280;
  margin-bottom: 4px;
}

.command-meta {
  display: flex;
  gap: 12px;
  font-size: 12px;
}

.command-category {
  background: #e5e7eb;
  padding: 2px 8px;
  border-radius: 12px;
  color: #4b5563;
}

.command-hotkey {
  color: #6b7280;
  font-family: monospace;
}

.command-palette-empty {
  padding: 40px 20px;
  text-align: center;
  color: #6b7280;
}

.empty-icon {
  font-size: 48px;
  margin-bottom: 16px;
  opacity: 0.5;
}

.empty-text {
  font-size: 18px;
  font-weight: 600;
  margin-bottom: 8px;
}

.empty-hint {
  font-size: 14px;
}

.command-palette-footer {
  padding: 12px 20px;
  border-top: 1px solid #e5e7eb;
  background: #f9fafb;
  display: flex;
  justify-content: space-between;
  align-items: center;
  font-size: 12px;
  color: #6b7280;
}

.footer-categories {
  display: flex;
  gap: 6px;
  flex-wrap: wrap;
}

.category-tag {
  background: #e5e7eb;
  padding: 2px 8px;
  border-radius: 12px;
  color: #4b5563;
}
</style>