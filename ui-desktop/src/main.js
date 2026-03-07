import { createApp } from 'vue'
import { createPinia } from 'pinia'
import App from './App.vue'
import router from './router'

// Создаем приложение
const app = createApp(App)

// Подключаем Pinia
const pinia = createPinia()
app.use(pinia)

// Подключаем роутер
app.use(router)

// Монтируем приложение
app.mount('#app')