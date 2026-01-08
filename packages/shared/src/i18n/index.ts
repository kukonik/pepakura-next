import { createI18n } from 'vue-i18n'
import en from './locales/en.json'
import ru from './locales/ru.json'

export const i18n = createI18n({
  legacy: false,
  locale: 'ru',
  fallbackLocale: 'en',
  messages: { en, ru }
})

export async function loadLocaleMessages(locale: string) {
  try {
    const messages = await import(\./locales/\.json\)
    i18n.global.setLocaleMessage(locale, messages.default)
    return messages.default
  } catch (error) {
    console.warn(\Locale \ not found, using fallback\)
    return null
  }
}
