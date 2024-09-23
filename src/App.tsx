import { useEffect, useState } from 'react'
import { getAllNotes, getLastUpdated } from './commands/note'
import Editor from './editor'
import { NoteData } from './types'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { Sidebar } from './components/widgets/Sidebar'
import { Topbar } from './components/widgets/Topbar'

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
      <Topbar
        close={() => appWindow.close()}
        toggleMaximize={() => {
          setIsFullScreen(!isFullScreen)
          appWindow.toggleMaximize()
          checkIsFullScreen()
        }}
        minimize={() => {
          appWindow.minimize()
        }}
      />
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
