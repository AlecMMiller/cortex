import { Plus } from 'lucide-react'
import { Popover } from './popover'
import { Meta, StoryObj } from '@storybook/react'
import { userEvent, within } from '@storybook/test'
import { PopoverContent, PopoverTrigger } from '@radix-ui/react-popover'

type PopoverPropsAndCustomArgs = React.ComponentProps<typeof Popover> & {}

const meta: Meta<PopoverPropsAndCustomArgs> = {
  component: Popover,
  args: {},
  render: ({ ...args }) => {
    return (
      <Popover {...args}>
        <PopoverTrigger data-testid="test-target" className="p-8">
          <Plus className="text-text" />
        </PopoverTrigger>
        <PopoverContent>
          <div className="rounded-md p-2 bg-surface0 text-text">
            Hello world
          </div>
        </PopoverContent>
      </Popover>
    )
  },
}

export default meta
type Story = StoryObj<PopoverPropsAndCustomArgs>

export const Default: Story = {
  play: async ({ canvasElement }) => {
    const canvas = within(canvasElement)

    await userEvent.click(canvas.getByTestId('test-target'))
  },
}
