<template>
  <div class="log-root">
    <header class="log-header">
      <span class="title">Журнал операций</span>
      <span class="subtitle">Здесь будут сообщения AI, загрузки 3D и раскройки</span>
    </header>

    <section class="log-body">
      <p class="empty">
        Лог пока пуст. Действия приложения будут появляться здесь в хронологическом порядке.
      </p>
    </section>
  </div>
</template>

<script setup lang="ts">
</script>

<style scoped>
.log-root {
  height: 100%;
  display: flex;
  flex-direction: column;
  color: #e5e7eb;
}

.log-header {
  height: 32px;
  border-bottom: 1px solid rgba(148, 163, 184, 0.6);
  padding: 0 0.6rem;
  display: flex;
  flex-direction: column;
  justify-content: center;
  gap: 0.05rem;
  background: rgba(15, 23, 42, 0.98);
}

.title {
  font-size: 0.78rem;
  font-weight: 500;
}

.subtitle {
  font-size: 0.72rem;
  color: #9ca3af;
}

.log-body {
  flex: 1;
  padding: 0.4rem 0.6rem;
  font-size: 0.78rem;
}

.empty {
  color: #9ca3af;
  margin: 0;
}
</style>
