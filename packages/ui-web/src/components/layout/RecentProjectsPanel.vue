<template>
  <div class="recent-projects-panel">
    <div class="panel-header">
      <h2>Недавние проекты</h2>
      <div class="header-actions">
        <select v-model="sortBy" class="sort-select">
          <option value="date">По дате</option>
          <option value="name">По имени</option>
          <option value="size">По размеру</option>
        </select>
      </div>
    </div>

    <div v-if="loading" class="loading-state">
      <div class="spinner"></div>
      <p>Загрузка проектов...</p>
    </div>

    <div v-else-if="recentProjects.length === 0" class="empty-state">
      <div class="empty-icon">&#128193;</div>
      <p>Нет недавних проектов</p>
      <Button @click="emit('create-project')" variant="primary">
        Создать первый проект
      </Button>
    </div>

    <div v-else class="projects-grid">
      <div
        v-for="project in sortedProjects"
        :key="project.id"
        class="project-card"
        :class="{ selected: isSelected(project.id) }"
        @click="selectProjectSimple(project.id)"
        @contextmenu.prevent="showContextMenuSimple(project)"
      >
        <div class="card-thumbnail">
          <img 
            v-if="project.thumbnail" 
            :src="project.thumbnail" 
            :alt="project.name"
            @error="handleImageError"
          />
          <div v-else class="placeholder-thumb">&#128193;</div>
        </div>
        
        <div class="card-content">
          <h3 class="project-name">{{ project.name }}</h3>
          <div class="project-meta">
            <span class="file-size">{{ formatFileSize(project.fileSize) }}</span>
            <span class="last-modified">{{ formatDate(project.lastModified) }}</span>
          </div>
          <div class="project-tags">
            <span 
              v-for="tag in project.tags.slice(0, 2)" 
              :key="tag" 
              class="tag"
            >
              {{ tag }}
            </span>
            <span v-if="project.tags.length > 2" class="tag more">
              +{{ project.tags.length - 2 }}
            </span>
          </div>
        </div>
      </div>
    </div>

    <div 
      v-if="contextMenu.visible" 
      class="context-menu"
      :style="{ top: contextMenu.y + 'px', left: contextMenu.x + 'px' }"
      @click.stop
    >
      <div class="menu-item" @click="openProject(contextMenu.project)">
        Открыть
      </div>
      <div class="menu-item" @click="deleteProject(contextMenu.project)">
        Удалить
      </div>
      <div class="menu-item" @click="showProperties(contextMenu.project)">
        Свойства
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, onMounted, onBeforeUnmount } from 'vue'
import { useProjectStore } from '../../stores/project.store'
import Button from '../ui/Button.vue'

const emit = defineEmits(['create-project'])

const projectStore = useProjectStore()
const sortBy = ref('date')
const contextMenu = ref({
  visible: false,
  x: 0,
  y: 0,
  project: null
})

const loading = computed(() => projectStore.loading)
const recentProjects = computed(() => projectStore.recentProjects)
const sortedProjects = computed(() => projectStore.sortedProjects(sortBy.value))

const isSelected = (id) => projectStore.selectedProjectIds.includes(id)

const selectProjectSimple = (id) => {
  projectStore.selectProject(id, false)
}

const showContextMenuSimple = (project) => {
  const event = window.event
  if (event) {
    event.preventDefault()
    contextMenu.value = {
      visible: true,
      x: event.clientX,
      y: event.clientY,
      project
    }
  }
}

const openProject = (project) => {
  console.log('Opening project:', project.name)
  contextMenu.value.visible = false
}

const deleteProject = (project) => {
  if (confirm('Удалить проект "' + project.name + '"?')) {
    projectStore.deleteProject(project.id)
  }
  contextMenu.value.visible = false
}

const showProperties = (project) => {
  console.log('Show properties for:', project.name)
  contextMenu.value.visible = false
}

const formatFileSize = (size) => {
  if (size < 1) return Math.round(size * 1024) + ' KB'
  return size.toFixed(1) + ' MB'
}

const formatDate = (date) => {
  const now = new Date()
  const diff = now.getTime() - date.getTime()
  const days = Math.floor(diff / (1000 * 60 * 60 * 24))
  
  if (days === 0) return 'Сегодня'
  if (days === 1) return 'Вчера'
  if (days < 7) return days + ' дней назад'
  return date.toLocaleDateString('ru-RU')
}

const handleImageError = (event) => {
  const img = event.target
  img.style.display = 'none'
}

const handleClickOutside = (event) => {
  if (contextMenu.value.visible) {
    contextMenu.value.visible = false
  }
}

onMounted(() => {
  projectStore.loadProjects()
  document.addEventListener('click', handleClickOutside)
})

onBeforeUnmount(() => {
  document.removeEventListener('click', handleClickOutside)
})
</script>

<style scoped>
.recent-projects-panel {
  padding: 20px;
  background: var(--bg-secondary);
  border-radius: 8px;
  box-shadow: var(--shadow-sm);
}

.panel-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 20px;
}

.panel-header h2 {
  margin: 0;
  font-size: 1.5rem;
  color: var(--text-primary);
}

.header-actions {
  display: flex;
  gap: 12px;
}

.sort-select {
  padding: 8px 12px;
  border: 1px solid var(--border-color);
  border-radius: 6px;
  background: var(--bg-primary);
  color: var(--text-primary);
  font-size: 0.9rem;
}

.loading-state {
  text-align: center;
  padding: 40px;
  color: var(--text-secondary);
}

.spinner {
  width: 40px;
  height: 40px;
  border: 4px solid var(--border-color);
  border-top: 4px solid var(--primary-color);
  border-radius: 50%;
  animation: spin 1s linear infinite;
  margin: 0 auto 16px;
}

@keyframes spin {
  0% { transform: rotate(0deg); }
  100% { transform: rotate(360deg); }
}

.empty-state {
  text-align: center;
  padding: 60px 20px;
  color: var(--text-secondary);
}

.empty-icon {
  font-size: 3rem;
  margin-bottom: 16px;
}

.projects-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
  gap: 20px;
}

.project-card {
  background: var(--bg-primary);
  border: 1px solid var(--border-color);
  border-radius: 8px;
  overflow: hidden;
  transition: all 0.2s ease;
  cursor: pointer;
}

.project-card:hover {
  border-color: var(--primary-color);
  box-shadow: var(--shadow-md);
  transform: translateY(-2px);
}

.project-card.selected {
  border-color: var(--primary-color);
  box-shadow: 0 0 0 2px var(--primary-color);
}

.card-thumbnail {
  height: 160px;
  background: var(--bg-tertiary);
  display: flex;
  align-items: center;
  justify-content: center;
}

.card-thumbnail img {
  width: 100%;
  height: 100%;
  object-fit: cover;
}

.placeholder-thumb {
  font-size: 3rem;
  opacity: 0.5;
}

.card-content {
  padding: 16px;
}

.project-name {
  margin: 0 0 8px 0;
  font-size: 1.1rem;
  color: var(--text-primary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.project-meta {
  display: flex;
  justify-content: space-between;
  margin-bottom: 12px;
  font-size: 0.85rem;
  color: var(--text-secondary);
}

.file-size {
  font-weight: 500;
}

.project-tags {
  display: flex;
  flex-wrap: wrap;
  gap: 6px;
}

.tag {
  background: var(--bg-tertiary);
  color: var(--text-secondary);
  padding: 2px 8px;
  border-radius: 12px;
  font-size: 0.75rem;
}

.tag.more {
  background: var(--primary-color);
  color: white;
}

.context-menu {
  position: fixed;
  background: var(--bg-primary);
  border: 1px solid var(--border-color);
  border-radius: 6px;
  box-shadow: var(--shadow-lg);
  z-index: 1000;
  min-width: 150px;
}

.menu-item {
  padding: 8px 16px;
  cursor: pointer;
  font-size: 0.9rem;
  color: var(--text-primary);
}

.menu-item:hover {
  background: var(--bg-secondary);
}

.menu-item:not(:last-child) {
  border-bottom: 1px solid var(--border-color);
}
</style>
