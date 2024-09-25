import { routeTree } from './routeTree.gen'
import { RouterProvider, createRouter } from '@tanstack/react-router'
import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import { useTranslation } from 'react-i18next'
import { useEffect } from 'react'
import { locale } from '@tauri-apps/plugin-os'

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
      const userLocale = await locale()
      if (userLocale === null) return
      console.log(`Using detected language ${userLocale}`)
      i18n.changeLanguage(userLocale)
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
