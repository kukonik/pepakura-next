/**
 * Роутер веб-приложения
 */

import { createRouter, createWebHistory } from 'vue-router';
import AutoSeamsView from '../views/AutoSeamsView.vue';
import ManualSeamsView from '../views/ManualSeamsView.vue';

/**
 * Роуты приложения
 */
const routes = [
  {
    path: '/',
    redirect: '/auto',
  },
  {
    path: '/auto',
    name: 'AutoSeams',
    component: AutoSeamsView,
  },
  {
    path: '/manual',
    name: 'ManualSeams',
    component: ManualSeamsView,
  },
];

/**
 * Создание роутера
 */
const router = createRouter({
  history: createWebHistory(),
  routes,
});

export default router;