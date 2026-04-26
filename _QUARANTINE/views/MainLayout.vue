<template>
  <div class="main-layout">
    <!-- Левая панель -->
    <aside class="sidebar">
      <div class="sidebar-header">
        <h1>Pepakura Next</h1>
      </div>
      
      <nav class="sidebar-nav">
        <button 
          v-for="tab in tabs" 
          :key="tab.id"
          :class="['nav-btn', { active: currentTab === tab.id }]"
          @click="currentTab = tab.id"
        >
          <span class="nav-icon">{{ tab.icon }}</span>
          <span class="nav-label">{{ tab.label }}</span>
        </button>
      </nav>
      
      <div class="sidebar-footer">
        <button @click="showSettings = true" class="settings-btn">
          ⚙️ Настройки
        </button>
      </div>
    </aside>

    <!-- Основная область -->
    <main class="content">
      <!-- Dashboard -->
      <div v-if="currentTab === 'dashboard'" class="dashboard-view">
        <QuickActions />
        <RecentProjectsPanel />
      </div>

      <!-- AI Assistant -->
      <div v-if="currentTab === 'ai'" class="ai-view">
        <AiAssistantPanel />
      </div>

      <!-- Projects -->
      <div v-if="currentTab === 'projects'" class="projects-view">
        <ProjectsList />
      </div>
    </main>

    <!-- Правая панель (AI помощник) -->
    <aside v-if="showAiPanel" class="ai-sidebar">
      <AiAssistantPanel compact />
    </aside>

    <!-- Модальные окна -->
    <SettingsModal v-if="showSettings" @close="showSettings = false" />
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { useSettingsStore } from '@/stores/settings.store'
import QuickActions from '@/components/dashboard/QuickActions.vue'
import RecentProjectsPanel from '@/components/dashboard/RecentProjectsPanel.vue'
import ProjectsList from '@/components/projects/ProjectsList.vue'
import AiAssistantPanel from '@/components/ai/AiAssistantPanel.vue'
import SettingsModal from '@/components/settings/SettingsModal.vue'

const settingsStore = useSettingsStore()

// Вкладки навигации
const tabs = [
  { id: 'dashboard', label: 'Главная', icon: '🏠' },
  { id: 'projects', label: 'Проекты', icon: '📁' },
  { id: 'ai', label: 'AI-помощник', icon: '🤖' },
]

// Текущая вкладка
const currentTab = ref('dashboard')

// Настройки
const showSettings = ref(false)
const showAiPanel = computed(() => settingsStore.settings.ai.showPanel)

// Переключение AI панели
function toggleAiPanel() {
  settingsStore.settings.ai.showPanel = !settingsStore.settings.ai.showPanel
}
</script>

<style scoped>
.main-layout {
  display: flex;
  height: 100vh;
  overflow: hidden;
}

/* Sidebar */
.sidebar {
  width: 250px;
  background: #1e1e1e;
  color: white;
  display: flex;
  flex-direction: column;
}

.sidebar-header {
  padding: 20px;
  border-bottom: 1px solid #333;
}

.sidebar-header h1 {
  margin: 0;
  font-size: 20px;
  font-weight: 600;
}

.sidebar-nav {
  flex: 1;
  padding: 10px;
}

.nav-btn {
  display: flex;
  align-items: center;
  gap: 12px;
  width: 100%;
  padding: 12px 16px;
  background: transparent;
  border: none;
  color: #aaa;
  cursor: pointer;
  border-radius: 8px;
  transition: all 0.2s;
  text-align: left;
}

.nav-btn:hover {
  background: #333;
  color: white;
}

.nav-btn.active {
  background: #1976d2;
  color: white;
}

.nav-icon {
  font-size: 20px;
}

.nav-label {
  font-size: 14px;
  font-weight: 500;
}

.sidebar-footer {
  padding: 10px;
  border-top: 1px solid #333;
}

.settings-btn {
  width: 100%;
  padding: 10px;
  background: #333;
  color: white;
  border: none;
  border-radius: 8px;
  cursor: pointer;
  transition: background 0.2s;
}

.settings-btn:hover {
  background: #444;
}

/* Content */
.content {
  flex: 1;
  overflow: auto;
  background: #f5f5f5;
  padding: 20px;
}

/* AI Sidebar */
.ai-sidebar {
  width: 350px;
  border-left: 1px solid #ddd;
  background: white;
}

/* Views */
.dashboard-view,
.ai-view,
.projects-view {
  height: 100%;
}

/* Dark theme */
:deep(.dark) .sidebar {
  background: #121212;
}

:deep(.dark) .sidebar-nav {
  border-color: #2a2a2a;
}

:deep(.dark) .nav-btn:hover {
  background: #2a2a2a;
}
</style>
