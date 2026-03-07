<template>
  <div class="quick-actions-panel">
    <h2 class="panel-title">Быстрые действия</h2>
    
    <div class="actions-grid">
      <button 
        v-for="(action, index) in actions" 
        :key="index"
        class="action-button"
        :class="{ 'is-primary': action.isPrimary }"
        @click="handleAction(action.id)"
        :disabled="action.disabled"
      >
        <div class="action-icon">
          <component :is="getIconComponent(action.icon)" />
        </div>
        <span class="action-label">{{ action.label }}</span>
        <span v-if="action.shortcut" class="action-shortcut">{{ action.shortcut }}</span>
      </button>
    </div>
    
    <div class="divider" />
    
    <div class="recent-files">
      <h3 class="section-title">Последние файлы</h3>
      <div v-if="loading" class="loading-state">
        <AppLoading size="small" />
      </div>
      <div v-else-if="recentFiles.length === 0" class="no-files">
        <span>Нет недавних файлов</span>
      </div>
      <div v-else class="files-list">
        <div 
          v-for="file in recentFiles" 
          :key="file.id" 
          class="file-item"
          @click="openFile(file)"
          @contextmenu.prevent="showFileContextMenu($event, file)"
        >
          <i-mdi:file class="file-icon" />
          <span class="file-name">{{ truncateFileName(file.name) }}</span>
          <span class="file-date">{{ formatDate(file.lastModified) }}</span>
        </div>
      </div>
    </div>

    <ContextMenu 
      :options="fileContextMenuOptions" 
      :position="contextMenuPosition" 
      :visible="showContextMenuFlag" 
      @close="closeContextMenu"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useProjectStore } from '@/stores/project.store'
import Button from '@/components/ui/Button.vue'
import AppLoading from '@/components/ui/AppLoading.vue'
import ContextMenu from '@/components/ui/ContextMenu.vue'

const projectStore = useProjectStore()
const contextMenuPosition = ref({ x: 0, y: 0 })
const showContextMenuFlag = ref(false)
const selectedFile = ref<any>(null)
const recentFiles = ref<any[]>([])

const actions = [
  {
    id: 'new-project',
    label: 'Новый проект',
    icon: 'mdi:plus',
    shortcut: 'Ctrl+N',
    isPrimary: true,
    disabled: false
  },
  {
    id: 'open-project',
    label: 'Открыть проект',
    icon: 'mdi:folder-open',
    shortcut: 'Ctrl+O',
    isPrimary: false,
    disabled: false
  },
  {
    id: 'import-model',
    label: 'Импорт 3D модели',
    icon: 'mdi:upload',
    shortcut: 'Ctrl+I',
    isPrimary: false,
    disabled: false
  },
  {
    id: 'ai-assistant',
    label: 'AI помощник',
    icon: 'mdi:robot',
    shortcut: 'Ctrl+K',
    isPrimary: false,
    disabled: false
  }
]

const fileContextMenuOptions = [
  { label: 'Открыть', icon: 'mdi:open-in-app', action: 'open' },
  { label: 'Копировать путь', icon: 'mdi:content-copy', action: 'copy-path' },
  { label: 'Удалить из истории', icon: 'mdi:history', action: 'remove-from-history' }
]

const loading = computed(() => projectStore.loading)

const getIconComponent = (iconName: string) => {
  return iconName.split(':')[1] || 'mdi:help'
}

const handleAction = (actionId: string) => {
  switch (actionId) {
    case 'new-project':
      createNewProject()
      break
    case 'open-project':
      openProjectDialog()
      break
    case 'import-model':
      importModel()
      break
    case 'ai-assistant':
      openAIAssistant()
      break
  }
}

const createNewProject = async () => {
  const projectName = prompt('Введите название проекта:', 'Мой проект')
  if (projectName) {
    const newProject = await projectStore.createProject(projectName)
    if (newProject) {
      console.log('Создан новый проект:', newProject.name)
      // Здесь будет переход в редактор
    }
  }
}

const openProjectDialog = async () => {
  try {
    // В реальном приложении здесь будет вызов Tauri диалога
    const filePath = 'D:/models/example.obj' // mock путь
    const project = await projectStore.importProject(filePath)
    if (project) {
      console.log('Импортирован проект:', project.name)
      // Здесь будет переход в редактор
    }
  } catch (error) {
    console.error('Error opening project:', error)
  }
}

const importModel = async () => {
  try {
    // В реальном приложении здесь будет вызов Tauri диалога для выбора файла
    alert('Выберите 3D модель для импорта')
    const filePath = '/path/to/model.obj' // mock путь
    const project = await projectStore.importProject(filePath)
    if (project) {
      console.log('Импортирована модель:', project.name)
    }
  } catch (error) {
    console.error('Error importing model:', error)
  }
}

const openAIAssistant = () => {
  console.log('Открытие AI помощника')
  // Здесь будет логика открытия AI помощника
}

const openFile = (file: any) => {
  console.log('Открытие файла:', file.name)
  // Здесь будет логика открытия файла
}

const truncateFileName = (name: string) => {
  return name.length > 25 ? name.substring(0, 22) + '...' : name
}

const formatDate = (dateString: string | Date) => {
  const date = new Date(dateString)
  return date.toLocaleDateString('ru-RU', {
    day: '2-digit',
    month: 'short'
  })
}

const showFileContextMenu = (event: MouseEvent, file: any) => {
  event.preventDefault()
  selectedFile.value = file
  contextMenuPosition.value = { x: event.clientX, y: event.clientY }
  showContextMenuFlag.value = true
  document.addEventListener('click', closeContextMenuOnClickOutside)
}

const closeContextMenu = () => {
  showContextMenuFlag.value = false
  selectedFile.value = null
  document.removeEventListener('click', closeContextMenuOnClickOutside)
}

const closeContextMenuOnClickOutside = (event: MouseEvent) => {
  const target = event.target as HTMLElement
  if (!target.closest('.context-menu') && !target.closest('.file-item')) {
    closeContextMenu()
  }
}

const handleFileContextMenuAction = (action: string) => {
  if (!selectedFile.value) return
  
  switch (action) {
    case 'open':
      openFile(selectedFile.value)
      break
    case 'copy-path':
      navigator.clipboard.writeText(selectedFile.value.path || '')
      break
    case 'remove-from-history':
      // Здесь будет логика удаления из истории
      console.log('Удаление из истории:', selectedFile.value.name)
      break
  }
  closeContextMenu()
}

const loadRecentFiles = () => {
  // В реальном приложении здесь будет загрузка из истории
  recentFiles.value = [
    { id: 'file1', name: 'knight_model.obj', lastModified: new Date('2026-01-15T14:30:00'), path: 'D:/models/knight.obj' },
    { id: 'file2', name: 'dragon_final.obj', lastModified: new Date('2026-01-14T09:15:00'), path: 'D:/models/dragon.obj' },
    { id: 'file3', name: 'spaceship_v2.obj', lastModified: new Date('2026-01-12T16:45:00'), path: 'D:/models/spaceship.obj' }
  ]
}

onMounted(() => {
  loadRecentFiles()
})
</script>

<style scoped>
.quick-actions-panel {
  background: var(--surface-800);
  border-radius: 16px;
  padding: 24px;
  border: 1px solid var(--border-color);
}

.panel-title {
  font-size: 20px;
  margin-bottom: 24px;
  color: var(--text-color);
  font-weight: 600;
}

.actions-grid {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 16px;
  margin-bottom: 24px;
}

.action-button {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 24px 16px;
  border-radius: 12px;
  border: 2px solid var(--border-color);
  background: var(--surface-700);
  cursor: pointer;
  transition: all 0.2s;
  text-align: center;
  min-height: 120px;
}

.action-button.is-primary {
  background: var(--primary-500);
  border-color: var(--primary-500);
}

.action-button:hover:not(:disabled) {
  transform: translateY(-2px);
  border-color: var(--primary-500);
}

.action-button:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.action-icon {
  font-size: 32px;
  margin-bottom: 12px;
  color: var(--primary-400);
}

.action-button.is-primary .action-icon,
.action-button.is-primary:hover .action-icon {
  color: white;
}

.action-label {
  font-weight: 500;
  font-size: 16px;
  margin-bottom: 4px;
  color: var(--text-color);
}

.action-button.is-primary .action-label {
  color: white;
}

.action-shortcut {
  background: var(--surface-900);
  color: var(--text-secondary);
  padding: 2px 8px;
  border-radius: 6px;
  font-size: 12px;
  margin-top: 4px;
}

.divider {
  height: 1px;
  background: var(--border-color);
  margin: 24px 0;
}

.section-title {
  font-size: 16px;
  margin-bottom: 16px;
  color: var(--text-color);
  font-weight: 500;
}

.loading-state, .no-files {
  text-align: center;
  padding: 16px;
  color: var(--text-secondary);
}

.files-list {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.file-item {
  display: flex;
  align-items: center;
  padding: 8px 12px;
  border-radius: 8px;
  cursor: pointer;
  transition: background 0.2s;
}

.file-item:hover {
  background: var(--surface-700);
}

.file-icon {
  font-size: 20px;
  color: var(--primary-400);
  margin-right: 12px;
}

.file-name {
  flex: 1;
  color: var(--text-color);
  font-size: 14px;
}

.file-date {
  color: var(--text-secondary);
  font-size: 12px;
  white-space: nowrap;
}
</style>
