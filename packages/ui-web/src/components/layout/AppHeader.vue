<template>
  <header class="app-header">
    <div class="header-content">
      <div class="logo-section">
        <h1 class="app-title">Pepakura Next</h1>
        <p class="app-subtitle">Современное приложение для развертки 3D-моделей</p>
      </div>
      
      <div class="header-actions">
        <button class="theme-toggle" @click="toggleTheme" title="Переключить тему">
          {{ isDarkMode ? '☀️' : '🌙' }}
        </button>
        
        <button class="settings-button" @click="goToSettings" title="Настройки">
          ⚙️
        </button>
      </div>
    </div>
  </header>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { useRouter } from 'vue-router'

const router = useRouter()
const isDarkMode = ref(false)

const toggleTheme = () => {
  isDarkMode.value = !isDarkMode.value
  if (isDarkMode.value) {
    document.body.classList.add('dark-theme')
    localStorage.setItem('theme', 'dark')
  } else {
    document.body.classList.remove('dark-theme')
    localStorage.setItem('theme', 'light')
  }
}

const goToSettings = () => {
  router.push('/settings')
}

onMounted(() => {
  const savedTheme = localStorage.getItem('theme')
  if (savedTheme === 'dark') {
    isDarkMode.value = true
    document.body.classList.add('dark-theme')
  }
})
</script>

<style scoped>
.app-header {
  background: var(--bg-secondary);
  border-bottom: 1px solid var(--border-color);
  padding: 20px 0;
}

.header-content {
  max-width: 1400px;
  margin: 0 auto;
  padding: 0 20px;
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.logo-section {
  flex: 1;
}

.app-title {
  margin: 0 0 8px 0;
  font-size: 2rem;
  font-weight: 700;
  color: var(--primary-color);
  letter-spacing: -0.5px;
}

.app-subtitle {
  margin: 0;
  font-size: 1rem;
  color: var(--text-secondary);
  font-weight: 400;
}

.header-actions {
  display: flex;
  gap: 12px;
  align-items: center;
}

.theme-toggle, .settings-button {
  width: 40px;
  height: 40px;
  border-radius: 50%;
  background: var(--bg-primary);
  border: 1px solid var(--border-color);
  color: var(--text-primary);
  cursor: pointer;
  font-size: 1.2rem;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.2s ease;
}

.theme-toggle:hover, .settings-button:hover {
  background: var(--primary-color);
  color: white;
  border-color: var(--primary-color);
  transform: scale(1.1);
}
</style>
      <nav class="navigation">
        <router-link to="/" class="nav-link">Главная</router-link>
        <router-link to="/editor" class="nav-link">Редактор</router-link>
        <router-link to="/assistant" class="nav-link">Ассистент</router-link>
      </nav>
