import { useEffect, useState } from 'react'
import { createNote, getAllNotes, getLastUpdated } from './commands/note'
import Editor from './editor'
import { NoteData } from './types'
import { Maximize, Minimize2, X } from 'lucide-react'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { Sidebar } from './components/widgets/Sidebar'

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
  const appWindow = getCurrentWindow()

  const [isFullScreen, setIsFullScreen] = useState(false)

  const checkIsFullScreen = async (): Promise<void> => {
    const state = await appWindow.isMaximized()
    setIsFullScreen(state)
  }

  useEffect(() => {
    checkIsFullScreen()
  }, [appWindow])

  const rounded = isFullScreen ? '' : 'rounded-md'

  return (
    <div
      className={`bg-crust text-3xl w-screen h-screen flex flex-col ${rounded}`}
    >
      <div className="flex text-text p-1 gap-3">
        <div data-tauri-drag-region className="grow" />
        <Minimize2
          onClick={() => {
            appWindow.minimize()
          }}
          size={16}
        />
        <Maximize
          onClick={() => {
            setIsFullScreen(!isFullScreen)
            appWindow.toggleMaximize()
            checkIsFullScreen()
          }}
          size={16}
        />
        <X
          onClick={() => {
            appWindow.close()
          }}
          size={16}
        />
      </div>
      <div className="flex flex-row justify-between flex-1 min-h-0">
        <Sidebar />
        <div className="bg-base w-full flex flex-row rounded-tl-md min-h-0">
          {content}
        </div>
      </div>
    </div>
  )
}

export default App
