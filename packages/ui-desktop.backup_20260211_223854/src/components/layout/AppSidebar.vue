<template>
  <aside class="app-sidebar">
    <nav class="sidebar-nav">
      <ul>
        <li>
          <router-link to="/unfold/editor">
            <i class="icon icon-unfold"></i>
            Редактор развёртки
          </router-link>
        </li>
        <li>
          <router-link to="/ai/assistant">
            <i class="icon icon-ai"></i>
            AI Ассистент
          </router-link>
        </li>
        <li>
          <router-link to="/settings">
            <i class="icon icon-settings"></i>
            Настройки
          </router-link>
        </li>
      </ul>
    </nav>
  </aside>
</template>

<script setup lang="ts">
// App sidebar component
</script>

<style scoped>
.app-sidebar {
  width: 250px;
  background: var(--sidebar-bg);
  color: var(--sidebar-text);
  border-right: 1px solid var(--border-color);
  overflow-y: auto;
}

.sidebar-nav ul {
  list-style: none;
  padding: 0;
  margin: 0;
}

.sidebar-nav li {
  border-bottom: 1px solid var(--border-color);
}

.sidebar-nav a {
  display: flex;
  align-items: center;
  padding: 16px 20px;
  color: var(--sidebar-text);
  text-decoration: none;
  transition: background-color 0.2s;
}

.sidebar-nav a:hover {
  background: var(--sidebar-hover);
}

.sidebar-nav a.router-link-active {
  background: var(--sidebar-active);
  color: var(--sidebar-active-text);
}

.icon {
  margin-right: 12px;
  width: 20px;
  height: 20px;
}
</style>