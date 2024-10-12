import { createLazyFileRoute } from '@tanstack/react-router'
import { useAllNotes } from '@/commands/note'
import { useTranslation } from 'react-i18next'
import { NoteLink } from '@/components/ui/note-link'

export const Route = createLazyFileRoute('/')({
  component: Index,
})

function Index(): JSX.Element {
  const { status, data } = useAllNotes({}, {})

  const { t } = useTranslation()

  if (status === 'pending' || status === 'error') {
    return <div>Loading</div>
  }

  const noteElements = data.map((note) => (
    <NoteLink key={note.uuid} note={note} />
  ))

  return (
    <div className="p-8 flex flex-col font-prose">
      <h1 className="text-text font-semibold">{t('Recent Notes')}</h1>
      {noteElements}
    </div>
  )
}
