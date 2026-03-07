<template>
  <div class="projects-view">
    <div class="header-section">
      <h1>Проекты</h1>
      <AppButton @click="createNewProject">
        Создать новый проект
      </AppButton>
    </div>
    
    <div class="projects-grid">
      <div 
        v-for="project in projects" 
        :key="project.id" 
        class="project-card"
        @click="openProject(project.id)"
      >
        <div class="project-thumbnail">
          <div class="placeholder-icon">📁</div>
        </div>
        <div class="project-info">
          <h3 class="project-name">{{ project.name }}</h3>
          <p class="project-path">{{ project.path }}</p>
          <div class="project-meta">
            <span class="meta-item">
              Создан: {{ formatDate(project.createdAt) }}
            </span>
            <span class="meta-item">
              Открыт: {{ formatDate(project.lastOpened) }}
            </span>
          </div>
        </div>
        <div class="project-actions">
          <AppButton 
            variant="secondary" 
            size="small" 
            @click.stop="deleteProject(project.id)"
          >
            Удалить
          </AppButton>
        </div>
      </div>
    </div>
    
    <div v-if="projects.length === 0" class="empty-state">
      <p>У вас пока нет проектов</p>
      <AppButton @click="createNewProject">
        Создать первый проект
      </AppButton>
    </div>
  </div>
</template>

<script setup lang="ts">
import { onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { useProjectsStore } from '../stores/projectsStore'
import AppButton from '../components/ui/AppButton.vue'

const router = useRouter()
const projectsStore = useProjectsStore()

onMounted(() => {
  projectsStore.loadProjects()
})

const createNewProject = () => {
  // In a real app, this would open a dialog to create a new project
  console.log('Create new project')
}

const openProject = (projectId: string) => {
  projectsStore.openProject(projectId)
  router.push(`/unfold/editor/${projectId}`)
}

const deleteProject = (projectId: string) => {
  if (confirm('Вы уверены, что хотите удалить этот проект?')) {
    projectsStore.deleteProject(projectId)
  }
}

const formatDate = (date: Date) => {
  return new Intl.DateTimeFormat('ru-RU', {
    day: '2-digit',
    month: '2-digit',
    year: 'numeric'
  }).format(date)
}

const projects = projectsStore.projects
</script>

<style scoped>
.projects-view {
  padding: 20px;
  max-width: 1200px;
  margin: 0 auto;
}

.header-section {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 30px;
}

.header-section h1 {
  margin: 0;
}

.projects-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(350px, 1fr));
  gap: 20px;
}

.project-card {
  border: 1px solid var(--border-color);
  border-radius: 8px;
  overflow: hidden;
  cursor: pointer;
  transition: all 0.2s;
  background: var(--card-bg);
}

.project-card:hover {
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
  transform: translateY(-2px);
}

.project-thumbnail {
  height: 180px;
  background: var(--project-thumbnail-bg);
  display: flex;
  align-items: center;
  justify-content: center;
}

.placeholder-icon {
  font-size: 3rem;
}

.project-info {
  padding: 16px;
}

.project-name {
  margin: 0 0 8px 0;
  font-size: 1.25rem;
}

.project-path {
  margin: 0 0 12px 0;
  color: var(--text-secondary);
  font-size: 0.875rem;
  word-break: break-all;
}

.project-meta {
  display: flex;
  flex-direction: column;
  gap: 4px;
  font-size: 0.75rem;
  color: var(--text-secondary);
}

.meta-item {
  display: block;
}

.project-actions {
  padding: 0 16px 16px 16px;
}

.empty-state {
  text-align: center;
  padding: 60px 20px;
}

.empty-state p {
  font-size: 1.25rem;
  margin-bottom: 20px;
  color: var(--text-secondary);
}
</style>