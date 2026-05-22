import './styles/main.css'

import { createApp } from 'vue'
import { createPinia } from 'pinia'
import App from './App.vue'
import router from './router'

// Создаем приложение
const app = createApp(App)

// Регистрируем плагины
app.use(createPinia())
app.use(router)

// Монтируем приложение
app.mount('#app')