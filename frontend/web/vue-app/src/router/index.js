import { createRouter, createWebHashHistory } from 'vue-router';
import GalleryPage from '../components/GalleryPage.vue';
import PromptEditorPage from '../components/PromptEditorPage.vue';
import ModelViewerPage from '../components/ModelViewerPage.vue';
import SettingsPage from '../components/SettingsPage.vue';

const router = createRouter({
  history: createWebHashHistory(),
  routes: [
    { path: '/', component: GalleryPage },
    { path: '/prompt', component: PromptEditorPage },
    { path: '/model', component: ModelViewerPage },
    { path: '/settings', component: SettingsPage },
  ],
});

export default router;
