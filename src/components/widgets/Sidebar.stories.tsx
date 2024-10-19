import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import { Sidebar } from './Sidebar'
import { Meta, StoryObj } from '@storybook/react'
import { within, userEvent } from '@storybook/test'

const queryClient = new QueryClient({
  defaultOptions: { queries: { staleTime: Infinity, refetchOnMount: true } },
})

const Component = Sidebar
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
  parameters: {
    layout: 'fullscreen',
  },
  render: () => {
    return (
      <div className="h-screen bg-crust w-fit">
        <Sidebar />
      </div>
    )
  },
}

export default meta
type Story = StoryObj<ComponentPropsAndCustomArgs>

export const Default: Story = {}

export const Tooltip: Story = {
  play: async ({ canvasElement }) => {
    const canvas = within(canvasElement)

    await userEvent.hover(canvas.getByTestId('home'))
  },
}
