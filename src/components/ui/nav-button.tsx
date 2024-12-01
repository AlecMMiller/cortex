import { useState } from 'react'
import { TooltipButton } from './button-tooltip'
import { LucideIcon } from 'lucide-react'
import { Dialog } from './dialog'
import { QueryClient } from '@tanstack/react-query'
import { PrefetchTypeFunction } from '@/commands/common'
import { array } from 'zod'

type SetOpen = (open: boolean) => void

export interface DialogFunctionProps {
  readonly queryClient: QueryClient
  readonly setOpen: SetOpen
}

type DialogFunction = (props: DialogFunctionProps) => JSX.Element

interface NavButtonProps<Args extends Array<any> | never[]> {
  readonly icon: LucideIcon
  readonly tooltip: string
  readonly onClick?: () => void
  readonly prefetch?: PrefetchTypeFunction<Args>
  readonly to?: string
  readonly DialogContent?: DialogFunction
  readonly testid?: string
  readonly queryClient: QueryClient
  readonly args?: Args
}

export function NavButton<Args extends Array<any>>(
  props: NavButtonProps<Args>,
): JSX.Element {
  const {
    icon: Icon,
    DialogContent,
    queryClient,
    testid,
    args,
    ...rest
  } = props

  const [open, setOpen] = useState(false)
  const isDialog = DialogContent !== undefined

  const button = (
    <TooltipButton
      client={queryClient}
      isDialog={isDialog}
      size="icon"
      variant="ghost"
      side="right"
      args={args ?? []}
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
