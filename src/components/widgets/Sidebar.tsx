import { NavButton } from '@/components/ui/nav-button'
import { Search, FilePlus2, Settings, House } from 'lucide-react'
import { createNote } from '@/commands/note'

export function Sidebar(): JSX.Element {
  return (
    <div className="flex flex-col">
      <NavButton icon={House} tooltip="Home" to="/" />
      <NavButton icon={Search} tooltip="Search" />
      <NavButton
        onClick={() => {
          createNote('Unnamed Note')
        }}
        icon={FilePlus2}
        tooltip="New Note"
      />
      <div className="grow" />
      <NavButton icon={Settings} tooltip="Settings" />
    </div>
  )
}
