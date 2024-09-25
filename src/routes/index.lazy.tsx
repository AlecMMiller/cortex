import { createLazyFileRoute, Link } from '@tanstack/react-router'
import { useAllNotes } from '@/commands/note'
import { NoteTitle } from '@/types'

export const Route = createLazyFileRoute('/')({
  component: Index,
})

interface RecentNoteProps {
  readonly note: NoteTitle
}

function RecentNote(props: RecentNoteProps): JSX.Element {
  const note = props.note
  return (
    <Link to={`notes/${note.uuid}`} className="text-blue">
      {note.title}
    </Link>
  )
}

function Index(): JSX.Element {
  const { status, data } = useAllNotes({})

  if (status === 'pending' || status === 'error') {
    return <div>Loading</div>
  }

  const noteElements = data.map((note) => (
    <RecentNote key={note.uuid} note={note} />
  ))

  return (
    <div className="p-8 flex flex-col">
      <h1 className="text-text">Recent Notes</h1>
      {noteElements}
    </div>
  )
}
