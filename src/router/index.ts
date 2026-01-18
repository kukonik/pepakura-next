// src/router/index.ts
import { createRouter, createWebHashHistory } from 'vue-router'

const router = createRouter({
  history: createWebHashHistory(),
  routes: [
    {
      path: '/',
      name: 'home',
      component: () => import('@/views/HomeView.vue'),
      meta: { 
        title: 'Главная - Pepakura Next',
        hideSidebar: true 
      }
    },
    {
      path: '/editor',
      name: 'editor',
      component: () => import('@/views/EditorView.vue'),
      meta: { 
        title: 'Редактор - Pepakura Next'
      }
    },
    {
      path: '/settings',
      name: 'settings',
      component: () => import('@/views/SettingsView.vue'),
      meta: { 
        title: 'Настройки - Pepakura Next' 
      }
    },
    {
      path: '/tutorial',
      name: 'tutorial',
      component: () => import('@/views/TutorialView.vue'),
      meta: { 
        title: 'Обучение - Pepakura Next' 
      }
    },
    {
      path: '/test-loading',
      name: 'test-loading',
      component: () => import('@/views/TestLoadingView.vue'),
      meta: { 
        title: 'Тест загрузки',
        hideHeader: true,
        hideSidebar: true 
      }
    }
  ]
})

// Глобальный guard для изменения заголовка страницы
router.beforeEach((to) => {
  // Устанавливаем заголовок страницы
  if (to.meta.title) {
    document.title = to.meta.title as string
  }
})

export default router
