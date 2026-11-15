import { createRouter, createWebHistory } from 'vue-router'
import EditorPage from '../views/EditorPage.vue'
import HomeView from '../views/HomeView.vue'
import SettingsView from '../views/SettingsView.vue'
import ProjectsView from '../views/ProjectsView.vue'
import AiAssistantPage from '../pages/AiAssistantPage.vue'

const routes = [
  { path: '/', component: HomeView },
  { path: '/editor', component: EditorPage },
  { path: '/projects', component: ProjectsView },
  { path: '/settings', component: SettingsView },
  { path: '/ai/assistant', component: AiAssistantPage },
  { path: '/nesting-editor', component: () => import('../pages/NestingEditorPage.vue') },
  { path: '/:pathMatch(.*)*', redirect: '/editor' }
]

const router = createRouter({
  history: createWebHistory(),
  routes
})

export default router
