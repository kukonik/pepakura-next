<template>
  <nav>
    <ul>
      <li><router-link to="/">Галерея</router-link></li>
      <li><router-link to="/prompt">Редактор промта</router-link></li>
      <li><router-link to="/model">Модель</router-link></li>
      <li><router-link to="/settings">Настройки</router-link></li>
    </ul>
  </nav>
</template>
<script>
export default {
  name: "Sidebar"
}
</script>
<style>
nav ul { list-style: none; padding: 0; margin: 0;}
nav ul li { margin-bottom: 10px;}
</style>
