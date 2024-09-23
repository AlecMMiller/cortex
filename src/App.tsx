import { useEffect, useState } from 'react'
import { getAllNotes, getLastUpdated } from './commands/note'
import Editor from './editor'
import { NoteData } from './types'
import { WindowWrapper } from './components/widgets/WindowWrapper'

function App(): JSX.Element {
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

  const content = note !== null ? <Editor note={note} /> : <div>Loading...</div>

  return <WindowWrapper>{content}</WindowWrapper>
}

export default App
