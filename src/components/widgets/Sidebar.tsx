import { NavButton } from '@/components/ui/nav-button'
import { Search, FilePlus2, Settings, House, Sprout } from 'lucide-react'
import { CreateNoteDialog } from '../dialogs/CreateNote'
import { SearchDialog } from '../dialogs/Search'
import { useQueryClient } from '@tanstack/react-query'
import { useTranslation } from 'react-i18next'
import { pretchAvailableSchemas } from '@/commands/objects'

export function Sidebar(): JSX.Element {
  const queryClient = useQueryClient()
  const { t } = useTranslation()

  return (
    <div className="flex flex-col h-full">
      <NavButton
        queryClient={queryClient}
        testid="home"
        icon={House}
        tooltip={t('Home')}
        to="/"
      />
      <NavButton
        queryClient={queryClient}
        icon={Search}
        tooltip={t('Search')}
        DialogContent={SearchDialog}
      />
      <NavButton
        queryClient={queryClient}
        icon={FilePlus2}
        tooltip={t('new_noun', { noun: t('Note', { count: 1 }) })}
        DialogContent={CreateNoteDialog}
      />
      <NavButton
        queryClient={queryClient}
        icon={Sprout}
        tooltip={t('Schema', { count: 100 })}
        to="/schemas"
        prefetch={pretchAvailableSchemas}
      />
      <div className="grow" />
      <NavButton
        queryClient={queryClient}
        icon={Settings}
        tooltip={t('Settings')}
        to="/settings"
      />
    </div>
  )
}
