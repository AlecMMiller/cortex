import { useState } from 'react'
import { TooltipButton } from './button-tooltip'
import { LucideIcon } from 'lucide-react'
import { Dialog } from './dialog'
import { QueryClient } from '@tanstack/react-query'

type SetOpen = (open: boolean) => void

export interface DialogFunctionProps {
  readonly queryClient: QueryClient
  readonly setOpen: SetOpen
}

type DialogFunction = (props: DialogFunctionProps) => JSX.Element

interface NavButtonProps {
  readonly icon: LucideIcon
  readonly tooltip: string
  readonly onClick?: () => void
  readonly prefetch?: () => void
  readonly to?: string
  readonly DialogContent?: DialogFunction
  readonly testid?: string
  readonly queryClient: QueryClient
}

export function NavButton(props: NavButtonProps): JSX.Element {
  const { icon: Icon, DialogContent, queryClient, testid, ...rest } = props

  const [open, setOpen] = useState(false)
  const isDialog = DialogContent !== undefined

  const button = (
    <TooltipButton
      isDialog={isDialog}
      size="icon"
      variant="ghost"
      side="right"
      {...rest}
    >
      <Icon
        data-testid={props.testid}
        className="m-2 text-subtext1 hover:text-text"
        size={24}
      />
    </TooltipButton>
  )

  if (DialogContent !== undefined) {
    return (
      <Dialog open={open} onOpenChange={setOpen}>
        {button}
        <DialogContent queryClient={queryClient} setOpen={setOpen} />
      </Dialog>
    )
  }

  return button
}
