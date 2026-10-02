import { createApp } from 'vue'
import App from './App.vue'

window.onerror = function(msg, src, line, col, err) { alert('FATAL ERROR: ' + msg); return false; };
createApp(App).mount('#app')

