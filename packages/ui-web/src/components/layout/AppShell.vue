<template>
  <div class="min-h-screen text-slate-100">
    <header class="border-b border-slate-800 bg-slate-950/90 px-4 py-2">
      <h1 class="text-lg font-semibold">
        Pepakura Next · prototype layout
      </h1>
    </header>

    <main class="h-[calc(100vh-48px)]">
      <slot />
    </main>
  </div>
</template>

<script setup lang="ts">
</script>

<style scoped>
.min-h-screen {
  min-height: 100vh;
  background: #020617;
}
</style>
