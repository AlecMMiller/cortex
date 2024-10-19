import { Input } from './input'
import { Meta, StoryObj } from '@storybook/react'
import { userEvent, within } from '@storybook/test'

const Component = Input
type ComponentPropsAndCustomArgs = React.ComponentProps<typeof Component> & {}

const meta: Meta<ComponentPropsAndCustomArgs> = {
  args: {
    className: 'w-32',
    placeholder: 'input',
  },
  component: Component,
  render: ({ ...args }) => {
    return <Input data-testid="target" {...args} />
  },
}

export default meta
type Story = StoryObj<ComponentPropsAndCustomArgs>

export const Default: Story = {}

export const WithInput: Story = {
  play: async ({ canvasElement }) => {
    const canvas = within(canvasElement)

    await userEvent.click(canvas.getByTestId('target'))
    await userEvent.keyboard('Hello World')
  },
}
