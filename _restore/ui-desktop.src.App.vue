<template>
  <AppShell />
  <GlobalMessages />
</template>

<script setup lang="ts">
import AppShell from '@/components/layout/AppShell.vue'
import GlobalMessages from '@/components/ui/GlobalMessages.vue'
</script>

<style>
html,
body,
#app {
  margin: 0;
  padding: 0;
  height: 100%;
}

body {
  background: #020617;
}
</style>
