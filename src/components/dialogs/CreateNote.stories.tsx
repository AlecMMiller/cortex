import { Dialog } from '@radix-ui/react-dialog'
import { CreateNoteDialog } from './CreateNote'
import { Meta, StoryObj } from '@storybook/react'

const Component = CreateNoteDialog
type ComponentPropsAndCustomArgs = React.ComponentProps<typeof Component>

const meta: Meta<ComponentPropsAndCustomArgs> = {
  component: Component,
  render: ({ ...args }) => {
    return (
      <Dialog open>
        <CreateNoteDialog {...args} />
      </Dialog>
    )
  },
}

export default meta
type Story = StoryObj<ComponentPropsAndCustomArgs>

export const Default: Story = {}
