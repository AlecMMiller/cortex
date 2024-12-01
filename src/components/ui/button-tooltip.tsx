import { Link } from '@tanstack/react-router'
import { ButtonProps, Button } from './button'
import {
  TooltipProvider,
  Tooltip,
  TooltipTrigger,
  TooltipContent,
} from './tooltip'
type Side = 'left' | 'right' | 'top' | 'bottom'
import { DialogTrigger } from './dialog'
import { PrefetchTypeFunction } from '@/commands/common'
import { QueryClient } from '@tanstack/react-query'

export interface TooltipButtonProps<Args extends Array<any>>
  extends ButtonProps {
  readonly side?: Side
  readonly to?: string
  readonly prefetch?: PrefetchTypeFunction<Args>
  readonly tooltip?: string
  readonly isDialog?: boolean
  readonly client: QueryClient
  readonly args: Args
}

export function TooltipButton<Args extends Array<any> | never[]>(
  props: TooltipButtonProps<Args>,
): JSX.Element {
  const { side, tooltip, to, prefetch, isDialog, client, args, ...rest } = props

  const doPrefetch = prefetch
    ? () => {
        prefetch(client, ...args)
      }
    : undefined

  const element = (to !== undefined && (
    <Link
      onMouseEnter={doPrefetch}
      onFocus={doPrefetch}
      className={props.className}
      to={to}
    >
      {props.children}
    </Link>
  )) ||
    (isDialog === true && <DialogTrigger>{props.children}</DialogTrigger>) || (
      <Button onMouseEnter={doPrefetch} onFocus={doPrefetch} {...rest} />
    )

  if (tooltip === undefined) return element

  return (
    <TooltipProvider>
      <Tooltip>
        <TooltipTrigger asChild>{element}</TooltipTrigger>
        <TooltipContent side={side}>{tooltip}</TooltipContent>
      </Tooltip>
    </TooltipProvider>
  )
}
