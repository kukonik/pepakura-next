<template>
  <div class="settings-layout">
    <!-- ВЕРХНЯЯ ПАНЕЛЬ -->
    <div class="top-bar">
      <div class="logo">
        <i class="fas fa-cube logo-icon"></i>
        <span class="logo-text">Pepakura Next</span>
      </div>
      
      <div class="top-actions">
        <button class="action-btn" @click="saveSettings">
          <i class="fas fa-save"></i> Сохранить
        </button>
        <button class="action-btn secondary" @click="resetSettings">
          <i class="fas fa-undo"></i> Сбросить
        </button>
      </div>
    </div>

    <!-- ГЛАВНЫЙ КОНТЕНТ -->
    <div class="main-content">
      <!-- БОКОВАЯ НАВИГАЦИЯ -->
      <div class="sidebar">
        <div class="nav-section">
          <h3 class="nav-title">Основные</h3>
          <ul class="nav-list">
            <li 
              v-for="item in mainNavItems" 
              :key="item.id"
              class="nav-item"
              :class="{ active: activeSection === item.id }"
              @click="activeSection = item.id"
            >
              <i :class="item.icon"></i>
              <span>{{ item.title }}</span>
            </li>
          </ul>
        </div>
        
        <div class="nav-section">
          <h3 class="nav-title">Дополнительно</h3>
          <ul class="nav-list">
            <li 
              v-for="item in advancedNavItems" 
              :key="item.id"
              class="nav-item"
              :class="{ active: activeSection === item.id }"
              @click="activeSection = item.id"
            >
              <i :class="item.icon"></i>
              <span>{{ item.title }}</span>
            </li>
          </ul>
        </div>
      </div>

      <!-- ОСНОВНАЯ ОБЛАСТЬ НАСТРОЕК -->
      <div class="settings-content">
        <!-- ОСНОВНЫЕ НАСТРОЙКИ -->
        <div v-if="activeSection === 'general'" class="settings-section">
          <h2 class="section-title"><i class="fas fa-cog"></i> Основные настройки</h2>
          
          <div class="settings-group">
            <h3>Интерфейс</h3>
            <div class="setting-item">
              <label>Тема интерфейса</label>
              <select class="form-control" v-model="settings.theme">
                <option value="dark">Тёмная (по умолчанию)</option>
                <option value="light">Светлая</option>
                <option value="cyberpunk">Киберпанк</option>
              </select>
            </div>
            
            <div class="setting-item">
              <label>Язык интерфейса</label>
              <select class="form-control" v-model="settings.language">
                <option value="ru">Русский</option>
                <option value="en">English</option>
                <option value="ja">日本語</option>
              </select>
            </div>
            
            <div class="setting-item">
              <label>Размер шрифта</label>
              <select class="form-control" v-model="settings.fontSize">
                <option value="small">Маленький</option>
                <option value="normal">Нормальный</option>
                <option value="large">Большой</option>
              </select>
            </div>
          </div>
          
          <div class="settings-group">
            <h3>Производительность</h3>
            <div class="setting-item">
              <label>Режим рендеринга</label>
              <select class="form-control" v-model="settings.renderMode">
                <option value="auto">Авто</option>
                <option value="high">Высокое качество</option>
                <option value="low">Экономия ресурсов</option>
              </select>
            </div>
            
            <div class="setting-item">
              <label>Кэширование моделей</label>
              <div class="toggle">
                <input type="checkbox" v-model="settings.cacheModels">
                <span class="slider"></span>
              </div>
            </div>
            
            <div class="setting-item">
              <label>Автосохранение</label>
              <div class="toggle">
                <input type="checkbox" v-model="settings.autoSave">
                <span class="slider"></span>
              </div>
            </div>
          </div>
        </div>

        <!-- НАСТРОЙКИ ИМПОРТА -->
        <div v-if="activeSection === 'import'" class="settings-section">
          <h2 class="section-title"><i class="fas fa-file-import"></i> Импорт моделей</h2>
          
          <div class="settings-group">
            <h3>Форматы импорта</h3>
            <div class="setting-item" v-for="format in importFormats" :key="format.id">
              <label>{{ format.name }}</label>
              <div class="toggle">
                <input type="checkbox" v-model="format.enabled">
                <span class="slider"></span>
              </div>
            </div>
          </div>
          
          <div class="settings-group">
            <h3>Параметры импорта</h3>
            <div class="setting-item">
              <label>Масштабировать по умолчанию</label>
              <select class="form-control" v-model="settings.defaultScale">
                <option value="1">1:1 (Оригинальный)</option>
                <option value="0.1">1:10 (Миниатюра)</option>
                <option value="10">10:1 (Увеличенный)</option>
              </select>
            </div>
            
            <div class="setting-item">
              <label>Единицы измерения</label>
              <select class="form-control" v-model="settings.units">
                <option value="mm">Миллиметры (мм)</option>
                <option value="cm">Сантиметры (см)</option>
                <option value="inch">Дюймы (in)</option>
              </select>
            </div>
            
            <div class="setting-item">
              <label>Автоматическая нормализация</label>
              <div class="toggle">
                <input type="checkbox" v-model="settings.autoNormalize">
                <span class="slider"></span>
              </div>
            </div>
          </div>
        </div>

        <!-- НАСТРОЙКИ РАЗВЁРТКИ -->
        <div v-if="activeSection === 'unfold'" class="settings-section">
          <h2 class="section-title"><i class="fas fa-cut"></i> Развёртка моделей</h2>
          
          <div class="settings-group">
            <h3>Параметры развёртки</h3>
            <div class="setting-item">
              <label>Метод развёртки</label>
              <select class="form-control" v-model="settings.unfoldMethod">
                <option value="auto">Авто (рекомендуется)</option>
                <option value="manual">Ручная настройка</option>
                <option value="ai">AI оптимизация</option>
              </select>
            </div>
            
            <div class="setting-item">
              <label>Ширина клапанов по умолчанию (мм)</label>
              <input type="range" class="form-control" v-model.number="settings.defaultFlapWidth" min="5" max="25" step="1">
              <span class="range-value">{{ settings.defaultFlapWidth }} мм</span>
            </div>
            
            <div class="setting-item">
              <label>Минимальный размер детали (мм)</label>
              <input type="range" class="form-control" v-model.number="settings.minPartSize" min="1" max="20" step="1">
              <span class="range-value">{{ settings.minPartSize }} мм</span>
            </div>
          </div>
          
          <div class="settings-group">
            <h3>Опции развёртки</h3>
            <div class="setting-item">
              <label>Нумерация деталей</label>
              <div class="toggle">
                <input type="checkbox" v-model="settings.showNumbers">
                <span class="slider"></span>
              </div>
            </div>
            
            <div class="setting-item">
              <label>Линии сгиба</label>
              <div class="toggle">
                <input type="checkbox" v-model="settings.showFolds">
                <span class="slider"></span>
              </div>
            </div>
            
            <div class="setting-item">
              <label>Текстуры на развёртке</label>
              <div class="toggle">
                <input type="checkbox" v-model="settings.showTextures">
                <span class="slider"></span>
              </div>
            </div>
          </div>
        </div>

        <!-- НАСТРОЙКИ ЭКСПОРТА -->
        <div v-if="activeSection === 'export'" class="settings-section">
          <h2 class="section-title"><i class="fas fa-share-square"></i> Экспорт развёрток</h2>
          
          <div class="settings-group">
            <h3>Форматы экспорта</h3>
            <div class="setting-item" v-for="format in exportFormats" :key="format.id">
              <label>{{ format.name }}</label>
              <div class="toggle">
                <input type="checkbox" v-model="format.enabled">
                <span class="slider"></span>
              </div>
            </div>
          </div>
          
          <div class="settings-group">
            <h3>Параметры PDF</h3>
            <div class="setting-item">
              <label>Размер страницы по умолчанию</label>
              <select class="form-control" v-model="settings.defaultPageSize">
                <option value="A4">A4 (210×297 мм)</option>
                <option value="A3">A3 (297×420 мм)</option>
                <option value="Letter">Letter (216×279 мм)</option>
              </select>
            </div>
            
            <div class="setting-item">
              <label>Ориентация страницы</label>
              <select class="form-control" v-model="settings.pageOrientation">
                <option value="portrait">Портретная</option>
                <option value="landscape">Альбомная</option>
              </select>
            </div>
            
            <div class="setting-item">
              <label>Поля страницы (мм)</label>
              <input type="number" class="form-control" v-model.number="settings.pageMargin" min="0" max="50">
            </div>
          </div>
        </div>

        <!-- НАСТРОЙКИ AI -->
        <div v-if="activeSection === 'ai'" class="settings-section">
          <h2 class="section-title"><i class="fas fa-robot"></i> AI Ассистент</h2>
          
          <div class="settings-group">
            <h3>Подключение к AI</h3>
            <div class="setting-item">
              <label>Источник AI</label>
              <select class="form-control" v-model="settings.aiProvider">
                <option value="openai">OpenAI (GPT)</option>
                <option value="local">Локальная модель</option>
                <option value="custom">Пользовательский сервер</option>
              </select>
            </div>
            
            <div class="setting-item" v-if="settings.aiProvider === 'custom'">
              <label>URL сервера</label>
              <input type="text" class="form-control" v-model="settings.aiServerUrl" placeholder="http://localhost:8000">
            </div>
            
            <div class="setting-item">
              <label>Ключ API</label>
              <input type="password" class="form-control" v-model="settings.apiKey" placeholder="sk-...">
            </div>
          </div>
          
          <div class="settings-group">
            <h3>Параметры генерации</h3>
            <div class="setting-item">
              <label>Температура генерации</label>
              <input type="range" class="form-control" v-model.number="settings.aiTemperature" min="0" max="1" step="0.1">
              <span class="range-value">{{ settings.aiTemperature }}</span>
            </div>
            
            <div class="setting-item">
              <label>Максимальное количество токенов</label>
              <input type="number" class="form-control" v-model.number="settings.maxTokens" min="100" max="4000">
            </div>
            
            <div class="setting-item">
              <label>Использовать контекст проекта</label>
              <div class="toggle">
                <input type="checkbox" v-model="settings.useProjectContext">
                <span class="slider"></span>
              </div>
            </div>
          </div>
        </div>

        <!-- НАСТРОЙКИ ПРОФИЛЯ -->
        <div v-if="activeSection === 'profile'" class="settings-section">
          <h2 class="section-title"><i class="fas fa-user"></i> Профиль пользователя</h2>
          
          <div class="settings-group">
            <h3>Личная информация</h3>
            <div class="setting-item">
              <label>Имя пользователя</label>
              <input type="text" class="form-control" v-model="settings.username" placeholder="Ваше имя">
            </div>
            
            <div class="setting-item">
              <label>Email</label>
              <input type="email" class="form-control" v-model="settings.email" placeholder="your@email.com">
            </div>
            
            <div class="setting-item">
              <label>Аватар</label>
              <div class="avatar-preview">
                <img :src="settings.avatar || '/default-avatar.png'" alt="Аватар">
                <button class="action-btn" @click="changeAvatar">Изменить</button>
              </div>
            </div>
          </div>
          
          <div class="settings-group">
            <h3>Уведомления</h3>
            <div class="setting-item">
              <label>Уведомления о завершении операций</label>
              <div class="toggle">
                <input type="checkbox" v-model="settings.notifyOnComplete">
                <span class="slider"></span>
              </div>
            </div>
            
            <div class="setting-item">
              <label>Уведомления об ошибках</label>
              <div class="toggle">
                <input type="checkbox" v-model="settings.notifyOnError">
                <span class="slider"></span>
              </div>
            </div>
            
            <div class="setting-item">
              <label>Звуковые уведомления</label>
              <div class="toggle">
                <input type="checkbox" v-model="settings.soundNotifications">
                <span class="slider"></span>
              </div>
            </div>
          </div>
        </div>

        <!-- НАСТРОЙКИ СИСТЕМЫ -->
        <div v-if="activeSection === 'system'" class="settings-section">
          <h2 class="section-title"><i class="fas fa-desktop"></i> Системные настройки</h2>
          
          <div class="settings-group">
            <h3>Производительность</h3>
            <div class="setting-item">
              <label>Использование GPU</label>
              <div class="toggle">
                <input type="checkbox" v-model="settings.useGPU">
                <span class="slider"></span>
              </div>
            </div>
            
            <div class="setting-item">
              <label>Максимальный объем видеопамяти (МБ)</label>
              <input type="number" class="form-control" v-model.number="settings.maxVRAM" min="512" max="16384">
            </div>
            
            <div class="setting-item">
              <label>Количество потоков CPU</label>
              <input type="number" class="form-control" v-model.number="settings.cpuThreads" min="1" :max="maxThreads">
            </div>
          </div>
          
          <div class="settings-group">
            <h3>Хранилище</h3>
            <div class="setting-item">
              <label>Путь к каталогу проектов</label>
              <div class="path-input">
                <input type="text" class="form-control" v-model="settings.projectPath" readonly>
                <button class="action-btn" @click="browseProjectPath">Обзор</button>
              </div>
            </div>
            
            <div class="setting-item">
              <label>Путь к временным файлам</label>
              <div class="path-input">
                <input type="text" class="form-control" v-model="settings.tempPath" readonly>
                <button class="action-btn" @click="browseTempPath">Обзор</button>
              </div>
            </div>
            
            <div class="setting-item">
              <label>Автоматическая очистка временных файлов</label>
              <div class="toggle">
                <input type="checkbox" v-model="settings.autoCleanup">
                <span class="slider"></span>
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
        <span id="detailCount">Настроек: {{ settingCount }}</span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'

// Reactive state
const activeSection = ref('general')
const statusMessage = ref('Готов к работе. Настройте параметры приложения.')
const settingCount = ref(0)

// Settings data
const settings = ref({
  // Основные
  theme: 'dark',
  language: 'ru',
  fontSize: 'normal',
  renderMode: 'auto',
  cacheModels: true,
  autoSave: true,
  
  // Импорт
  defaultScale: '1',
  units: 'mm',
  autoNormalize: true,
  
  // Развёртка
  unfoldMethod: 'auto',
  defaultFlapWidth: 12,
  minPartSize: 5,
  showNumbers: true,
  showFolds: true,
  showTextures: true,
  
  // Экспорт
  defaultPageSize: 'A4',
  pageOrientation: 'portrait',
  pageMargin: 10,
  
  // AI
  aiProvider: 'openai',
  aiServerUrl: '',
  apiKey: '',
  aiTemperature: 0.7,
  maxTokens: 2000,
  useProjectContext: true,
  
  // Профиль
  username: 'Пользователь',
  email: '',
  avatar: '',
  notifyOnComplete: true,
  notifyOnError: true,
  soundNotifications: true,
  
  // Система
  useGPU: true,
  maxVRAM: 4096,
  cpuThreads: 4,
  projectPath: '',
  tempPath: '',
  autoCleanup: true
})

// Navigation items
const mainNavItems = ref([
  { id: 'general', title: 'Основные', icon: 'fas fa-cog' },
  { id: 'import', title: 'Импорт', icon: 'fas fa-file-import' },
  { id: 'unfold', title: 'Развёртка', icon: 'fas fa-cut' },
  { id: 'export', title: 'Экспорт', icon: 'fas fa-share-square' },
  { id: 'ai', title: 'AI Ассистент', icon: 'fas fa-robot' },
  { id: 'profile', title: 'Профиль', icon: 'fas fa-user' }
])

const advancedNavItems = ref([
  { id: 'system', title: 'Система', icon: 'fas fa-desktop' }
])

// Format options
const importFormats = ref([
  { id: 'obj', name: 'OBJ', enabled: true },
  { id: 'stl', name: 'STL', enabled: true },
  { id: 'gltf', name: 'GLTF/GLB', enabled: true },
  { id: 'fbx', name: 'FBX', enabled: false },
  { id: 'dae', name: 'COLLADA (DAE)', enabled: false }
])

const exportFormats = ref([
  { id: 'pdf', name: 'PDF', enabled: true },
  { id: 'svg', name: 'SVG', enabled: true },
  { id: 'dxf', name: 'DXF', enabled: true },
  { id: 'png', name: 'PNG', enabled: true },
  { id: 'jpg', name: 'JPG', enabled: true },
  { id: 'stl', name: 'STL', enabled: false }
])

// System info
const maxThreads = ref(8)

// Computed
const totalSettings = computed(() => {
  return Object.keys(settings.value).length + 
         importFormats.value.filter(f => f.enabled).length + 
         exportFormats.value.filter(f => f.enabled).length
})

// Functions
const saveSettings = () => {
  showMessage('Настройки успешно сохранены!', 'success')
  settingCount.value = totalSettings.value
}

const resetSettings = () => {
  if (confirm('Вы уверены, что хотите сбросить все настройки к значениям по умолчанию?')) {
    // В реальной реализации здесь будет сброс к значениям по умолчанию
    showMessage('Настройки сброшены к значениям по умолчанию', 'info')
  }
}

const changeAvatar = () => {
  showMessage('Открыт диалог выбора аватара', 'info')
}

const browseProjectPath = () => {
  showMessage('Открыт диалог выбора каталога проектов', 'info')
}

const browseTempPath = () => {
  showMessage('Открыт диалог выбора каталога временных файлов', 'info')
}

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
  // В реальной реализации здесь будет загрузка сохраненных настроек
  settings.value.projectPath = 'C:/Users/User/Documents/Pepakura Projects'
  settings.value.tempPath = 'C:/Users/User/AppData/Local/Temp/pepakura'
  settingCount.value = totalSettings.value
  showMessage('Настройки загружены', 'success')
})
</script>

<style scoped>
/* Основные стили */
.settings-layout {
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
  justify-content: space-between;
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

.top-actions {
  display: flex;
  gap: 0.8rem;
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

.action-btn.secondary {
  background: rgba(71, 85, 105, 0.6);
  color: #e2e8f0;
}

/* ГЛАВНЫЙ КОНТЕНТ */
.main-content {
  display: flex;
  flex: 1;
  overflow: hidden;
}

/* БОКОВАЯ НАВИГАЦИЯ */
.sidebar {
  width: 280px;
  background: rgba(15, 23, 42, 0.7);
  border-right: 1px solid rgba(255, 255, 255, 0.08);
  padding: 1.5rem;
  overflow-y: auto;
  flex-shrink: 0;
}

.nav-section {
  margin-bottom: 2rem;
}

.nav-title {
  font-size: 1rem;
  font-weight: 600;
  color: #94a3b8;
  margin-bottom: 1rem;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.nav-list {
  list-style: none;
  padding: 0;
  margin: 0;
}

.nav-item {
  padding: 0.8rem 1rem;
  border-radius: 10px;
  cursor: pointer;
  display: flex;
  align-items: center;
  gap: 0.8rem;
  transition: all 0.2s;
  margin-bottom: 0.3rem;
}

.nav-item:hover {
  background: rgba(56, 70, 95, 0.9);
  box-shadow: 0 0 10px rgba(96, 165, 250, 0.3);
}

.nav-item.active {
  background: linear-gradient(135deg, #3b82f6, #6366f1);
  color: white;
  box-shadow: 0 0 15px rgba(59, 130, 246, 0.5);
}

.nav-item i {
  width: 20px;
  text-align: center;
}

/* ОСНОВНАЯ ОБЛАСТЬ НАСТРОЕК */
.settings-content {
  flex: 1;
  overflow-y: auto;
  padding: 1.5rem;
}

.settings-section {
  max-width: 800px;
  margin: 0 auto;
}

.section-title {
  font-size: 1.5rem;
  font-weight: 600;
  color: #e2e8f0;
  display: flex;
  align-items: center;
  gap: 0.7rem;
  margin-bottom: 1.5rem;
  padding-bottom: 1rem;
  border-bottom: 1px solid rgba(255, 255, 255, 0.1);
}

.section-title i {
  color: #60a5fa;
}

.settings-group {
  background: rgba(30, 41, 59, 0.6);
  border-radius: 14px;
  padding: 1.5rem;
  border: 1px solid rgba(255, 255, 255, 0.08);
  margin-bottom: 1.5rem;
}

.settings-group h3 {
  font-size: 1.2rem;
  font-weight: 600;
  margin-bottom: 1.2rem;
  color: #cbd5e1;
}

.setting-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0.8rem 0;
  border-bottom: 1px solid rgba(255, 255, 255, 0.05);
}

.setting-item:last-child {
  border-bottom: none;
}

.setting-item label {
  font-size: 0.95rem;
  color: #cbd5e1;
  flex: 1;
}

.form-control {
  padding: 0.6rem 1rem;
  background: rgba(15, 23, 42, 0.8);
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 8px;
  color: #e2e8f0;
  font-size: 0.95rem;
  width: 250px;
}

.form-control:focus {
  outline: none;
  border-color: #60a5fa;
  box-shadow: 0 0 0 3px rgba(96, 165, 250, 0.15);
}

.path-input {
  display: flex;
  gap: 0.8rem;
  align-items: center;
}

.path-input .form-control {
  flex: 1;
}

.range-value {
  width: 50px;
  text-align: right;
  font-size: 0.9rem;
  color: #94a3b8;
}

.avatar-preview {
  display: flex;
  align-items: center;
  gap: 1rem;
}

.avatar-preview img {
  width: 60px;
  height: 60px;
  border-radius: 50%;
  object-fit: cover;
  border: 2px solid rgba(255, 255, 255, 0.1);
}

/* Утилиты */
.toggle {
  position: relative;
  display: inline-block;
  width: 50px;
  height: 24px;
}

.toggle input {
  opacity: 0;
  width: 0;
  height: 0;
}

.slider {
  position: absolute;
  cursor: pointer;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: #475569;
  transition: .4s;
  border-radius: 34px;
}

.slider:before {
  position: absolute;
  content: "";
  height: 16px;
  width: 16px;
  left: 4px;
  bottom: 4px;
  background-color: white;
  transition: .4s;
  border-radius: 50%;
}

input:checked + .slider {
  background: linear-gradient(135deg, #3b82f6, #6366f1);
}

input:checked + .slider:before {
  transform: translateX(26px);
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
  .sidebar {
    width: 220px;
  }
  
  .form-control {
    width: 200px;
  }
}

@media (max-width: 992px) {
  .sidebar {
    width: 180px;
  }
  
  .nav-item span {
    display: none;
  }
  
  .nav-item {
    justify-content: center;
  }
  
  .form-control {
    width: 150px;
  }
}

@media (max-width: 768px) {
  .top-bar {
    flex-direction: column;
    gap: 1rem;
  }
  
  .sidebar {
    width: 100%;
    height: 200px;
    border-right: none;
    border-bottom: 1px solid rgba(255, 255, 255, 0.08);
  }
  
  .main-content {
    flex-direction: column;
  }
  
  .settings-content {
    padding: 1rem;
  }
  
  .setting-item {
    flex-direction: column;
    align-items: flex-start;
    gap: 0.5rem;
  }
  
  .form-control {
    width: 100%;
  }
  
  .path-input {
    flex-direction: column;
    align-items: flex-start;
  }
  
  .status-bar {
    flex-direction: column;
    gap: 0.5rem;
    align-items: flex-start;
  }
}
</style>