import { createLazyFileRoute, Link } from '@tanstack/react-router'
import { buildPrefetchNote, useAllNotes } from '@/commands/note'
import { NoteTitle } from '@/types'
import { useQueryClient } from '@tanstack/react-query'

export const Route = createLazyFileRoute('/')({
  component: Index,
})

interface RecentNoteProps {
  readonly note: NoteTitle
}

function RecentNote(props: RecentNoteProps): JSX.Element {
  const note = props.note
  const client = useQueryClient()
  const prefetch = buildPrefetchNote(client, { uuid: note.uuid })
  return (
    <Link
      to={`notes/${note.uuid}`}
      className="text-blue"
      onFocus={prefetch}
      onMouseEnter={prefetch}
    >
      {note.title}
    </Link>
  )
}

function Index(): JSX.Element {
  const { status, data } = useAllNotes({}, {})

  console.log(status)
  console.log(data)

  if (status === 'pending' || status === 'error') {
    return <div>Loading</div>
  }

  const noteElements = data.map((note) => (
    <RecentNote key={note.uuid} note={note} />
  ))

  return (
    <div className="p-8 flex flex-col font-prose">
      <h1 className="text-text font-semibold">Recent Notes</h1>
      {noteElements}
    </div>
  )
}
