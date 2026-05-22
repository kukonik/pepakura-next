<template>
  <button class="toggle-theme" @click="toggleTheme" :aria-label="`Переключить тему на ${nextTheme}`">
    <i class="icon" :class="themeIcon"></i>
    {{ nextTheme }}
  </button>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useUiStore } from '../../stores/ui'

const uiStore = useUiStore()
const isDark = computed(() => uiStore.isDarkMode)

const nextTheme = computed(() => isDark.value ? 'Светлая' : 'Тёмная')
const themeIcon = computed(() => isDark.value ? 'icon-sun' : 'icon-moon')

const toggleTheme = () => {
  uiStore.toggleDarkMode()
}
</script>

<style scoped>
.toggle-theme {
  background: var(--button-bg);
  color: var(--button-text);
  border: 1px solid var(--button-border);
  border-radius: 4px;
  padding: 8px 16px;
  cursor: pointer;
  display: flex;
  align-items: center;
  gap: 8px;
  transition: all 0.2s;
}

.toggle-theme:hover {
  background: var(--button-hover-bg);
  border-color: var(--button-hover-border);
}

.icon {
  width: 16px;
  height: 16px;
}
</style>