import './styles/main.css'

import { createApp } from 'vue'
import { createPinia } from 'pinia'
import App from './App.vue'
import router from './router'
import i18n from './i18n'

// Создаем приложение
const app = createApp(App)

// Регистрируем плагины
app.use(createPinia())
app.use(i18n)
app.use(router)

// Монтируем приложение
app.mount('#app')