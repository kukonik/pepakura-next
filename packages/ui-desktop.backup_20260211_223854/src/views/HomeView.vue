<template>
  <div class="home-view">
    <div class="header">
      <h1>Pepakura Next</h1>
      <p class="subtitle">Advanced 3D Unfolding Tool</p>
    </div>
    
    <div class="content">
      <div class="quick-actions">
        <h2>Quick Actions</h2>
        <div class="actions-grid">
          <button class="action-card" @click="openProject">
            <div class="icon">📂</div>
            <div class="label">Open Project</div>
          </button>
          
          <button class="action-card" @click="createNewProject">
            <div class="icon">✨</div>
            <div class="label">New Project</div>
          </button>
          
          <button class="action-card" @click="importModel">
            <div class="icon">📥</div>
            <div class="label">Import Model</div>
          </button>
          
          <button class="action-card" @click="openSettings">
            <div class="icon">⚙️</div>
            <div class="label">Settings</div>
          </button>
        </div>
      </div>
      
      <div class="recent-projects">
        <h2>Recent Projects</h2>
        <div class="projects-list">
          <div 
            v-for="project in recentProjects" 
            :key="project.id"
            class="project-item"
            @click="openProject(project.id)"
          >
            <div class="project-icon">📁</div>
            <div class="project-info">
              <div class="project-name">{{ project.name }}</div>
              <div class="project-path">{{ project.path }}</div>
            </div>
            <div class="project-date">{{ formatDate(project.lastOpened) }}</div>
          </div>
          <div v-if="recentProjects.length === 0" class="no-projects">
            No recent projects
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { useRouter } from 'vue-router'

interface Project {
  id: string;
  name: string;
  path: string;
  lastOpened: Date;
}

const router = useRouter();
const recentProjects = ref<Project[]>([
  {
    id: '1',
    name: 'Robot Model',
    path: '/home/user/projects/robot.pnx',
    lastOpened: new Date(Date.now() - 86400000) // Вчера
  },
  {
    id: '2',
    name: 'Spaceship Design',
    path: '/home/user/projects/spaceship.pnx',
    lastOpened: new Date(Date.now() - 172800000) // Позавчера
  }
]);

const openProject = (projectId?: string) => {
  if (projectId) {
    // Открываем существующий проект
    console.log('Opening project:', projectId);
  } else {
    // Открываем диалог выбора проекта
    console.log('Open project dialog');
  }
};

const createNewProject = () => {
  // Создаем новый проект
  console.log('Create new project');
};

const importModel = () => {
  // Импортируем 3D модель
  console.log('Import model');
};

const openSettings = () => {
  router.push('/settings');
};

const formatDate = (date: Date) => {
  return date.toLocaleDateString();
};
</script>

<style scoped>
.home-view {
  display: flex;
  flex-direction: column;
  height: 100%;
  padding: 20px;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
}

.header {
  text-align: center;
  margin-bottom: 40px;
}

.header h1 {
  font-size: 3rem;
  margin: 0 0 10px 0;
  text-shadow: 2px 2px 4px rgba(0,0,0,0.3);
}

.subtitle {
  font-size: 1.2rem;
  opacity: 0.9;
  margin: 0;
}

.content {
  flex: 1;
  display: flex;
  flex-direction: column;
  max-width: 800px;
  margin: 0 auto;
  width: 100%;
}

.quick-actions {
  margin-bottom: 40px;
}

.quick-actions h2 {
  text-align: center;
  margin-bottom: 20px;
  font-size: 1.8rem;
}

.actions-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(150px, 1fr));
  gap: 20px;
}

.action-card {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 20px;
  background: rgba(255, 255, 255, 0.1);
  border: 1px solid rgba(255, 255, 255, 0.2);
  border-radius: 12px;
  backdrop-filter: blur(10px);
  cursor: pointer;
  transition: all 0.3s ease;
}

.action-card:hover {
  background: rgba(255, 255, 255, 0.2);
  transform: translateY(-5px);
  box-shadow: 0 10px 20px rgba(0,0,0,0.2);
}

.action-card .icon {
  font-size: 2rem;
  margin-bottom: 10px;
}

.action-card .label {
  font-size: 1.1rem;
  font-weight: 500;
}

.recent-projects h2 {
  text-align: center;
  margin-bottom: 20px;
  font-size: 1.8rem;
}

.projects-list {
  background: rgba(255, 255, 255, 0.1);
  border-radius: 12px;
  padding: 20px;
  backdrop-filter: blur(10px);
}

.project-item {
  display: flex;
  align-items: center;
  padding: 15px;
  border-bottom: 1px solid rgba(255, 255, 255, 0.1);
  cursor: pointer;
  transition: background 0.2s;
}

.project-item:hover {
  background: rgba(255, 255, 255, 0.1);
}

.project-item:last-child {
  border-bottom: none;
}

.project-icon {
  font-size: 1.5rem;
  margin-right: 15px;
}

.project-info {
  flex: 1;
}

.project-name {
  font-weight: 500;
  font-size: 1.1rem;
  margin-bottom: 4px;
}

.project-path {
  font-size: 0.9rem;
  opacity: 0.8;
}

.project-date {
  font-size: 0.9rem;
  opacity: 0.8;
}

.no-projects {
  text-align: center;
  padding: 40px;
  opacity: 0.7;
  font-style: italic;
}
</style>