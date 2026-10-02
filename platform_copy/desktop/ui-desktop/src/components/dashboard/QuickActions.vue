<template>
  <div class="grid grid-cols-2 md:grid-cols-4 gap-4">
    <button
      @click="createNewProject"
      class="flex flex-col items-center justify-center p-6 bg-blue-50 dark:bg-blue-900/30 border-2 border-dashed border-blue-300 dark:border-blue-700 rounded-xl hover:bg-blue-100 dark:hover:bg-blue-800/50 transition-colors group"
    >
      <div class="w-12 h-12 mb-3 flex items-center justify-center bg-blue-100 dark:bg-blue-800 rounded-full">
        <svg class="w-6 h-6 text-blue-600 dark:text-blue-300" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6v6m0 0v6m0-6h6m-6 0H6" />
        </svg>
      </div>
      <span class="font-semibold text-blue-700 dark:text-blue-300">{{ $t('dashboard.newProject') }}</span>
      <p class="text-sm text-blue-600/70 dark:text-blue-400/70 mt-1">{{ $t('dashboard.newProjectHint') }}</p>
    </button>

    <button
      @click="importModel"
      class="flex flex-col items-center justify-center p-6 bg-green-50 dark:bg-green-900/30 border-2 border-dashed border-green-300 dark:border-green-700 rounded-xl hover:bg-green-100 dark:hover:bg-green-800/50 transition-colors group"
    >
      <div class="w-12 h-12 mb-3 flex items-center justify-center bg-green-100 dark:bg-green-800 rounded-full">
        <svg class="w-6 h-6 text-green-600 dark:text-green-300" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M15 13l-3-3m0 0l-3 3m3-3v12" />
        </svg>
      </div>
      <span class="font-semibold text-green-700 dark:text-green-300">{{ $t('dashboard.import') }}</span>
      <p class="text-sm text-green-600/70 dark:text-green-400/70 mt-1">{{ $t('dashboard.importHint') }}</p>
    </button>

    <button
      @click="openFromFile"
      class="flex flex-col items-center justify-center p-6 bg-purple-50 dark:bg-purple-900/30 border-2 border-dashed border-purple-300 dark:border-purple-700 rounded-xl hover:bg-purple-100 dark:hover:bg-purple-800/50 transition-colors group"
    >
      <div class="w-12 h-12 mb-3 flex items-center justify-center bg-purple-100 dark:bg-purple-800 rounded-full">
        <svg class="w-6 h-6 text-purple-600 dark:text-purple-300" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
        </svg>
      </div>
      <span class="font-semibold text-purple-700 dark:text-purple-300">{{ $t('dashboard.open') }}</span>
      <p class="text-sm text-purple-600/70 dark:text-purple-400/70 mt-1">{{ $t('dashboard.openHint') }}</p>
    </button>

    <button
      @click="openSettings"
      class="flex flex-col items-center justify-center p-6 bg-gray-50 dark:bg-gray-800/30 border-2 border-dashed border-gray-300 dark:border-gray-700 rounded-xl hover:bg-gray-100 dark:hover:bg-gray-800/50 transition-colors group"
    >
      <div class="w-12 h-12 mb-3 flex items-center justify-center bg-gray-100 dark:bg-gray-800 rounded-full">
        <svg class="w-6 h-6 text-gray-600 dark:text-gray-300" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" />
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
        </svg>
      </div>
      <span class="font-semibold text-gray-700 dark:text-gray-300">{{ $t('dashboard.settings') }}</span>
      <p class="text-sm text-gray-600/70 dark:text-gray-400/70 mt-1">{{ $t('dashboard.settingsHint') }}</p>
    </button>
  </div>
</template>

<script setup lang="ts">
import { useProjectStore } from '@/stores/projectStore';
import { useRouter } from 'vue-router';
import { useI18n } from 'vue-i18n';
import { usePlatform } from '@pepakura/shared/composables/usePlatform';

const { t } = useI18n();
const projectStore = useProjectStore();
const router = useRouter();
const { isDesktop } = usePlatform();

const createNewProject = async () => {
  const name = prompt(t('dashboard.newProjectPrompt'), t('dashboard.newProjectDefault'));
  if (!name) return;
  try {
    const projectId = await projectStore.create(name);
    console.log('Project created with ID:', projectId);
    router.push({ name: 'editor', params: { projectId } });
  } catch (error) {
    console.error('Failed to create project:', error);
    alert(t('dashboard.createError'));
  }
};

const importModel = async () => {
  if (isDesktop.value) {
    const { open } = await import('@tauri-apps/plugin-dialog');
    const selected = await open({
      multiple: false,
      filters: [{ name: '3D Models', extensions: ['obj', 'stl', 'gltf', 'fbx'] }],
    });
    if (selected === null || Array.isArray(selected)) return;
    const path = selected;
    const format = path.split('.').pop() || 'obj';
    try {
      const mesh = await projectStore.importModel(path, format);
      console.log('Model imported:', mesh);
      alert(t('dashboard.importSuccess'));
    } catch (error) {
      console.error('Failed to import model:', error);
      alert(t('dashboard.importError'));
    }
  } else {
    // Web fallback: use native file input
    const input = document.createElement('input');
    input.type = 'file';
    input.accept = '.obj,.stl,.gltf,.fbx';
    input.onchange = async (e) => {
      const file = (e.target as HTMLInputElement).files?.[0];
      if (!file) return;
      const format = file.name.split('.').pop() || 'obj';
      try {
        const arrayBuffer = await file.arrayBuffer();
        const mesh = await projectStore.importModel(new Uint8Array(arrayBuffer), format);
        console.log('Model imported:', mesh);
        alert(t('dashboard.importSuccess'));
      } catch (error) {
        console.error('Failed to import model:', error);
        alert(t('dashboard.importError'));
      }
    };
    input.click();
  }
};

const openFromFile = async () => {
  if (isDesktop.value) {
    const { open } = await import('@tauri-apps/plugin-dialog');
    const selected = await open({
      multiple: false,
      filters: [{ name: 'Pepakura Project', extensions: ['ppk', 'json'] }],
    });
    if (selected === null || Array.isArray(selected)) return;
    console.log('Opening project from file:', selected);
    alert(t('dashboard.openNotImplemented'));
  } else {
    // Web fallback
    alert(t('dashboard.openNotImplemented'));
  }
};

const openSettings = () => {
  router.push({ name: 'settings' });
};
</script>