<template>
  <div class="dashboard-layout">
    <AppHeader />
    <main class="p-4 container mx-auto">
      <QuickActions class="mb-6" />
      <RecentProjectsPanel />
    </main>
  </div>
</template>

<script setup lang="ts">
import AppHeader from '@/components/common/AppHeader.vue'
import QuickActions from '@/components/dashboard/QuickActions.vue'
import RecentProjectsPanel from '@/components/dashboard/RecentProjectsPanel.vue'
</script>

<style scoped>
.dashboard-layout {
  min-height: 100vh;
  background: linear-gradient(155deg, #0b1120 0%, #1a202c 100%);
  color: #e2e8f0;
}
</style>