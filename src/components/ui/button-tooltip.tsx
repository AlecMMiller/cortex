import { Link } from '@tanstack/react-router'
import { ButtonProps, Button } from './button'
import {
  TooltipProvider,
  Tooltip,
  TooltipTrigger,
  TooltipContent,
} from './tooltip'
import { useTranslation } from 'react-i18next'
type Side = 'left' | 'right' | 'top' | 'bottom'
import { DialogTrigger } from './dialog'

export interface TooltipButtonProps extends ButtonProps {
  readonly side?: Side
  readonly to?: string
  readonly prefetch?: () => void
  readonly tooltip?: string
  readonly isDialog?: boolean
}

export function TooltipButton(props: TooltipButtonProps): JSX.Element {
  const { side, tooltip, to, prefetch, isDialog, ...rest } = props

  const element = (to !== undefined && (
    <Link
      onMouseEnter={prefetch}
      onFocus={prefetch}
      className={props.className}
      to={to}
    >
      {props.children}
    </Link>
  )) ||
    (isDialog === true && <DialogTrigger>{props.children}</DialogTrigger>) || (
      <Button onMouseEnter={prefetch} onFocus={prefetch} {...rest} />
    )

  if (tooltip === undefined) return element

  const { t } = useTranslation()

  return (
    <TooltipProvider>
      <Tooltip>
        <TooltipTrigger asChild>{element}</TooltipTrigger>
        <TooltipContent side={side}>{t(tooltip)}</TooltipContent>
      </Tooltip>
    </TooltipProvider>
  )
}
