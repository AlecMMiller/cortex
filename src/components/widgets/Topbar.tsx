import { Minimize2, Maximize, X, LucideIcon } from 'lucide-react'

interface TopButtonProps {
  readonly onClick: () => void
  readonly icon: LucideIcon
}

function TopButton(props: TopButtonProps) {
  const { onClick, icon: Icon } = props
  return <Icon onClick={onClick} size={16} />
}

interface TopbarProps {
  readonly handleClose: () => void
  readonly handleMinimize: () => void
  readonly handleMaximize: () => void
}

export function Topbar(props: TopbarProps): JSX.Element {
  return (
    <div className="flex text-text p-1 gap-3">
      <div data-tauri-drag-region className="grow" />
      <TopButton onClick={props.handleMinimize} icon={Minimize2} />
      <TopButton onClick={props.handleMaximize} icon={Maximize} />
      <TopButton onClick={props.handleClose} icon={X} />
    </div>
  )
}
