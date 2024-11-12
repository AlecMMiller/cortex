import { useQueryClient } from '@tanstack/react-query'
import { buildPrefetchNote } from '@/commands/note'
import { Link } from '@tanstack/react-router'
import { NoteTitle } from '@/bindings'

interface NoteLinkProps {
  readonly note: NoteTitle
  readonly onClick?: () => void
  readonly className?: string
}

export function NoteLink(props: NoteLinkProps): JSX.Element {
  const note = props.note
  const client = useQueryClient()
  const prefetch = buildPrefetchNote(client, note.uuid)
  const className = 'text-blue ' + props.className
  return (
    <Link
      onClick={props.onClick}
      to={`/notes/${note.uuid}`}
      className={className}
      onFocus={prefetch}
      onMouseEnter={prefetch}
    >
      {note.title}
    </Link>
  )
}
