<script setup lang="ts">
import type { LocaleCode } from "../../i18n/messages";
import { useI18nShared } from "../../i18n/useI18nShared";

const props = defineProps<{
  locale: { value: LocaleCode };
}>();

const { t } = useI18nShared(props.locale);
</script>

<template>
  <div class="stage-txt-root">
    <textarea class="stage-txt-editor" spellcheck="false">
Здесь позже будет текстовый/логовый стейдж (например, отчёты, лог, сценарии).
    </textarea>
  </div>
</template>

<style scoped>
.stage-txt-root {
  width: 100%;
  height: 100%;
  display: flex;
}

.stage-txt-editor {
  flex: 1 1 auto;
  resize: none;
  border: none;
  outline: none;
  padding: 8px;
  background-color: #101010;
  color: #ddd;
  font-family: Consolas, "Fira Code", monospace;
  font-size: 12px;
}
</style>
