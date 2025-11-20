import { createApp } from 'vue'
import App from './App.vue'
import router from './router'

// Если понадобится i18n, добавим позже
createApp(App)
  .use(router)
  .mount('#app')
