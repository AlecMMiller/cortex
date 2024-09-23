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
  readonly tooltip?: string
}

export function TooltipButton(props: TooltipButtonProps): JSX.Element {
  const { side, tooltip, ...rest } = props

  if (tooltip === undefined) return <Button {...rest} />

  return (
    <TooltipProvider>
      <Tooltip>
        <TooltipTrigger asChild>
          <Button {...rest} />
        </TooltipTrigger>
        <TooltipContent side={side}>{tooltip}</TooltipContent>
      </Tooltip>
    </TooltipProvider>
  )
}
