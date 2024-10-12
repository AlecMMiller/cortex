import { NavButton } from '@/components/ui/nav-button'
import { Search, FilePlus2, Settings, House } from 'lucide-react'
import { CreateNoteDialog } from '../dialogs/CreateNote'
import { SearchDialog } from '../dialogs/Search'

export function Sidebar(): JSX.Element {
  return (
    <div className="flex flex-col">
      <NavButton icon={House} tooltip="Home" to="/" />
      <NavButton icon={Search} tooltip="Search" DialogContent={SearchDialog} />
      <NavButton
        icon={FilePlus2}
        tooltip="New Note"
        DialogContent={CreateNoteDialog}
      />
      <div className="grow" />
      <NavButton icon={Settings} tooltip="Settings" to="/settings" />
    </div>
  )
}
