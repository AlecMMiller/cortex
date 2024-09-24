import { createFileRoute } from '@tanstack/react-router'
import { useEffect, useState } from 'react'
import { NoteData } from '@/types'
import { getNote } from '@/commands/note'
import Editor from '@/editor'

export const Route = createFileRoute('/notes/$noteId')({
  component: NoteComponent,
})

function NoteComponent(): JSX.Element {
  const { noteId } = Route.useParams()

  const [note, setNote] = useState<NoteData | null>(null)
  useEffect(() => {
    const doGet = async () => {
      const note = await getNote(noteId)
      setNote(note)
    }

    doGet()
  }, [])

  if (note === null) return <></>

  return <Editor note={note} />
}
