<template>
  <div style="max-width:600px;margin:2em auto;">
    <h1>Редактор промта</h1>
    <textarea v-model="prompt" rows="5" style="width:100%;margin-bottom:1em;"></textarea>
    <div>
      <button @click="savePrompt" style="margin-bottom:1em;">Сохранить</button>
      <button @click="gotoModel" style="margin-left:1em;">К модели</button>
    </div>
    <div v-if="history.length">
      <h2>История:</h2>
      <ul>
        <li v-for="(item,i) in history" :key="i">
          {{ item }}
        </li>
      </ul>
    </div>
  </div>
</template>

<script>
export default {
  name: "PromptEditorPage",
  data() {
    return {
      prompt: "",
      history: []
    }
  },
  methods: {
    savePrompt() {
      if (this.prompt.trim()) {
        this.history.unshift(this.prompt)
        this.prompt = ""
      }
    },
    gotoModel() {
      this.$router.push('/model')
    }
  }
}
</script>
