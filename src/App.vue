<!-- src/App.vue -->
<template>
  <div id="app">
    <AppHeader v-if="!route.meta.hideHeader" />
    <main :class="{ 'no-header': route.meta.hideHeader }">
      <router-view v-slot="{ Component }">
        <Suspense>
          <component :is="Component" />
          <template #fallback>
            <AppLoading />
          </template>
        </Suspense>
      </router-view>
    </main>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useRoute } from 'vue-router'
import AppHeader from '@/components/layout/AppHeader.vue'
import AppLoading from '@/components/ui/AppLoading.vue'

const route = useRoute()
</script>
