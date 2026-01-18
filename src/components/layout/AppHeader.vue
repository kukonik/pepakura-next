<!-- src/components/layout/AppHeader.vue -->
<template>
  <header class="app-header">
    <div class="app-header__left">
      <h1 class="app-header__title">Pepakura Next</h1>
    </div>
    <div class="app-header__right">
      <slot />
    </div>
  </header>
</template>

<script setup lang="ts">
</script>

<style scoped>
.app-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0.75rem 1rem;
  border-bottom: 1px solid rgba(255, 255, 255, 0.08);
  background-color: var(--color-surface, #1e1e1e);
}

.app-header__title {
  margin: 0;
  font-size: 1rem;
  font-weight: 600;
}
</style>
