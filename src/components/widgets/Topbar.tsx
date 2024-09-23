import { Minimize2, Maximize, X, LucideIcon } from 'lucide-react'

interface TopButtonProps {
  onClick: () => void
  icon: LucideIcon
}

function TopButton(props: TopButtonProps) {
  const { onClick, icon: Icon } = props
  return <Icon onClick={onClick} size={16} />
}

interface TopbarProps {
  close: () => void
  minimize: () => void
  toggleMaximize: () => void
}

export function Topbar(props: TopbarProps): JSX.Element {
  return (
    <div className="flex text-text p-1 gap-3">
      <div data-tauri-drag-region className="grow" />
      <TopButton onClick={props.close} icon={Minimize2} />
      <TopButton onClick={props.toggleMaximize} icon={Maximize} />
      <TopButton onClick={props.close} icon={X} />
    </div>
  )
}
