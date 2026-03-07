<template>
  <header class="app-header">
    <div class="header-content">
      <h1 class="app-title">Pepakura Next</h1>
      <nav class="header-nav">
        <router-link to="/">Главная</router-link>
        <router-link to="/projects">Проекты</router-link>
        <router-link to="/settings">Настройки</router-link>
      </nav>
      <AppToggleTheme />
    </div>
  </header>
</template>

<script setup lang="ts">
import AppToggleTheme from '../ui/AppToggleTheme.vue'
</script>

<style scoped>
.app-header {
  background: var(--header-bg);
  color: var(--header-text);
  padding: 0 20px;
  box-shadow: 0 2px 4px rgba(0,0,0,0.1);
}

.header-content {
  display: flex;
  align-items: center;
  justify-content: space-between;
  height: 60px;
}

.app-title {
  margin: 0;
  font-size: 1.5rem;
}

.header-nav {
  display: flex;
  gap: 20px;
}

.header-nav a {
  color: var(--header-text);
  text-decoration: none;
  padding: 8px 16px;
  border-radius: 4px;
  transition: background-color 0.2s;
}

.header-nav a:hover {
  background: var(--header-hover);
}

.header-nav a.router-link-active {
  background: var(--header-active);
}
</style>