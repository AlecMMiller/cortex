import { createLazyFileRoute, Link } from '@tanstack/react-router'
import { getAllNotes } from '@/commands/note'
import { NoteTitle } from '@/types'
import { useEffect, useState } from 'react'

export const Route = createLazyFileRoute('/')({
  component: Index,
})

interface RecentNoteProps {
  note: NoteTitle
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
  const [notes, setNotes] = useState<NoteTitle[]>([])

  useEffect(() => {
    const fetchNotes = async () => {
      const notes = await getAllNotes()
      setNotes(notes)
    }

    fetchNotes()
  }, [])

  const noteElements = notes.map((note) => (
    <RecentNote key={note.uuid} note={note} />
  ))

  return (
    <div className="p-8 flex flex-col">
      <h1 className="text-text">Recent Notes</h1>
      {noteElements}
    </div>
  )
}
