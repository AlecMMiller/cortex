import { createFileRoute } from '@tanstack/react-router'
import { useNote } from '@/commands/note'
import Editor from '@/editor'

export const Route = createFileRoute('/notes/$noteId')({
  component: NoteComponent,
})

function NoteComponent(): JSX.Element {
  const { noteId } = Route.useParams()

  const { data, status } = useNote(noteId, {})

  if (status === 'error' || status === 'pending') return <></>

  return <Editor note={data} />
}
