import Editor from '@/editor'
import { NoteData } from '@/types'
import { useEffect, useState } from 'react'
import { getLastUpdated, getAllNotes } from '@/commands/note'

export function TestPage(): JSX.Element {
  const [note, setNote] = useState<NoteData | null>(null)

  useEffect(() => {
    async function loadLatest(): Promise<void> {
      const result = await getLastUpdated()
      setNote(result)
      const all = await getAllNotes()
      console.log(all)
    }
    loadLatest()
  }, [])

  if (note === null) {
    return <div>Loading...</div>
  }

  return <Editor note={note} />
}
