import { Link } from '@tanstack/react-router'
import { ButtonProps, Button } from './button'
import {
  TooltipProvider,
  Tooltip,
  TooltipTrigger,
  TooltipContent,
} from './tooltip'
type Side = 'left' | 'right' | 'top' | 'bottom'

export interface TooltipButtonProps extends ButtonProps {
  readonly side?: Side
  readonly to?: string
  readonly tooltip?: string
}

export function TooltipButton(props: TooltipButtonProps): JSX.Element {
  const { side, tooltip, to, ...rest } = props

  const element =
    to === undefined ? (
      <Button {...rest} />
    ) : (
      <Link className={props.className} to={to}>
        {props.children}
      </Link>
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
