import { Search } from 'lucide-react'
import { Button } from './button'
import { Meta, StoryObj } from '@storybook/react'

enum ChildOptions {
  Text = 'Text',
  Icon = 'Icon',
}

type ButtonPropsAndCustomArgs = React.ComponentProps<typeof Button> & {
  readonly child: ChildOptions
  readonly text?: string
}

const meta: Meta<ButtonPropsAndCustomArgs> = {
  component: Button,
  args: {
    asChild: false,
    variant: 'default',
    size: 'default',
    child: ChildOptions.Text,
    text: 'Do the Thing',
  },
  argTypes: {
    variant: {
      control: 'select',
      options: ['default', 'ghost'],
    },
    size: {
      control: 'select',
      options: ['default', 'sm', 'lg', 'icon', 'fit'],
    },
    asChild: {
      control: 'boolean',
    },
    child: {
      control: 'select',
      options: Object.values(ChildOptions),
    },
  },
  render: ({ child, text, ...args }) => {
    const childElement = child === ChildOptions.Icon ? <Search /> : text

    return (
      <Button className="text-text" {...args}>
        {childElement}
      </Button>
    )
  },
}

export default meta
type Story = StoryObj<ButtonPropsAndCustomArgs>

export const Default: Story = {}

export const Small: Story = {
  args: {
    size: 'sm',
  },
}

export const Large: Story = {
  args: {
    size: 'lg',
  },
}

export const Ghost: Story = {
  args: {
    variant: 'ghost',
    child: ChildOptions.Icon,
  },
}
