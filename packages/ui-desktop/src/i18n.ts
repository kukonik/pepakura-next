import { createI18n } from 'vue-i18n'
import en from '../../platform/desktop/ui-desktop/src/locales/en.json'
import ru from '../../platform/desktop/ui-desktop/src/locales/ru.json'

const messages = {
  en,
  ru
}

const i18n = createI18n({
  legacy: false,
  locale: 'ru',
  fallbackLocale: 'en',
  messages
})

export default i18n
