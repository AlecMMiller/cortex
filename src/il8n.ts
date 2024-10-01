import i18n from 'i18next'
import { initReactI18next } from 'react-i18next'

// the translations
// (tip move them in a JSON file and import them,
// or even better, manage them separated from your code: https://react.i18next.com/guides/multiple-translation-files)
const resources = {
  en: {
    translation: {
      Home: 'Home',
      Search: 'Search',
      'New Note': 'New Note',
      Settings: 'Settings',
      Title: 'Title',
      'Note Title': 'Note Title',
      Create: 'Create',
      'Recent Notes': 'Recent Notes',
    },
  },
  ja: {
    translation: {
      Home: 'ホーム',
      Search: '検索',
      'New Note': '新しいメモ',
      Settings: '設定',
      Title: 'タイトル',
      'Note Title': 'メモのタイトル',
      Create: '作成',
      'Recent Notes': '最近のメモ',
    },
  },
}

i18n
  .use(initReactI18next) // passes i18n down to react-i18next
  .init({
    resources,
    // lng: 'jp', // language to use, more information here: https://www.i18next.com/overview/configuration-options#languages-namespaces-resources
    // you can use the i18n.changeLanguage function to change the language manually: https://www.i18next.com/overview/api#changelanguage
    // if you're using a language detector, do not define the lng option

    interpolation: {
      escapeValue: false, // react already safes from xss
    },
  })

export default i18n
