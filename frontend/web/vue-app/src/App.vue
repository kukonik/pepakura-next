<template>
  <div>
    <nav style="padding:1em; background:#e8e8e8;">
      <router-link to="/" style="margin-right:1em;">Галерея</router-link>
      <router-link to="/prompt" style="margin-right:1em;">Редактор промта</router-link>
      <router-link to="/model" style="margin-right:1em;">Модель</router-link>
      <router-link to="/settings">Настройки</router-link>
    </nav>
    <router-view />
  </div>
</template>

<script>
export default {
  name: 'App'
}
</script>

<style>
body { background: #fafafa; font-family: sans-serif; }
nav a { text-decoration: none; color: #185089; font-weight: bold;}
nav a.router-link-exact-active { color: #db284c; }
</style>
