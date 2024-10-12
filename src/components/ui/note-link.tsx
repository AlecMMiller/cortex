import { NoteTitle } from '@/types'
import { useQueryClient } from '@tanstack/react-query'
import { buildPrefetchNote } from '@/commands/note'
import { Link } from '@tanstack/react-router'

interface NoteLinkProps {
  readonly note: NoteTitle
  readonly onClick?: () => void
}

export function NoteLink(props: NoteLinkProps): JSX.Element {
  const note = props.note
  const client = useQueryClient()
  const prefetch = buildPrefetchNote(client, { uuid: note.uuid })
  return (
    <Link
      onClick={props.onClick}
      to={`/notes/${note.uuid}`}
      className="text-blue"
      onFocus={prefetch}
      onMouseEnter={prefetch}
    >
      {note.title}
    </Link>
  )
}
