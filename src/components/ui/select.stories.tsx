import { Select } from './select'
import { Meta, StoryObj } from '@storybook/react'
import { userEvent, within, expect, waitFor } from '@storybook/test'

type SelectPropsAndCustomArgs = React.ComponentProps<typeof Select> & {}

const meta: Meta<SelectPropsAndCustomArgs> = {
  component: Select,
  args: {
    value: 'A',
    options: [
      {
        value: 'A',
        content: 'Option A',
      },
      {
        value: 'B',
        content: 'Option B',
      },
      {
        value: 'C',
        content: 'Option C',
      },
    ],
    triggerClassname: 'w-32',
  },
  render: ({ ...args }) => {
    return <Select {...args} />
  },
}

export default meta
type Story = StoryObj<SelectPropsAndCustomArgs>

export const Default: Story = {}

export const Expand: Story = {
  play: async ({ canvasElement }) => {
    const canvas = within(canvasElement)

    await userEvent.click(canvas.getByTestId('select-trigger'))
  },
}
