import { TooltipButton } from './button-tooltip'
import { LucideIcon } from 'lucide-react'

interface NavButtonProps {
  icon: LucideIcon
  tooltip: string
  onClick?: () => void
}

export function NavButton (props: NavButtonProps): JSX.Element {
  const { icon: Icon, tooltip, onClick } = props

  return (
    <TooltipButton tooltip={tooltip} onClick={onClick} size='icon' variant='ghost' side='right'>
      <Icon className='m-2 text-subtext1 hover:text-text' size={24} />
    </TooltipButton>
  )
}
