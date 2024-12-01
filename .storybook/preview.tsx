import React, { Suspense, useEffect } from 'react'
import type { Preview } from '@storybook/react'
import '../src/styles.css'
import {
  RouterProvider,
  createMemoryHistory,
  createRootRoute,
  createRouter,
} from '@tanstack/react-router'

import i18n from '../src/i18n'
import { I18nextProvider } from 'react-i18next'

const withI18next = (Story) => {
  return (
    <Suspense fallback={<div>loading translations...</div>}>
      <Story />
    </Suspense>
  )
}

const preview: Preview = {
  decorators: [
    (Story, context) => {
      const { locale } = context.globals

      useEffect(() => {
        i18n.changeLanguage(locale)
      }, [locale])

      return (
        <I18nextProvider i18n={i18n}>
          <RouterProvider
            router={createRouter({
              history: createMemoryHistory(),
              routeTree: createRootRoute({
                component: Story,
              }),
            })}
          />
        </I18nextProvider>
      )
    },
  ],
  parameters: {
    backgrounds: {
      default: 'Dark',
      values: [{ name: 'Dark', value: '#1e1e2e' }],
    },
    controls: {
      matchers: {
        color: /(background|color)$/i,
        date: /Date$/i,
      },
    },
  },
}

export const globalTypes = {
  locale: {
    name: 'Locale',
    description: 'Internationalization locale',
    toolbar: {
      icon: 'globe',
      items: [
        { value: 'en', title: 'English' },
        { value: 'ja', title: '日本語' },
      ],
      showName: true,
    },
  },
}

export default preview
