<template>
  <div class="not-found-view">
    <h1>404</h1>
    <p>Страница не найдена</p>
    <router-link to="/" class="button">На главную</router-link>
  </div>
</template>

<style scoped>
.not-found-view {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 100vh;
  text-align: center;
}

h1 {
  font-size: 8rem;
  color: var(--color-primary);
  margin-bottom: 1rem;
}

p {
  font-size: 1.5rem;
  color: var(--color-text-secondary);
  margin-bottom: 2rem;
}

.button {
  padding: 0.75rem 1.5rem;
  background: var(--color-primary);
  color: white;
  text-decoration: none;
  border-radius: var(--radius-md);
  font-weight: 500;
}
</style>
