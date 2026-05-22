<!-- packages/shared/src/components/PepakuraLayout.vue -->
<template>
  <div class="pepakura-layout">
    <header class="pepakura-layout__header">
      <slot name="header">
        <h1 class="pepakura-layout__title">Pepakura Next</h1>
      </slot>
    </header>

    <main class="pepakura-layout__main">
      <slot />
    </main>

    <footer class="pepakura-layout__footer">
      <slot name="footer">
        <span class="pepakura-layout__footer-text">
          Pepakura Next · prototype layout
        </span>
      </slot>
    </footer>
  </div>
</template>

<script setup lang="ts">
</script>

<style scoped>
.pepakura-layout {
  display: flex;
  flex-direction: column;
  min-height: 100vh;
  background-color: #121212;
  color: rgba(255, 255, 255, 0.87);
}

.pepakura-layout__header {
  padding: 0.5rem 1rem;
  border-bottom: 1px solid rgba(255, 255, 255, 0.08);
  background-color: #1e1e1e;
}

.pepakura-layout__title {
  margin: 0;
  font-size: 1rem;
  font-weight: 600;
}

.pepakura-layout__main {
  flex: 1;
  padding: 1rem;
  overflow: auto;
}

.pepakura-layout__footer {
  padding: 0.25rem 1rem;
  border-top: 1px solid rgba(255, 255, 255, 0.08);
  background-color: #1e1e1e;
  font-size: 0.75rem;
  opacity: 0.7;
}
</style>
