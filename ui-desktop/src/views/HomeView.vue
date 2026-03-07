<template>
  <div class="home-layout">
    <!-- ВЕРХНЯЯ ПАНЕЛЬ -->
    <div class="top-bar">
      <div class="logo">
        <i class="fas fa-cube logo-icon"></i>
        <span class="logo-text">Pepakura Next</span>
      </div>
      
      <div class="search-container">
        <i class="fas fa-search search-icon"></i>
        <input 
          type="text" 
          class="search-box" 
          v-model="searchQuery" 
          placeholder="Опишите модель, задайте вопрос AI или введите веб-адрес..."
          @keyup.enter="aiSearch"
        >
        <div class="search-actions">
          <button class="search-btn secondary" @click="webSearch">
            <i class="fas fa-globe"></i> Веб
          </button>
          <button class="search-btn primary" @click="aiSearch">
            <i class="fas fa-robot"></i> AI Поиск
          </button>
        </div>
      </div>
      
      <div style="display: flex; gap: 0.8rem;">
        <button class="action-btn" @click="openSettings">
          <i class="fas fa-cog"></i>
        </button>
      </div>
    </div>

    <!-- ГЛАВНЫЙ КОНТЕНТ -->
    <div class="main-content">
      <!-- ЛЕВАЯ ПАНЕЛЬ -->
      <div class="left-panel">
        <div class="panel-section">
          <h2 class="section-title"><i class="fas fa-star"></i> Избранное</h2>
          <div class="favorites-grid">
            <div 
              class="favorite-item" 
              v-for="item in favorites" 
              :key="item.id"
              @click="openProject(item)"
            >
              <div class="item-icon">
                <i :class="item.icon"></i>
              </div>
              <div class="item-info">
                <h3>{{ item.name }}</h3>
                <p>{{ item.description }}</p>
              </div>
            </div>
          </div>
        </div>

        <div class="panel-section">
          <h2 class="section-title"><i class="fas fa-history"></i> Недавние</h2>
          <div class="recent-list">
            <div 
              class="recent-item" 
              v-for="item in recentProjects" 
              :key="item.id"
              @click="openProject(item)"
            >
              <div class="item-icon">
                <i :class="item.icon"></i>
              </div>
              <div class="item-info">
                <h3>{{ item.name }}</h3>
                <p>{{ item.date }}</p>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- ЦЕНТРАЛЬНАЯ ОБЛАСТЬ -->
      <div class="center-panel">
        <div class="welcome-banner">
          <h1>Добро пожаловать в Pepakura Next</h1>
          <p>Создавайте потрясающие бумажные модели из 3D объектов</p>
          <div class="banner-actions">
            <button class="action-btn primary" @click="createNewProject">
              <i class="fas fa-plus"></i> Создать проект
            </button>
            <button class="action-btn secondary" @click="openProjectDialog">
              <i class="fas fa-folder-open"></i> Открыть проект
            </button>
          </div>
        </div>

        <div class="features-grid">
          <div class="feature-card">
            <div class="feature-icon">
              <i class="fas fa-cube"></i>
            </div>
            <h3>Импорт 3D моделей</h3>
            <p>Поддержка OBJ, STL, GLTF и других форматов</p>
          </div>
          
          <div class="feature-card">
            <div class="feature-icon">
              <i class="fas fa-cut"></i>
            </div>
            <h3>Авторазвёртка</h3>
            <p>Создание 2D развёрток с оптимизацией</p>
          </div>
          
          <div class="feature-card">
            <div class="feature-icon">
              <i class="fas fa-print"></i>
            </div>
            <h3>Экспорт для печати</h3>
            <p>PDF, SVG, DXF для профессиональной печати</p>
          </div>
          
          <div class="feature-card">
            <div class="feature-icon">
              <i class="fas fa-robot"></i>
            </div>
            <h3>AI Ассистент</h3>
            <p>Генерация моделей по текстовому описанию</p>
          </div>
        </div>

        <div class="quick-actions">
          <h2 class="section-title"><i class="fas fa-bolt"></i> Быстрые действия</h2>
          <div class="actions-grid">
            <button class="quick-action-btn" @click="import3D">
              <i class="fas fa-file-import"></i>
              <span>Импорт 3D</span>
            </button>
            <button class="quick-action-btn" @click="import2D">
              <i class="fas fa-image"></i>
              <span>Импорт 2D</span>
            </button>
            <button class="quick-action-btn" @click="createFromText">
              <i class="fas fa-font"></i>
              <span>Из текста</span>
            </button>
            <button class="quick-action-btn" @click="createFromWeb">
              <i class="fas fa-cloud-download-alt"></i>
              <span>Из интернета</span>
            </button>
          </div>
        </div>
      </div>

      <!-- ПРАВАЯ ПАНЕЛЬ -->
      <div class="right-panel">
        <div class="panel-section">
          <h2 class="section-title"><i class="fas fa-lightbulb"></i> AI Рекомендации</h2>
          <div class="ai-tips">
            <div class="tip-item" v-for="tip in aiTips" :key="tip.id">
              <div class="tip-icon">
                <i :class="tip.icon"></i>
              </div>
              <div class="tip-content">
                <h3>{{ tip.title }}</h3>
                <p>{{ tip.description }}</p>
              </div>
            </div>
          </div>
        </div>

        <div class="panel-section">
          <h2 class="section-title"><i class="fas fa-trophy"></i> Достижения</h2>
          <div class="achievements-grid">
            <div class="achievement-item" v-for="achievement in achievements" :key="achievement.id">
              <div class="achievement-icon">
                <i :class="achievement.icon"></i>
              </div>
              <div class="achievement-info">
                <h3>{{ achievement.name }}</h3>
                <p>{{ achievement.description }}</p>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- НИЖНИЙ СТАТУС-БАР -->
    <div class="status-bar">
      <div class="status-item">
        <div class="status-dot"></div>
        <span id="statusMessage">{{ statusMessage }}</span>
      </div>
      <div class="status-item">
        <i class="fas fa-microchip"></i>
        <span id="performanceStatus">Режим: Авто (AI доступен)</span>
        <div class="progress-bar">
          <div class="progress-fill"></div>
        </div>
      </div>
      <div class="status-item">
        <i class="fas fa-hdd"></i>
        <span>Память: <span id="memoryUsage">124 МБ</span></span>
        <i class="fas fa-layer-group" style="margin-left: 1rem;"></i>
        <span id="detailCount">Проектов: {{ projectCount }}</span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRouter } from 'vue-router'

// Router
const router = useRouter()

// Reactive state
const searchQuery = ref('')
const statusMessage = ref('Готов к работе. Загрузите модель для начала.')
const projectCount = ref(12)

// Mock data
const favorites = ref([
  { id: 1, name: 'Космический корабль', description: 'Фантастическая модель', icon: 'fas fa-rocket' },
  { id: 2, name: 'Механический дракон', description: 'Сложная модель с анимацией', icon: 'fas fa-dragon' },
  { id: 3, name: 'Архитектурный проект', description: 'Здание в стиле киберпанк', icon: 'fas fa-city' }
])

const recentProjects = ref([
  { id: 1, name: 'Робот-трансформер', date: '2 часа назад', icon: 'fas fa-robot' },
  { id: 2, name: 'Киберпанк мотоцикл', date: 'Вчера', icon: 'fas fa-motorcycle' },
  { id: 3, name: 'Механический павлин', date: '3 дня назад', icon: 'fas fa-feather' },
  { id: 4, name: 'Космическая станция', date: 'Неделю назад', icon: 'fas fa-space-shuttle' }
])

const aiTips = ref([
  { id: 1, title: 'Оптимизация развёртки', description: 'Попробуйте увеличить ширину клапанов для прочности модели', icon: 'fas fa-lightbulb' },
  { id: 2, title: 'Текстуры', description: 'Используйте UV-развертку для точного нанесения текстур', icon: 'fas fa-paint-brush' },
  { id: 3, title: 'Экспорт', description: 'Для печати выберите PDF с настройками под ваш тип бумаги', icon: 'fas fa-print' }
])

const achievements = ref([
  { id: 1, name: 'Первый проект', description: 'Создан первый проект', icon: 'fas fa-medal' },
  { id: 2, name: 'Мастер развёрток', description: 'Создано 50 развёрток', icon: 'fas fa-trophy' },
  { id: 3, name: 'AI Эксперт', description: '10 моделей создано с помощью AI', icon: 'fas fa-robot' }
])

// Functions
const aiSearch = () => {
  if (!searchQuery.value) {
    showMessage('Введите запрос для AI поиска', 'warning')
    return
  }
  showMessage(`AI анализирует запрос: "${searchQuery.value}"`, 'info')
  setTimeout(() => {
    showMessage('AI нашел 3 подходящих шаблона и создал рекомендации', 'success')
  }, 1800)
}

const webSearch = () => {
  showMessage('Открываю веб-поиск...', 'info')
}

const createNewProject = () => {
  router.push('/editor');
};

const openProjectDialog = () => {
  showMessage('Открыт диалог открытия проекта', 'info');
};

const openProject = (project: any) => {
  showMessage(`Открываю проект: ${project.name}`, 'info');
  router.push('/editor');
};

const import3D = () => {
  showMessage('Открыт диалог импорта 3D моделей', 'info');
  router.push('/editor');
};

const import2D = () => {
  showMessage('Открыт диалог импорта 2D файлов', 'info');
  router.push('/editor');
};

const createFromText = () => {
  if (!searchQuery.value) {
    showMessage('Введите описание модели для генерации', 'warning');
    return;
  }
  showMessage(`Генерация 3D модели из текста: "${searchQuery.value}"`, 'info');
  router.push('/editor');
};

const createFromWeb = () => {
  showMessage('Открыт диалог загрузки модели из интернета', 'info');
  router.push('/editor');
};

const openSettings = () => {
  showMessage('Открыты настройки', 'info');
};

const showMessage = (message: string, type: string = 'info') => {
  statusMessage.value = message
  console.log(`${type.toUpperCase()}: ${message}`)
  
  // Обновляем цвет индикатора статуса
  const dotEl = document.querySelector('.status-dot') as HTMLElement
  if (dotEl) {
    const colors: Record<string, string> = {
      info: '#3b82f6',
      success: '#10b981',
      warning: '#f59e0b',
      error: '#ef4444'
    }
    dotEl.style.backgroundColor = colors[type as keyof typeof colors]!
  }
}

// Initialize
onMounted(() => {
  showMessage('Добро пожаловать в Pepakura Next!', 'success')
})
</script>

<style scoped>
/* Основные стили */
.home-layout {
  display: flex;
  flex-direction: column;
  height: 100vh;
  background: linear-gradient(155deg, #0b1120 0%, #1a202c 100%);
  color: #e2e8f0;
  overflow: hidden;
  font-family: 'Inter', 'Segoe UI', sans-serif;
}

/* ВЕРХНЯЯ ПАНЕЛЬ */
.top-bar {
  background: rgba(15, 23, 42, 0.92);
  backdrop-filter: blur(15px);
  border-bottom: 1px solid rgba(255, 255, 255, 0.08);
  padding: 0.8rem 1.5rem;
  display: flex;
  align-items: center;
  gap: 1.5rem;
  flex-shrink: 0;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.25);
}

.logo {
  display: flex;
  align-items: center;
  gap: 0.8rem;
  font-weight: 700;
}

.logo-icon {
  color: #60a5fa;
  font-size: 1.5rem;
}

.logo-text {
  font-size: 1.3rem;
  background: linear-gradient(90deg, #60a5fa, #38bdf8);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
}

.search-container {
  flex: 1;
  max-width: 700px;
  position: relative;
}

.search-box {
  width: 100%;
  padding: 0.85rem 1rem 0.85rem 3rem;
  background: rgba(30, 41, 59, 0.8);
  border: 1px solid rgba(255, 255, 255, 0.12);
  border-radius: 12px;
  color: #e2e8f0;
  font-size: 0.95rem;
  transition: all 0.3s ease;
}

.search-box:focus {
  outline: none;
  border-color: #60a5fa;
  box-shadow: 0 0 0 3px rgba(96, 165, 250, 0.15);
}

.search-icon {
  position: absolute;
  left: 1rem;
  top: 0.95rem;
  transform: none;
  color: #94a3b8;
}

.search-actions {
  position: absolute;
  right: 0.5rem;
  top: 50%;
  transform: translateY(-50%);
  display: flex;
  gap: 0.5rem;
}

.search-btn {
  padding: 0.7rem 1.2rem;
  border-radius: 10px;
  border: none;
  font-weight: 500;
  cursor: pointer;
  display: flex;
  align-items: center;
  gap: 0.5rem;
  transition: all 0.2s;
}

.search-btn.primary {
  background: linear-gradient(135deg, #3b82f6, #6366f1);
  color: white;
}

.search-btn.secondary {
  background: rgba(71, 85, 105, 0.6);
  color: #e2e8f0;
}

.search-btn:hover {
  transform: translateY(-2px);
  box-shadow: 0 0 10px rgba(96, 165, 250, 0.3);
}

.action-btn {
  padding: 0.6rem 1.2rem;
  background: rgba(30, 41, 59, 0.8);
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 10px;
  color: #cbd5e1;
  cursor: pointer;
  display: flex;
  align-items: center;
  gap: 0.5rem;
  font-weight: 500;
  transition: all 0.2s;
}

.action-btn:hover {
  background: rgba(56, 70, 95, 0.9);
  box-shadow: 0 0 10px rgba(96, 165, 250, 0.3);
}

.action-btn.primary {
  background: linear-gradient(135deg, #3b82f6, #6366f1);
  color: white;
}

.action-btn.secondary {
  background: rgba(71, 85, 105, 0.6);
  color: #e2e8f0;
}

/* ГЛАВНЫЙ КОНТЕНТ */
.main-content {
  display: flex;
  flex: 1;
  overflow: hidden;
  padding: 1.5rem;
  gap: 1.5rem;
}

/* ПАНЕЛИ */
.left-panel, .right-panel {
  width: 300px;
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
  flex-shrink: 0;
}

.center-panel {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 2rem;
  min-width: 0;
}

/* СЕКЦИИ ПАНЕЛЕЙ */
.panel-section {
  background: rgba(30, 41, 59, 0.6);
  border-radius: 14px;
  padding: 1.5rem;
  border: 1px solid rgba(255, 255, 255, 0.08);
}

.section-title {
  font-size: 1.2rem;
  font-weight: 600;
  color: #e2e8f0;
  display: flex;
  align-items: center;
  gap: 0.7rem;
  margin-bottom: 1.2rem;
}

.section-title i {
  color: #60a5fa;
}

/* ИЗБРАННОЕ */
.favorites-grid {
  display: grid;
  grid-template-columns: 1fr;
  gap: 1rem;
}

.favorite-item {
  display: flex;
  align-items: center;
  gap: 1rem;
  padding: 1rem;
  background: rgba(15, 23, 42, 0.6);
  border-radius: 12px;
  border: 1px solid rgba(255, 255, 255, 0.08);
  cursor: pointer;
  transition: all 0.2s;
}

.favorite-item:hover {
  background: rgba(56, 70, 95, 0.9);
  transform: translateY(-2px);
  box-shadow: 0 0 10px rgba(96, 165, 250, 0.3);
}

.item-icon {
  width: 40px;
  height: 40px;
  background: rgba(30, 41, 59, 0.8);
  border-radius: 10px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 1.2rem;
  color: #60a5fa;
}

.item-info h3 {
  font-size: 1rem;
  font-weight: 600;
  margin-bottom: 0.2rem;
}

.item-info p {
  font-size: 0.85rem;
  color: #94a3b8;
}

/* НЕДАВНИЕ */
.recent-list {
  display: flex;
  flex-direction: column;
  gap: 0.8rem;
}

.recent-item {
  display: flex;
  align-items: center;
  gap: 1rem;
  padding: 0.8rem;
  background: rgba(15, 23, 42, 0.6);
  border-radius: 10px;
  border: 1px solid rgba(255, 255, 255, 0.08);
  cursor: pointer;
  transition: all 0.2s;
}

.recent-item:hover {
  background: rgba(56, 70, 95, 0.9);
  transform: translateY(-2px);
  box-shadow: 0 0 10px rgba(96, 165, 250, 0.3);
}

/* ЦЕНТРАЛЬНАЯ ОБЛАСТЬ */
.welcome-banner {
  background: linear-gradient(135deg, rgba(30, 41, 59, 0.8), rgba(15, 23, 42, 0.9));
  border-radius: 16px;
  padding: 2.5rem;
  text-align: center;
  border: 1px solid rgba(255, 255, 255, 0.1);
  box-shadow: 0 10px 30px rgba(0, 0, 0, 0.3);
}

.welcome-banner h1 {
  font-size: 2.2rem;
  font-weight: 700;
  margin-bottom: 1rem;
  background: linear-gradient(90deg, #60a5fa, #38bdf8);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
}

.welcome-banner p {
  font-size: 1.1rem;
  color: #cbd5e1;
  max-width: 600px;
  margin: 0 auto 2rem;
}

.banner-actions {
  display: flex;
  justify-content: center;
  gap: 1.5rem;
}

.features-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
  gap: 1.5rem;
}

.feature-card {
  background: rgba(30, 41, 59, 0.6);
  border-radius: 14px;
  padding: 2rem 1.5rem;
  text-align: center;
  border: 1px solid rgba(255, 255, 255, 0.08);
  transition: all 0.3s;
}

.feature-card:hover {
  transform: translateY(-5px);
  box-shadow: 0 10px 20px rgba(0, 0, 0, 0.2);
  border-color: rgba(96, 165, 250, 0.3);
}

.feature-icon {
  width: 60px;
  height: 60px;
  background: rgba(30, 41, 59, 0.8);
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 1.8rem;
  color: #60a5fa;
  margin: 0 auto 1.2rem;
}

.feature-card h3 {
  font-size: 1.2rem;
  font-weight: 600;
  margin-bottom: 0.8rem;
}

.feature-card p {
  color: #94a3b8;
  font-size: 0.95rem;
}

.quick-actions {
  background: rgba(30, 41, 59, 0.6);
  border-radius: 14px;
  padding: 1.5rem;
  border: 1px solid rgba(255, 255, 255, 0.08);
}

.actions-grid {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 1rem;
  margin-top: 1rem;
}

.quick-action-btn {
  padding: 1.2rem;
  background: rgba(15, 23, 42, 0.8);
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 12px;
  color: #cbd5e1;
  cursor: pointer;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 0.5rem;
  transition: all 0.2s;
}

.quick-action-btn:hover {
  background: rgba(56, 70, 95, 0.9);
  transform: translateY(-3px);
  box-shadow: 0 0 10px rgba(96, 165, 250, 0.3);
}

.quick-action-btn i {
  font-size: 1.5rem;
}

/* ПРАВАЯ ПАНЕЛЬ */
.ai-tips, .achievements-grid {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.tip-item, .achievement-item {
  display: flex;
  gap: 1rem;
  padding: 1rem;
  background: rgba(15, 23, 42, 0.6);
  border-radius: 12px;
  border: 1px solid rgba(255, 255, 255, 0.08);
}

.tip-icon, .achievement-icon {
  width: 40px;
  height: 40px;
  background: rgba(30, 41, 59, 0.8);
  border-radius: 10px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 1.2rem;
  color: #60a5fa;
}

.tip-content h3, .achievement-info h3 {
  font-size: 1rem;
  font-weight: 600;
  margin-bottom: 0.2rem;
}

.tip-content p, .achievement-info p {
  font-size: 0.85rem;
  color: #94a3b8;
}

/* НИЖНИЙ СТАТУС-БАР */
.status-bar {
  background: rgba(15, 23, 42, 0.95);
  border-top: 1px solid rgba(255, 255, 255, 0.08);
  padding: 0.8rem 1.5rem;
  display: flex;
  justify-content: space-between;
  font-size: 0.85rem;
  flex-shrink: 0;
}

.status-item {
  display: flex;
  align-items: center;
  gap: 0.7rem;
}

.status-dot {
  width: 10px;
  height: 10px;
  border-radius: 50%;
  background: #10b981;
}

.progress-bar {
  width: 180px;
  height: 6px;
  background: rgba(255, 255, 255, 0.1);
  border-radius: 3px;
  overflow: hidden;
}

.progress-fill {
  height: 100%;
  background: linear-gradient(90deg, #3b82f6, #60a5fa);
  width: 65%;
}

/* Адаптивность */
@media (max-width: 1200px) {
  .left-panel, .right-panel {
    width: 250px;
  }
  
  .actions-grid {
    grid-template-columns: repeat(2, 1fr);
  }
}

@media (max-width: 992px) {
  .main-content {
    flex-direction: column;
  }
  
  .left-panel, .right-panel {
    width: 100%;
    flex-direction: row;
  }
  
  .panel-section {
    flex: 1;
  }
  
  .actions-grid {
    grid-template-columns: repeat(4, 1fr);
  }
}

@media (max-width: 768px) {
  .top-bar {
    flex-wrap: wrap;
  }
  
  .search-container {
    order: 3;
    max-width: 100%;
    margin-top: 0.5rem;
  }
  
  .welcome-banner {
    padding: 1.5rem;
  }
  
  .welcome-banner h1 {
    font-size: 1.8rem;
  }
  
  .banner-actions {
    flex-direction: column;
    gap: 1rem;
  }
  
  .actions-grid {
    grid-template-columns: repeat(2, 1fr);
  }
  
  .left-panel, .right-panel {
    flex-direction: column;
  }
}
</style>