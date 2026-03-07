import { computed, Ref } from "vue";
import { messages, LocaleCode } from "./messages";

export function useI18nShared(locale: Ref<LocaleCode>) {
  const t = (key: string): string => {
    const loc = locale.value;
    const dict = messages[loc] ?? messages.en;
    return dict[key] ?? messages.en[key] ?? key;
  };

  const currentLocale = computed(() => locale.value);

  return {
    t,
    currentLocale,
  };
}
