<template>
  <div class="about-container">
    <h1>О проекте Pepakura Next</h1>
    <p>Это страница-заглушка для маршрута /about.</p>
    <router-link to="/" class="back-link">← Вернуться к вьюверу</router-link>
  </div>
</template>

<script setup lang="ts">
// Здесь можно добавить логику About страницы
</script>

<style scoped>
.about-container {
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  height: 100vh;
  color: white;
  background-color: #1a1a1a;
  text-align: center;
}

h1 { margin-bottom: 1rem; }

.back-link {
  margin-top: 2rem;
  color: #6c5ce7;
  text-decoration: none;
  font-size: 1.2rem;
  cursor: pointer;
}
.back-link:hover { text-decoration: underline; }
</style>
