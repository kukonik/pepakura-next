<script setup lang="ts">
import type { LocaleCode } from "../../i18n/messages";
import { useI18nShared } from "../../i18n/useI18nShared";

const props = defineProps<{
  locale: { value: LocaleCode };
}>();

const { t } = useI18nShared(props.locale);
</script>

<template>
  <div class="stage-unfold-root">
    <div class="stage-unfold-placeholder">
      <p>Стейдж развёртки (Unfold) — заглушка.</p>
      <p>Здесь позже будет алгоритм развёртки и инструменты редактирования.</p>
    </div>
  </div>
</template>

<style scoped>
.stage-unfold-root {
  width: 100%;
  height: 100%;
  display: flex;
}

.stage-unfold-placeholder {
  margin: auto;
  text-align: center;
  color: #ccc;
  font-size: 13px;
}
</style>
