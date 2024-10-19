import { Search } from 'lucide-react'
import { TooltipButton } from './button-tooltip'
import { Meta, StoryObj } from '@storybook/react'
import { userEvent, within } from '@storybook/test'

type ButtonPropsAndCustomArgs = React.ComponentProps<typeof TooltipButton>

const meta: Meta<ButtonPropsAndCustomArgs> = {
  component: TooltipButton,
  args: {
    asChild: false,
    tooltip: 'Example Button',
    variant: 'ghost',
  },
  argTypes: {
    tooltip: {
      type: 'string',
    },
  },
  render: ({ ...args }) => {
    return (
      <TooltipButton data-testid="test_target" className="text-text" {...args}>
        <Search />
      </TooltipButton>
    )
  },
}

export default meta
type Story = StoryObj<ButtonPropsAndCustomArgs>

export const Default: Story = {
  play: async ({ canvasElement }) => {
    const canvas = within(canvasElement)

    await userEvent.hover(canvas.getByTestId('test_target'))
  },
}
