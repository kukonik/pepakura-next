<!-- src/views/HomeView.vue -->
<template>
  <div class="home-view">
    <!-- Приветственная секция -->
    <div class="welcome-section">
      <div class="welcome-content">
        <h1 class="welcome-title">
          <span class="gradient-text">Добро пожаловать в Pepakura Next</span>
        </h1>
        <p class="welcome-subtitle">
          Современное приложение для развертки 3D-моделей с искусственным интеллектом
        </p>
        <div class="welcome-stats">
          <span class="stat">
            <strong>{{ projectStore.stats.total }}</strong> проектов
          </span>
          <span class="stat">
            <strong>{{ projectStore.stats.recentCount }}</strong> недавних
          </span>
          <span class="stat">
            <strong>{{ projectStore.stats.totalSize.toFixed(1) }}</strong> МБ
          </span>
        </div>
      </div>
      <div class="welcome-actions">
        <Button 
          variant="primary" 
          size="lg"
          @click="handleCreateProject"
        >
          <i class="icon-plus"></i>
          Новый проект
        </Button>
        <Button 
          variant="outline" 
          size="lg"
          @click="handleImportProject"
        >
          <i class="icon-upload"></i>
          Импорт модели
        </Button>
      </div>
    </div>

    <!-- Основной контент в две колонки -->
    <div class="main-content">
      <!-- Левая колонка: Быстрые действия -->
      <div class="column-left">
        <QuickActionsPanel
          @create-project="handleCreateProject"
          @import-project="handleImportProject"
          @open-settings="openSettings"
          @open-tutorial="openTutorial"
        />
        
        <!-- Блок с глобальным поиском -->
        <div class="search-section">
          <h3 class="section-title">
            <i class="icon-search"></i>
            Глобальный поиск по проектам
          </h3>
          <Input
            v-model="searchQuery"
            placeholder="Поиск по названию, тегам, содержимому..."
            size="lg"
            :icon="searchQuery ? 'icon-x' : 'icon-search'"
            @icon-click="searchQuery = ''"
          >
            <template #suffix>
              <Button 
                variant="ghost" 
                size="sm"
                :disabled="!searchQuery"
                @click="performSearch"
              >
                Найти
              </Button>
            </template>
          </Input>
          <div v-if="recentSearches.length" class="recent-searches">
            <div class="recent-title">Недавние поиски:</div>
            <div class="search-tags">
              <span 
                v-for="term in recentSearches" 
                :key="term"
                class="search-tag"
                @click="searchQuery = term"
              >
                {{ term }}
                <i class="icon-x" @click.stop="removeRecentSearch(term)"></i>
              </span>
            </div>
          </div>
        </div>
      </div>

      <!-- Правая колонка: Недавние проекты -->
      <div class="column-right">
        <RecentProjectsPanel
          ref="projectsPanel"
          @create-project="handleCreateProject"
          @open-project="openProject"
          @delete-projects="handleDeleteProjects"
        />
        
        <!-- Статус системы -->
        <div class="system-status">
          <div class="status-header">
            <h3 class="status-title">
              <i class="icon-server"></i>
              Статус системы
            </h3>
            <div class="status-indicator" :class="systemStatus">
              {{ statusText }}
            </div>
          </div>
          <div class="status-details">
            <div class="status-item">
              <span class="item-label">Rust ядро:</span>
              <span class="item-value status-good">Работает</span>
            </div>
            <div class="status-item">
              <span class="item-label">Память:</span>
              <span class="item-value">{{ memoryUsage }}%</span>
            </div>
            <div class="status-item">
              <span class="item-label">AI помощник:</span>
              <span class="item-value status-warning">Включен</span>
            </div>
            <div class="status-item">
              <span class="item-label">Обновления:</span>
              <span class="item-value status-good">Доступны</span>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Модальное окно создания проекта -->
    <div v-if="showCreateModal" class="modal-overlay" @click.self="closeModal">
      <div class="modal">
        <div class="modal-header">
          <h2>Создание нового проекта</h2>
          <Button variant="ghost" size="sm" @click="closeModal">
            <i class="icon-x"></i>
          </Button>
        </div>
        <div class="modal-body">
          <div class="form-group">
            <label>Название проекта</label>
            <Input 
              v-model="newProjectName" 
              placeholder="Введите название проекта"
              autofocus
            />
          </div>
          <div class="form-group">
            <label>Шаблон (опционально)</label>
            <div class="template-grid">
              <div 
                v-for="template in templates" 
                :key="template.id"
                class="template-card"
                :class="{ selected: selectedTemplate === template.id }"
                @click="selectedTemplate = template.id"
              >
                <div class="template-icon">
                  <i :class="template.icon"></i>
                </div>
                <div class="template-name">{{ template.name }}</div>
              </div>
            </div>
          </div>
        </div>
        <div class="modal-footer">
          <Button variant="outline" @click="closeModal">Отмена</Button>
          <Button 
            variant="primary" 
            :disabled="!newProjectName.trim()"
            @click="confirmCreateProject"
          >
            Создать проект
          </Button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { useProjectStore } from '@/stores/project.store'
import Button from '@/components/ui/Button.vue'
import Input from '@/components/ui/Input.vue'
import RecentProjectsPanel from '@/components/layout/RecentProjectsPanel.vue'
import QuickActionsPanel from '@/components/layout/QuickActionsPanel.vue'

// Router
const router = useRouter()

// Store
const projectStore = useProjectStore()

// Refs
const searchQuery = ref('')
const showCreateModal = ref(false)
const newProjectName = ref('')
const selectedTemplate = ref<string | null>(null)
const projectsPanel = ref()

// Состояние системы
const systemStatus = ref('good') // good, warning, error
const memoryUsage = ref(45)

// Данные
const recentSearches = ref(['дракон', 'архитектура', 'игрушка'])
const templates = [
  { id: 'empty', name: 'Пустой проект', icon: 'icon-file' },
  { id: 'box', name: 'Коробка', icon: 'icon-cube' },
  { id: 'sphere', name: 'Сфера', icon: 'icon-globe' },
  { id: 'animal', name: 'Животное', icon: 'icon-paw' }
]

// Computed
const statusText = computed(() => {
  switch (systemStatus.value) {
    case 'good': return 'Стабильно'
    case 'warning': return 'Внимание'
    case 'error': return 'Ошибка'
    default: return 'Неизвестно'
  }
})

// Methods
const handleCreateProject = () => {
  newProjectName.value = ''
  selectedTemplate.value = null
  showCreateModal.value = true
}

const handleImportProject = async () => {
  // Здесь будет интеграция с Tauri для выбора файла
  console.log('Import project')
  
  // Временная реализация
  const project = await projectStore.createProject('Импортированная модель')
  openProject(project)
}

const openProject = (project: any) => {
  router.push({ 
    name: 'editor', 
    query: { projectId: project.id } 
  })
}

const handleDeleteProjects = (ids: string[]) => {
  console.log('Delete projects:', ids)
  // Здесь может быть дополнительная логика
}

const openSettings = () => {
  router.push({ name: 'settings' })
}

const openTutorial = () => {
  router.push({ name: 'tutorial' })
}

const performSearch = () => {
  if (!searchQuery.value.trim()) return
  
  if (!recentSearches.value.includes(searchQuery.value)) {
    recentSearches.value.unshift(searchQuery.value)
    if (recentSearches.value.length > 5) {
      recentSearches.value.pop()
    }
  }
  
  console.log('Searching for:', searchQuery.value)
  // Здесь будет интеграция с поиском
}

const removeRecentSearch = (term: string) => {
  recentSearches.value = recentSearches.value.filter(t => t !== term)
}

const closeModal = () => {
  showCreateModal.value = false
  newProjectName.value = ''
  selectedTemplate.value = null
}

const confirmCreateProject = async () => {
  if (!newProjectName.value.trim()) return
  
  try {
    const project = await projectStore.createProject(newProjectName.value)
    closeModal()
    openProject(project)
  } catch (error) {
    console.error('Failed to create project:', error)
  }
}

// Lifecycle
onMounted(() => {
  // Загружаем проекты при монтировании
  projectStore.loadRecentProjects()
  
  // Мониторинг состояния системы
  setInterval(() => {
    memoryUsage.value = Math.min(100, Math.max(0, memoryUsage.value + (Math.random() - 0.5) * 10))
    
    if (memoryUsage.value > 90) {
      systemStatus.value = 'error'
    } else if (memoryUsage.value > 70) {
      systemStatus.value = 'warning'
    } else {
      systemStatus.value = 'good'
    }
  }, 5000)
})
</script>

<style scoped>
.home-view {
  padding: 2rem;
  max-width: 1440px;
  margin: 0 auto;
}

.welcome-section {
  background: linear-gradient(135deg, 
    rgba(var(--color-primary-rgb), 0.1) 0%,
    rgba(var(--color-secondary-rgb), 0.1) 100%);
  border-radius: 16px;
  padding: 2.5rem;
  margin-bottom: 2.5rem;
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 2rem;
}

.welcome-content {
  flex: 1;
}

.welcome-title {
  font-size: 2.5rem;
  font-weight: 800;
  margin: 0 0 0.75rem 0;
  line-height: 1.2;
}

.gradient-text {
  background: linear-gradient(90deg, 
    var(--color-primary) 0%,
    var(--color-secondary) 50%,
    var(--color-accent) 100%);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
}

.welcome-subtitle {
  font-size: 1.125rem;
  color: var(--color-text-secondary);
  margin: 0 0 1.5rem 0;
  max-width: 600px;
}

.welcome-stats {
  display: flex;
  gap: 2rem;
}

.stat {
  display: flex;
  flex-direction: column;
  color: var(--color-text-secondary);
}

.stat strong {
  font-size: 1.5rem;
  color: var(--color-text);
}

.welcome-actions {
  display: flex;
  gap: 1rem;
  flex-shrink: 0;
}

.main-content {
  display: grid;
  grid-template-columns: 1fr 1.5fr;
  gap: 2rem;
}

.column-left,
.column-right {
  display: flex;
  flex-direction: column;
  gap: 2rem;
}

.search-section {
  background: var(--color-background);
  border-radius: 12px;
  border: 1px solid var(--color-border);
  padding: 1.5rem;
}

.section-title {
  font-size: 1rem;
  font-weight: 600;
  display: flex;
  align-items: center;
  gap: 0.5rem;
  margin: 0 0 1rem 0;
  color: var(--color-text-secondary);
}

.recent-searches {
  margin-top: 1rem;
}

.recent-title {
  font-size: 0.875rem;
  color: var(--color-text-secondary);
  margin-bottom: 0.5rem;
}

.search-tags {
  display: flex;
  flex-wrap: wrap;
  gap: 0.5rem;
}

.search-tag {
  background: var(--color-surface);
  border: 1px solid var(--color-border);
  border-radius: 16px;
  padding: 0.375rem 0.75rem;
  font-size: 0.875rem;
  display: flex;
  align-items: center;
  gap: 0.375rem;
  cursor: pointer;
  transition: all 0.2s ease;
}

.search-tag:hover {
  background: var(--color-surface-variant);
  border-color: var(--color-primary);
}

.search-tag .icon-x {
  font-size: 0.75rem;
  opacity: 0.6;
}

.search-tag .icon-x:hover {
  opacity: 1;
}

.system-status {
  background: var(--color-background);
  border-radius: 12px;
  border: 1px solid var(--color-border);
  padding: 1.5rem;
}

.status-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 1.25rem;
}

.status-title {
  font-size: 1rem;
  font-weight: 600;
  display: flex;
  align-items: center;
  gap: 0.5rem;
  margin: 0;
}

.status-indicator {
  padding: 0.25rem 0.75rem;
  border-radius: 20px;
  font-size: 0.75rem;
  font-weight: 600;
  text-transform: uppercase;
}

.status-indicator.good {
  background: rgba(var(--color-success-rgb), 0.1);
  color: var(--color-success);
  border: 1px solid var(--color-success);
}

.status-indicator.warning {
  background: rgba(var(--color-warning-rgb), 0.1);
  color: var(--color-warning);
  border: 1px solid var(--color-warning);
}

.status-indicator.error {
  background: rgba(var(--color-error-rgb), 0.1);
  color: var(--color-error);
  border: 1px solid var(--color-error);
}

.status-details {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 1rem;
}

.status-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0.75rem;
  background: var(--color-surface);
  border-radius: 8px;
}

.item-label {
  font-size: 0.875rem;
  color: var(--color-text-secondary);
}

.item-value {
  font-size: 0.875rem;
  font-weight: 600;
}

.item-value.status-good {
  color: var(--color-success);
}

.item-value.status-warning {
  color: var(--color-warning);
}

/* Modal */
.modal-overlay {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.modal {
  background: var(--color-background);
  border-radius: 12px;
  width: 90%;
  max-width: 500px;
  max-height: 90vh;
  overflow: hidden;
  display: flex;
  flex-direction: column;
  box-shadow: 0 20px 60px rgba(0, 0, 0, 0.3);
}

.modal-header {
  padding: 1.5rem;
  border-bottom: 1px solid var(--color-border);
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.modal-header h2 {
  margin: 0;
  font-size: 1.5rem;
}

.modal-body {
  padding: 1.5rem;
  flex: 1;
  overflow-y: auto;
}

.form-group {
  margin-bottom: 1.5rem;
}

.form-group label {
  display: block;
  margin-bottom: 0.5rem;
  font-weight: 600;
  color: var(--color-text);
}

.template-grid {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 1rem;
  margin-top: 0.5rem;
}

.template-card {
  border: 2px solid var(--color-border);
  border-radius: 8px;
  padding: 1rem;
  text-align: center;
  cursor: pointer;
  transition: all 0.2s ease;
}

.template-card:hover {
  border-color: var(--color-primary);
  background: rgba(var(--color-primary-rgb), 0.05);
}

.template-card.selected {
  border-color: var(--color-primary);
  background: rgba(var(--color-primary-rgb), 0.1);
}

.template-icon {
  font-size: 2rem;
  margin-bottom: 0.5rem;
  color: var(--color-primary);
}

.template-name {
  font-size: 0.875rem;
  font-weight: 600;
}

.modal-footer {
  padding: 1.5rem;
  border-top: 1px solid var(--color-border);
  display: flex;
  justify-content: flex-end;
  gap: 1rem;
}

/* Иконки */
.icon-plus::before { content: "➕"; }
.icon-upload::before { content: "📤"; }
.icon-search::before { content: "🔍"; }
.icon-x::before { content: "✕"; }
.icon-server::before { content: "🖥️"; }
.icon-file::before { content: "📄"; }
.icon-cube::before { content: "📦"; }
.icon-globe::before { content: "🌐"; }
.icon-paw::before { content: "🐾"; }

@media (max-width: 1024px) {
  .main-content {
    grid-template-columns: 1fr;
  }
  
  .welcome-section {
    flex-direction: column;
    text-align: center;
  }
  
  .welcome-stats {
    justify-content: center;
  }
  
  .welcome-actions {
    width: 100%;
    justify-content: center;
  }
}

@media (max-width: 768px) {
  .home-view {
    padding: 1rem;
  }
  
  .welcome-title {
    font-size: 2rem;
  }
  
  .template-grid {
    grid-template-columns: 1fr;
  }
  
  .modal {
    width: 95%;
  }
}
</style>
