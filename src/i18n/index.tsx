import i18n from 'i18next'
import { initReactI18next } from 'react-i18next'
import { en } from './en'
import { ja } from './ja'

const resources = {
  en,
  ja,
}

i18n
  .use(initReactI18next) // passes i18n down to react-i18next
  .init({
    resources,
    interpolation: {
      escapeValue: false, // react already safes from xss
    },
  })
