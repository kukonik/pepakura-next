<template>
  <div class="recent-projects-panel">
    <div class="panel-header">
      <h2 class="panel-title">Недавние проекты</h2>
      <div class="panel-actions">
        <button class="view-toggle" :class="{ active: viewMode === 'grid' }" @click="viewMode = 'grid'">
          <i-mdi:view-grid-outline />
        </button>
        <button class="view-toggle" :class="{ active: viewMode === 'list' }" @click="viewMode = 'list'">
          <i-mdi:view-list-outline />
        </button>
      </div>
    </div>

    <div v-if="loading" class="loading-container">
      <AppLoading size="medium" />
    </div>

    <div v-else-if="error" class="error-container">
      <div class="error-message">
        <i-mdi:alert-circle-outline class="error-icon" />
        <span>{{ error }}</span>
        <Button variant="secondary" @click="loadProjects" class="retry-button">
          Повторить
        </Button>
      </div>
    </div>

    <div v-else-if="recentProjects.length === 0" class="empty-container">
      <div class="empty-state">
        <i-mdi:file-document-multiple-outline class="empty-icon" />
        <p>Нет недавних проектов</p>
        <Button @click="createNewProject" variant="primary">
          Создать проект
        </Button>
      </div>
    </div>

    <div v-else class="projects-container" :class="viewMode">
      <div 
        v-for="project in recentProjects" 
        :key="project.id" 
        class="project-card"
        @click="selectProject(project)"
        @contextmenu.prevent="showContextMenu($event, project)"
      >
        <div class="project-thumbnail">
          <img :src="project.thumbnail || '/placeholder.jpg'" alt="Превью проекта" />
          <div class="project-overlay">
            <i-mdi:eye-outline class="view-icon" />
          </div>
        </div>
        <div class="project-info">
          <h3 class="project-name">{{ project.name }}</h3>
          <div class="project-meta">
            <span class="project-date">{{ formatDate(project.lastModified) }}</span>
            <span class="project-size">{{ project.fileSize.toFixed(1) }} МБ</span>
            <span class="project-sheets">{{ project.sheetCount }} лист{{ getSheetCountSuffix(project.sheetCount) }}</span>
          </div>
          <div class="project-tags">
            <span v-for="(tag, index) in project.tags" :key="index" class="tag">{{ tag }}</span>
          </div>
        </div>
      </div>
    </div>

    <ContextMenu 
      :options="contextMenuOptions" 
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
const viewMode = ref<'grid' | 'list'>('grid')
const contextMenuPosition = ref({ x: 0, y: 0 })
const showContextMenuFlag = ref(false)
const selectedProject = ref<any>(null)

const loading = computed(() => projectStore.loading)
const error = computed(() => projectStore.error)
const recentProjects = computed(() => projectStore.recentProjects)

const contextMenuOptions = computed(() => [
  { label: 'Открыть', icon: 'mdi:open-in-app', action: 'open' },
  { label: 'Дублировать', icon: 'mdi:content-copy', action: 'duplicate' },
  { label: 'Экспорт', icon: 'mdi:export', action: 'export' },
  { label: 'Удалить', icon: 'mdi:delete', action: 'delete', danger: true }
])

const loadProjects = async () => {
  await projectStore.loadProjects()
}

const selectProject = (project: any) => {
  if (!showContextMenuFlag.value) {
    console.log('Открытие проекта:', project.name)
    // Здесь будет навигация к редактору
  }
}

const createNewProject = () => {
  console.log('Создание нового проекта')
  // Здесь будет модальное окно создания проекта
}

const formatDate = (date: Date | string): string => {
  const d = date instanceof Date ? date : new Date(date)
  return d.toLocaleDateString('ru-RU', { 
    day: '2-digit', 
    month: 'short', 
    year: 'numeric',
    hour: '2-digit',
    minute: '2-digit'
  })
}

const getSheetCountSuffix = (count: number): string => {
  if (count % 10 === 1 && count % 100 !== 11) return ''
  if ([2, 3, 4].includes(count % 10) && ![12, 13, 14].includes(count % 100)) return 'а'
  return 'ов'
}

const showContextMenu = (event: MouseEvent, project: any) => {
  event.preventDefault()
  selectedProject.value = project
  contextMenuPosition.value = { x: event.clientX, y: event.clientY }
  showContextMenuFlag.value = true
  document.addEventListener('click', closeContextMenuOnClickOutside)
}

const closeContextMenu = () => {
  showContextMenuFlag.value = false
  selectedProject.value = null
  document.removeEventListener('click', closeContextMenuOnClickOutside)
}

const closeContextMenuOnClickOutside = (event: MouseEvent) => {
  const target = event.target as HTMLElement
  if (!target.closest('.context-menu') && !target.closest('.project-card')) {
    closeContextMenu()
  }
}

const handleContextMenuAction = (action: string) => {
  if (!selectedProject.value) return
  
  switch (action) {
    case 'open':
      console.log('Открытие проекта:', selectedProject.value.name)
      break
    case 'duplicate':
      console.log('Дублирование проекта:', selectedProject.value.name)
      break
    case 'export':
      console.log('Экспорт проекта:', selectedProject.value.name)
      break
    case 'delete':
      if (confirm(`Удалить проект "${selectedProject.value.name}"?`)) {
        projectStore.deleteProject(selectedProject.value.id)
      }
      break
  }
  closeContextMenu()
}

onMounted(() => {
  if (recentProjects.value.length === 0) {
    loadProjects()
  }
})
</script>

<style scoped>
.recent-projects-panel {
  background: var(--surface-800);
  border-radius: 16px;
  border: 1px solid var(--border-color);
  padding: 24px;
  height: 100%;
  display: flex;
  flex-direction: column;
}

.panel-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 24px;
}

.panel-title {
  font-size: 20px;
  margin: 0;
  color: var(--text-color);
  font-weight: 600;
}

.panel-actions {
  display: flex;
  gap: 8px;
}

.view-toggle {
  width: 36px;
  height: 36px;
  border-radius: 8px;
  background: var(--surface-700);
  border: 1px solid var(--border-color);
  color: var(--text-secondary);
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  transition: all 0.2s ease;
}

.view-toggle:hover {
  background: var(--surface-600);
  color: var(--primary-400);
}

.view-toggle.active {
  background: var(--primary-500);
  border-color: var(--primary-500);
  color: white;
}

.loading-container, .error-container, .empty-container {
  display: flex;
  justify-content: center;
  align-items: center;
  flex-grow: 1;
  padding: 40px;
}

.error-message {
  display: flex;
  flex-direction: column;
  align-items: center;
  text-align: center;
  gap: 16px;
}

.error-icon {
  font-size: 48px;
  color: var(--error-400);
}

.retry-button {
  margin-top: 16px;
}

.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  text-align: center;
  gap: 24px;
}

.empty-icon {
  font-size: 64px;
  color: var(--text-secondary);
}

.projects-container {
  display: grid;
  gap: 20px;
}

.projects-container.grid {
  grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
}

.projects-container.list {
  grid-template-columns: 1fr;
}

.project-card {
  background: var(--surface-700);
  border-radius: 12px;
  overflow: hidden;
  cursor: pointer;
  transition: all 0.2s ease;
  border: 1px solid var(--border-color);
  display: flex;
  flex-direction: column;
}

.project-card:hover {
  transform: translateY(-2px);
  border-color: var(--primary-500);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
}

.project-card:active {
  transform: translateY(0);
}

.project-thumbnail {
  position: relative;
  width: 100%;
  height: 160px;
  overflow: hidden;
}

.project-thumbnail img {
  width: 100%;
  height: 100%;
  object-fit: cover;
  transition: transform 0.3s ease;
}

.project-card:hover .project-thumbnail img {
  transform: scale(1.05);
}

.project-overlay {
  position: absolute;
  inset: 0;
  background: rgba(0, 0, 0, 0.6);
  display: flex;
  justify-content: center;
  align-items: center;
  opacity: 0;
  transition: opacity 0.2s ease;
}

.project-card:hover .project-overlay {
  opacity: 1;
}

.view-icon {
  font-size: 32px;
  color: white;
}

.project-info {
  padding: 16px;
  flex-grow: 1;
}

.project-name {
  font-size: 18px;
  margin: 0 0 8px 0;
  color: var(--text-color);
  font-weight: 600;
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}

.project-meta {
  display: flex;
  flex-wrap: wrap;
  gap: 12px;
  margin-bottom: 12px;
  color: var(--text-secondary);
  font-size: 13px;
}

.project-date, .project-size, .project-sheets {
  display: flex;
  align-items: center;
  gap: 4px;
}

.project-tags {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
}

.tag {
  background: var(--surface-900);
  color: var(--primary-300);
  padding: 2px 8px;
  border-radius: 12px;
  font-size: 12px;
  font-weight: 500;
}
</style>
