import { TooltipButton } from './button-tooltip'
import { LucideIcon } from 'lucide-react'

interface NavButtonProps {
  readonly icon: LucideIcon
  readonly tooltip: string
  readonly onClick?: () => void
  readonly prefetch?: () => void
  readonly to?: string
  readonly isDialog?: boolean
}

export function NavButton(props: NavButtonProps): JSX.Element {
  const { icon: Icon, ...rest } = props

  return (
    <TooltipButton
      isDialog={props.isDialog}
      size="icon"
      variant="ghost"
      side="right"
      {...rest}
    >
      <Icon className="m-2 text-subtext1 hover:text-text" size={24} />
    </TooltipButton>
  )
}
