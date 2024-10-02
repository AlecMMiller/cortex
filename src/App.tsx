import { routeTree } from './routeTree.gen'
import { RouterProvider, createRouter } from '@tanstack/react-router'
import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import { useTranslation } from 'react-i18next'
import { useEffect } from 'react'
import { locale } from '@tauri-apps/plugin-os'
import { getOrSetSetting } from './commands/settings'

const router = createRouter({
  routeTree,
})

const queryClient = new QueryClient()

declare module '@tanstack/react-router' {
  interface Register {
    router: typeof router
  }
}

function App(): JSX.Element {
  const { i18n } = useTranslation()

  useEffect(() => {
    const doSetLanguage = async () => {
      let userLocale = await locale()
      if (userLocale === null) userLocale = 'en-US'
      console.log(`Detected language ${userLocale}`)

      const stored = await getOrSetSetting('language', userLocale)

      console.log(`Using language ${stored}`)

      i18n.changeLanguage(stored)
    }
    doSetLanguage()
  }, [])

  return (
    <QueryClientProvider client={queryClient}>
      <RouterProvider router={router} />
    </QueryClientProvider>
  )
}

export default App
