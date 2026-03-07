<script setup lang="ts">
import type { LocaleCode } from "../../i18n/messages";
import { useI18nShared } from "../../i18n/useI18nShared";

const props = defineProps<{
  locale: { value: LocaleCode };
}>();

const { t } = useI18nShared(props.locale);
</script>

<template>
  <div class="stage2d-root">
    <div class="stage2d-placeholder">
      <p>Стейдж 2D (заглушка).</p>
      <p>Здесь позже будет редактор развёртки / листов.</p>
    </div>
  </div>
</template>

<style scoped>
.stage2d-root {
  width: 100%;
  height: 100%;
  display: flex;
}

.stage2d-placeholder {
  margin: auto;
  text-align: center;
  color: #ccc;
  font-size: 13px;
}
</style>
