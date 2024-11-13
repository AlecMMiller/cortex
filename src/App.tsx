import { routeTree } from './routeTree.gen'
import { RouterProvider, createRouter } from '@tanstack/react-router'
import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import { useTranslation } from 'react-i18next'
import { useEffect, useState } from 'react'
import { locale } from '@tauri-apps/plugin-os'
import { useGetSettingOrSet } from './commands/settings'

const router = createRouter({
  routeTree,
})

const queryClient = new QueryClient()

declare module '@tanstack/react-router' {
  interface Register {
    router: typeof router
  }
}

interface LanguageProviderProps {
  readonly children: React.ReactNode
}

function LanguageProvider(props: LanguageProviderProps): JSX.Element {
  const { i18n } = useTranslation()

  const [detectedLanguage, setDetectedLanguage] = useState<string>('')

  const { data, status } = useGetSettingOrSet({}, 'locale', detectedLanguage)

  useEffect(() => {
    const doSetLanguage = async () => {
      let userLocale = await locale()
      if (userLocale === null) userLocale = 'en-US'
      setDetectedLanguage(userLocale)
    }
    doSetLanguage()
  }, [])

  useEffect(() => {
    if (status === 'success') {
      i18n.changeLanguage(data.value)
    }
  }, [data, status])

  return <>{props.children}</>
}

function App(): JSX.Element {
  return (
    <QueryClientProvider client={queryClient}>
      <LanguageProvider>
        <RouterProvider router={router} />
      </LanguageProvider>
    </QueryClientProvider>
  )
}

export default App
