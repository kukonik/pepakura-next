<template>
  <div class="home-container">
    <div class="header-section">
      <div class="welcome-message">
        <h1>Добро пожаловать, {{ userName }}!</h1>
        <p>Создавайте удивительные бумажные модели с AI-помощью</p>
      </div>
      <div class="search-container">
        <GlobalSearch />
      </div>
    </div>
    
    <div class="content-grid">
      <div class="main-column">
        <QuickActionsPanel class="panel" />
        
        <div class="panel">
          <RecentProjectsPanel />
        </div>
      </div>
      
      <div class="sidebar">
        <div class="panel stats-panel">
          <h2 class="section-title">Статистика</h2>
          <div class="stats-grid">
            <div class="stat-item">
              <i-mdi:shape class="stat-icon" />
              <div>
                <div class="stat-value">{{ stats.total }}</div>
                <div class="stat-label">Моделей</div>
              </div>
            </div>
            <div class="stat-item">
              <i-mdi:paper-cut-vertical class="stat-icon" />
              <div>
                <div class="stat-value">{{ stats.totalSize.toFixed(1) }}</div>
                <div class="stat-label">ГБ</div>
              </div>
            </div>
            <div class="stat-item">
              <i-mdi:clock-time-eight class="stat-icon" />
              <div>
                <div class="stat-value">{{ stats.recentCount }}</div>
                <div class="stat-label">Недавние</div>
              </div>
            </div>
          </div>
        </div>
        
        <div class="panel ai-panel">
          <h2 class="section-title">AI Помощник</h2>
          <div class="ai-content">
            <div class="ai-avatar">
              <i-mdi:robot class="avatar-icon" />
            </div>
            <p class="ai-message">
              Привет! Я могу помочь вам создать проект, оптимизировать развертку или найти идеи для моделирования. Просто спросите!
            </p>
            <Button @click="openAIChat" variant="secondary" class="ai-button">
              Начать диалог
            </Button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import { useProjectStore } from '@/stores/project.store'
import GlobalSearch from '@/components/ui/GlobalSearch.vue'
import QuickActionsPanel from '@/components/layout/QuickActionsPanel.vue'
import RecentProjectsPanel from '@/components/layout/RecentProjectsPanel.vue'
import Button from '@/components/ui/Button.vue'

const projectStore = useProjectStore()
const userName = ref('Алексей')

const stats = computed(() => projectStore.stats)

const openAIChat = () => {
  console.log('Открытие AI чата')
  // Здесь будет логика открытия AI чата
}

onMounted(async () => {
  // Загрузка пользовательских данных
  try {
    // В реальном приложении здесь будет запрос к API
    const user = { name: 'Алексей' }
    userName.value = user.name || userName.value
  } catch (error) {
    console.error('Error loading user data:', error)
  }
  
  // Загрузка статистики
  if (projectStore.projects.length === 0) {
    await projectStore.loadProjects()
  }
})
</script>

<style scoped>
.home-container {
  padding: 24px;
  max-width: 1400px;
  margin: 0 auto;
}

.header-section {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  margin-bottom: 32px;
  gap: 24px;
}

.welcome-message h1 {
  font-size: 32px;
  margin-bottom: 8px;
  color: var(--text-color);
}

.welcome-message p {
  color: var(--text-secondary);
  font-size: 18px;
}

.search-container {
  flex: 1;
  max-width: 500px;
}

.content-grid {
  display: grid;
  grid-template-columns: 2fr 1fr;
  gap: 24px;
}

.panel {
  background: var(--surface-800);
  border-radius: 16px;
  padding: 24px;
  margin-bottom: 24px;
  border: 1px solid var(--border-color);
}

.section-title {
  font-size: 20px;
  margin-bottom: 16px;
  color: var(--text-color);
  font-weight: 600;
}

.stats-panel {
  margin-bottom: 24px;
}

.stats-grid {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 16px;
}

.stat-item {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 16px;
  background: var(--surface-700);
  border-radius: 12px;
}

.stat-icon {
  font-size: 28px;
  color: var(--primary-400);
}

.stat-value {
  font-size: 24px;
  font-weight: 700;
  color: var(--text-color);
}

.stat-label {
  color: var(--text-secondary);
  font-size: 14px;
}

.ai-panel {
  background: linear-gradient(135deg, var(--surface-800) 0%, var(--primary-900) 100%);
  border: 1px solid var(--primary-700);
}

.ai-content {
  display: flex;
  flex-direction: column;
  align-items: center;
  text-align: center;
}

.ai-avatar {
  width: 80px;
  height: 80px;
  background: var(--primary-500);
  border-radius: 50%;
  display: flex;
  justify-content: center;
  align-items: center;
  margin-bottom: 20px;
}

.avatar-icon {
  font-size: 40px;
  color: white;
}

.ai-message {
  color: var(--text-color);
  margin-bottom: 24px;
  line-height: 1.5;
}

.ai-button {
  width: 100%;
}
</style>
