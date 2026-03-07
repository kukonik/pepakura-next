<template>
  <div id="app">
    <router-view />
  </div>
</template>

<script setup lang="ts">
// Глобальные стили теперь находятся в этом файле
</script>

<style>
/* ГЛОБАЛЬНЫЕ СТИЛИ ДЛЯ ВСЕГО ПРИЛОЖЕНИЯ */

/* Основные стили */
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

html, body {
  height: 100%;
  overflow: hidden;
}

body {
  font-family: 'Inter', 'Segoe UI', sans-serif;
  background: linear-gradient(155deg, #0b1120 0%, #1a202c 100%);
  color: #e2e8f0;
  overflow: hidden;
}

#app {
  height: 100vh;
  overflow: hidden;
}

/* Стили для иконок Font Awesome */
.fas, .far, .fab {
  font-family: 'Font Awesome 5 Free';
  font-weight: 900;
}

/* Стили для кнопок */
button {
  font-family: inherit;
  cursor: pointer;
}

button:focus {
  outline: none;
}

/* Стили для инпутов */
input, select, textarea {
  font-family: inherit;
}

input:focus, select:focus, textarea:focus {
  outline: none;
}

/* Стили для скроллбаров */
::-webkit-scrollbar {
  width: 8px;
  height: 8px;
}

::-webkit-scrollbar-track {
  background: rgba(15, 23, 42, 0.5);
  border-radius: 4px;
}

::-webkit-scrollbar-thumb {
  background: rgba(96, 165, 250, 0.5);
  border-radius: 4px;
}

::-webkit-scrollbar-thumb:hover {
  background: rgba(96, 165, 250, 0.8);
}

/* Анимации */
@keyframes fadeIn {
  from { opacity: 0; transform: translateY(10px); }
  to { opacity: 1; transform: translateY(0); }
}

@keyframes pulse {
  0% { transform: scale(1); }
  50% { transform: scale(1.05); }
  100% { transform: scale(1); }
}

/* Утилиты */
.text-center {
  text-align: center;
}

.text-right {
  text-align: right;
}

.text-left {
  text-align: left;
}

.d-flex {
  display: flex;
}

.justify-content-center {
  justify-content: center;
}

.justify-content-between {
  justify-content: space-between;
}

.align-items-center {
  align-items: center;
}

.flex-column {
  flex-direction: column;
}

.flex-wrap {
  flex-wrap: wrap;
}

.gap-1 { gap: 0.5rem; }
.gap-2 { gap: 1rem; }
.gap-3 { gap: 1.5rem; }
.gap-4 { gap: 2rem; }

.m-0 { margin: 0; }
.m-1 { margin: 0.5rem; }
.m-2 { margin: 1rem; }
.m-3 { margin: 1.5rem; }
.m-4 { margin: 2rem; }

.mt-1 { margin-top: 0.5rem; }
.mt-2 { margin-top: 1rem; }
.mt-3 { margin-top: 1.5rem; }
.mt-4 { margin-top: 2rem; }

.mb-1 { margin-bottom: 0.5rem; }
.mb-2 { margin-bottom: 1rem; }
.mb-3 { margin-bottom: 1.5rem; }
.mb-4 { margin-bottom: 2rem; }

.p-1 { padding: 0.5rem; }
.p-2 { padding: 1rem; }
.p-3 { padding: 1.5rem; }
.p-4 { padding: 2rem; }

.pt-1 { padding-top: 0.5rem; }
.pt-2 { padding-top: 1rem; }
.pt-3 { padding-top: 1.5rem; }
.pt-4 { padding-top: 2rem; }

.pb-1 { padding-bottom: 0.5rem; }
.pb-2 { padding-bottom: 1rem; }
.pb-3 { padding-bottom: 1.5rem; }
.pb-4 { padding-bottom: 2rem; }

.w-100 { width: 100%; }
.h-100 { height: 100%; }

.cursor-pointer {
  cursor: pointer;
}

/* Адаптивность */
@media (max-width: 768px) {
  .hide-mobile {
    display: none !important;
  }
  
  .text-center-mobile {
    text-align: center;
  }
}

@media (max-width: 480px) {
  .hide-small {
    display: none !important;
  }
}
</style>