import { NavButton } from '@/components/ui/nav-button'
import { Search, FilePlus2, Settings, House, Sprout } from 'lucide-react'
import { CreateNoteDialog } from '../dialogs/CreateNote'
import { SearchDialog } from '../dialogs/Search'

export function Sidebar(): JSX.Element {
  return (
    <div className="flex flex-col h-full">
      <NavButton testid="home" icon={House} tooltip="Home" to="/" />
      <NavButton icon={Search} tooltip="Search" DialogContent={SearchDialog} />
      <NavButton
        icon={FilePlus2}
        tooltip="New Note"
        DialogContent={CreateNoteDialog}
      />
      <NavButton icon={Sprout} tooltip="Objects" to="/objects" />
      <div className="grow" />
      <NavButton icon={Settings} tooltip="Settings" to="/settings" />
    </div>
  )
}
