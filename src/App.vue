<template>
  <div id="app">
    <RestoreSessionModal
      v-if="showRestoreModal"
      :show="showRestoreModal"
      :autosave="pendingAutosave"
      @restore="onRestore"
      @discard="onDiscard"
      @close="closeRestoreModal"
    />
    
    <CommandPalette
      :is-open="commandPaletteOpen"
      @close="closeCommandPalette"
      @command-executed="onCommandExecuted"
    />
    
    <header>
      <h1>Pepakura Next - AI 3D Model Generator</h1>
      <div class="header-controls">
        <button class="command-palette-button" @click="openCommandPalette" title="Открыть палитру команд (Ctrl+K)">
          <span class="button-icon">⌘</span>
          <span class="button-text">Команды</span>
          <span class="button-hotkey">Ctrl+K</span>
        </button>
        <button class="ai-assistant-button" @click="toggleAiPanel" :title="showAiPanel ? 'Скрыть AI ассистента' : 'Показать AI ассистента'">
          <span class="button-icon">🤖</span>
          <span class="button-text">AI Анализ</span>
          <span class="button-hotkey">Ctrl+I</span>
        </button>
        <div class="autosave-status" v-if="autosaveStatus">
          {{ autosaveStatus }}
        </div>
      </div>
    </header>
    <main>
      <div class="main-layout">
        <div class="content-section">
          <AIGenerator />
          <div class="model-viewer-section">
            <ModelViewer3D ref="modelViewer" />
            <UnfoldButton />
          </div>
          <div class="sheet-preview-section">
            <SheetPreview />
          </div>
        </div>
        <div class="sidebar-section" v-if="showAiPanel">
          <AiAssistantPanel />
        </div>
      </div>
    </main>
  </div>
</template>

<script setup>
import AIGenerator from '@/components/AIGenerator.vue'
import ModelViewer3D from '@/components/ModelViewer3D.vue'
import SheetPreview from '@/components/SheetPreview.vue'
import UnfoldButton from '@/components/UnfoldButton.vue'
import RestoreSessionModal from '@/components/RestoreSessionModal.vue'
import CommandPalette from '@/components/CommandPalette.vue'
import AiAssistantPanel from '@/components/AiAssistantPanel.vue'
import { ref, onMounted, onUnmounted } from 'vue'
import { autosaveService } from './services/autosave.service'
import { useCommandPalette } from './composables/useCommandPalette'

const modelViewer = ref(null)
const showRestoreModal = ref(false)
const pendingAutosave = ref(null)
const autosaveStatus = ref('')
const showAiPanel = ref(false)

// Command Palette
const { isOpen: commandPaletteOpen, open: openCommandPalette, close: closeCommandPalette } = useCommandPalette()

// Toggle AI Panel
function toggleAiPanel() {
  showAiPanel.value = !showAiPanel.value
}

// Handle keyboard shortcuts
function handleKeydown(event) {
  if ((event.ctrlKey || event.metaKey) && event.key === 'i') {
    event.preventDefault()
    toggleAiPanel()
  }
  if ((event.ctrlKey || event.metaKey) && event.key === 'k') {
    event.preventDefault()
    openCommandPalette()
  }
}

onMounted(async () => {
  // Инициализируем автосохранение
  await autosaveService.initialize()
  
  // Проверяем наличие несохранённой сессии
  const hasUnfinished = await autosaveService.hasUnfinishedSession()
  if (hasUnfinished) {
    const latest = await autosaveService.getLatestAutosave()
    if (latest) {
      pendingAutosave.value = latest
      showRestoreModal.value = true
    }
  }

  // Обновляем статус автосохранения
  updateAutosaveStatus()

  // Регистрируем обработчик горячих клавиш
  window.addEventListener('keydown', handleKeydown)
})

onUnmounted(() => {
  // Останавливаем автосохранение при размонтировании
  autosaveService.stopAutoSave()
  // Удаляем обработчик горячих клавиш
  window.removeEventListener('keydown', handleKeydown)
})

const updateAutosaveStatus = async () => {
  const autosaves = await autosaveService.getAllAutosaves()
  if (autosaves.length > 0) {
    const last = autosaves[0]
    const time = new Date(last.timestamp).toLocaleTimeString('ru-RU', { hour: '2-digit', minute: '2-digit' })
    autosaveStatus.value = `Автосохранение: ${time}`
  } else {
    autosaveStatus.value = ''
  }
}

const onRestore = () => {
  showRestoreModal.value = false
  autosaveStatus.value = 'Сессия восстановлена'
  // Можно показать уведомление
  setTimeout(() => {
    autosaveStatus.value = ''
  }, 3000)
}

const onDiscard = () => {
  showRestoreModal.value = false
  autosaveStatus.value = 'Сессия отклонена'
  setTimeout(() => {
    autosaveStatus.value = ''
  }, 3000)
}

const closeRestoreModal = () => {
  showRestoreModal.value = false
}

// Обработка выполнения команды из палитры
const onCommandExecuted = (command) => {
  console.log(`Команда выполнена: ${command.title}`)
  // Можно добавить уведомление или логирование
}
</script>

<style>
#app {
  font-family: Arial, sans-serif;
  text-align: center;
  color: #2c3e50;
  margin-top: 60px;
}

header {
  background-color: #2c3e50;
  color: white;
  padding: 20px;
  margin-bottom: 30px;
  position: relative;
}

.header-controls {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-top: 10px;
}

.command-palette-button {
  display: flex;
  align-items: center;
  gap: 8px;
  background: rgba(255, 255, 255, 0.15);
  color: white;
  border: 1px solid rgba(255, 255, 255, 0.3);
  border-radius: 6px;
  padding: 8px 16px;
  font-size: 14px;
  cursor: pointer;
  transition: all 0.2s;
}

.command-palette-button:hover {
  background: rgba(255, 255, 255, 0.25);
  transform: translateY(-1px);
}

.command-palette-button:active {
  transform: translateY(0);
}

.button-icon {
  font-size: 16px;
  font-weight: bold;
}

.button-text {
  font-weight: 500;
}

.button-hotkey {
  font-size: 12px;
  opacity: 0.8;
  background: rgba(255, 255, 255, 0.1);
  padding: 2px 6px;
  border-radius: 4px;
}

.autosave-status {
  font-size: 12px;
  background: rgba(255, 255, 255, 0.2);
  padding: 4px 8px;
  border-radius: 4px;
}

.model-viewer-section {
  display: flex;
  flex-direction: column;
  align-items: center;
  margin: 20px 0;
}

.sheet-preview-section {
  margin: 20px 0;
  height: 500px;
}

/* AI Assistant Button */
.ai-assistant-button {
  display: flex;
  align-items: center;
  gap: 8px;
  background: rgba(106, 17, 203, 0.7);
  color: white;
  border: 1px solid rgba(255, 255, 255, 0.3);
  border-radius: 6px;
  padding: 8px 16px;
  font-size: 14px;
  cursor: pointer;
  transition: all 0.2s;
  margin-left: 10px;
}

.ai-assistant-button:hover {
  background: rgba(106, 17, 203, 0.9);
  transform: translateY(-1px);
}

.ai-assistant-button:active {
  transform: translateY(0);
}

/* Main layout with sidebar */
.main-layout {
  display: flex;
  gap: 20px;
  max-width: 1400px;
  margin: 0 auto;
  padding: 0 20px;
}

.content-section {
  flex: 3;
  min-width: 0;
}

.sidebar-section {
  flex: 1;
  min-width: 300px;
  max-width: 400px;
}
</style>
