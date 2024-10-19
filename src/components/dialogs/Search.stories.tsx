import { Dialog } from '@radix-ui/react-dialog'
import { SearchDialog } from './Search'
import { Meta, StoryObj } from '@storybook/react'
import { QueryClient, QueryClientProvider } from '@tanstack/react-query'

const queryClient = new QueryClient({
  defaultOptions: { queries: { staleTime: Infinity, refetchOnMount: true } },
})

const Component = SearchDialog
type ComponentPropsAndCustomArgs = React.ComponentProps<typeof Component>

const meta: Meta<ComponentPropsAndCustomArgs> = {
  component: Component,
  decorators: [
    (Story) => {
      queryClient.setQueryData(['cash-key'], {
        //... set your mocked data here
      })
      return (
        <QueryClientProvider client={queryClient}>
          <Story />
        </QueryClientProvider>
      )
    },
  ],
  render: ({ ...args }) => {
    return (
      <Dialog open>
        <SearchDialog {...args} />
      </Dialog>
    )
  },
}

export default meta
type Story = StoryObj<ComponentPropsAndCustomArgs>

export const Default: Story = {}
