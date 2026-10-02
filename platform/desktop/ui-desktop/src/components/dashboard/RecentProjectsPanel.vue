<template>
  <div class="bg-white dark:bg-gray-800 rounded-lg shadow p-4">
    <div class="flex justify-between items-center mb-4">
      <h2 class="text-xl font-semibold text-gray-800 dark:text-gray-200">
        {{ $t('dashboard.title') }}
      </h2>
      <div class="relative">
        <input
          v-model="searchQuery"
          type="text"
          :placeholder="$t('dashboard.searchPlaceholder')"
          class="pl-10 pr-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-gray-50 dark:bg-gray-700 text-gray-900 dark:text-gray-100 focus:ring-2 focus:ring-blue-500 focus:border-transparent"
        />
        <div class="absolute left-3 top-2.5 text-gray-400">
          <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
          </svg>
        </div>
      </div>
    </div>

    <div v-if="loading" class="text-center py-8">
      <div class="inline-block animate-spin rounded-full h-8 w-8 border-b-2 border-blue-600"></div>
      <p class="mt-2 text-gray-500 dark:text-gray-400">{{ $t('dashboard.loading') }}</p>
    </div>

    <div v-else-if="filteredProjects.length === 0" class="text-center py-8 text-gray-500 dark:text-gray-400">
      <svg class="mx-auto h-12 w-12 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
      </svg>
      <p class="mt-2">{{ $t('dashboard.noProjects') }}</p>
    </div>

    <div v-else class="space-y-3 max-h-96 overflow-y-auto" ref="scrollContainer">
      <div
        v-for="project in filteredProjects"
        :key="project.id"
        class="group flex items-center justify-between p-3 border border-gray-200 dark:border-gray-700 rounded-lg hover:bg-gray-50 dark:hover:bg-gray-700 transition-colors"
        @contextmenu.prevent="openContextMenu(project, $event)"
      >
        <div class="flex items-center space-x-3">
          <div class="flex-shrink-0">
            <div class="w-10 h-10 bg-blue-100 dark:bg-blue-900 rounded-lg flex items-center justify-center">
              <svg class="w-6 h-6 text-blue-600 dark:text-blue-300" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
              </svg>
            </div>
          </div>
          <div>
            <h3 class="font-medium text-gray-900 dark:text-gray-100">{{ project.name }}</h3>
            <p class="text-sm text-gray-500 dark:text-gray-400">
              {{ $t('dashboard.created') }} {{ formatDate(project.createdAt) }} • {{ project.meshCount || 0 }} {{ $t('dashboard.models') }}
            </p>
          </div>
        </div>
        <div class="flex items-center space-x-2 opacity-0 group-hover:opacity-100 transition-opacity">
          <button
            @click="openProject(project)"
            class="p-2 text-blue-600 dark:text-blue-400 hover:bg-blue-50 dark:hover:bg-blue-900 rounded-lg"
            :title="$t('common.open')"
          >
            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z" />
            </svg>
          </button>
          <button
            @click="exportProject(project)"
            class="p-2 text-green-600 dark:text-green-400 hover:bg-green-50 dark:hover:bg-green-900 rounded-lg"
            :title="$t('dashboard.export')"
          >
            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 10v6m0 0l-3-3m3 3l3-3m2 8H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
            </svg>
          </button>
          <button
            @click="deleteProject(project)"
            class="p-2 text-red-600 dark:text-red-400 hover:bg-red-50 dark:hover:bg-red-900 rounded-lg"
            :title="$t('common.delete')"
          >
            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
            </svg>
          </button>
        </div>
      </div>
    </div>

    <!-- Контекстное меню -->
    <div
      v-if="contextMenu.visible"
      class="fixed z-50 w-48 bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-lg shadow-lg"
      :style="{ top: contextMenu.y + 'px', left: contextMenu.x + 'px' }"
      @click="contextMenu.visible = false"
    >
      <div class="py-1">
        <button @click="openProject(contextMenu.project)" class="w-full text-left px-4 py-2 text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700">
          {{ $t('common.open') }}
        </button>
        <button @click="exportProject(contextMenu.project)" class="w-full text-left px-4 py-2 text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700">
          {{ $t('dashboard.export') }}
        </button>
        <button @click="deleteProject(contextMenu.project)" class="w-full text-left px-4 py-2 text-red-600 dark:text-red-400 hover:bg-gray-100 dark:hover:bg-gray-700">
          {{ $t('common.delete') }}
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { useI18n } from 'vue-i18n';
import { useProjectStore } from '@/stores/projectStore';
import { usePlatform } from '@pepakura/shared/composables/usePlatform';
import type { ProjectInfo } from '@/types';

const { t } = useI18n();
const projectStore = useProjectStore();
const loading = ref(true);
const searchQuery = ref('');
const scrollContainer = ref<HTMLElement | null>(null);

interface ContextMenu {
  visible: boolean;
  x: number;
  y: number;
  project: ProjectInfo | null;
}

const contextMenu = ref<ContextMenu>({
  visible: false,
  x: 0,
  y: 0,
  project: null,
});

const filteredProjects = computed(() => {
  const query = searchQuery.value.toLowerCase();
  return projectStore.projects.filter(p =>
    p.name.toLowerCase().includes(query) ||
    p.tags?.some(tag => tag.toLowerCase().includes(query))
  );
});

onMounted(async () => {
  const { init } = usePlatform();
  try {
    await init();
    await projectStore.loadRecent();
  } catch (error) {
    console.error('Failed to load projects:', error);
  } finally {
    loading.value = false;
  }
});

const formatDate = (dateString: string) => {
  const date = new Date(dateString);
  return new Intl.DateTimeFormat('ru-RU', {
    day: 'numeric',
    month: 'long',
    year: 'numeric',
  }).format(date);
};

const openProject = (project: ProjectInfo) => {
  projectStore.setCurrentProject(project.id);
  // В будущем: навигация на страницу редактора
  console.log('Opening project:', project.id);
};

const exportProject = (project: ProjectInfo) => {
  console.log('Exporting project:', project.id);
  // Вызов Tauri команды экспорта
};

const deleteProject = async (project: ProjectInfo) => {
  if (!confirm(t('dashboard.deleteConfirm', { name: project.name }))) return;
  try {
    await projectStore.remove(project.id);
  } catch (error) {
    console.error('Failed to delete project:', error);
  }
};

const openContextMenu = (project: ProjectInfo, event: MouseEvent) => {
  event.preventDefault();
  contextMenu.value = {
    visible: true,
    x: event.clientX,
    y: event.clientY,
    project,
  };
};
</script>

<style scoped>
/* Стили для виртуального скроллинга (заглушка) */
.max-h-96 {
  max-height: 24rem;
}
</style>