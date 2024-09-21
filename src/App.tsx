import { useEffect, useState } from 'react'
import { createNote, getAllNotes, getLastUpdated } from './commands/note'
import Editor from './editor'
import { NoteData } from './types'
import {
  Maximize,
  Minimize2,
  Search,
  Settings,
  X,
  FilePlus2,
  Icon
} from 'lucide-react'
import { getCurrentWindow } from '@tauri-apps/api/window'

import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
  TooltipProvider
} from '@/components/ui/tooltip'

interface SideButtonProps {
  icon: typeof Icon
  tooltip: string
  onClick?: () => void
}

function SideButton (props: SideButtonProps): JSX.Element {
  const Actual = props.icon
  return (
    <TooltipProvider>
      <Tooltip>
        <TooltipTrigger>
          <Actual onClick={props.onClick} className='m-2 text-subtext1 hover:text-text' size={24} />
        </TooltipTrigger>
        <TooltipContent side='right'>{props.tooltip}</TooltipContent>
      </Tooltip>
    </TooltipProvider>
  )
}

function App (): JSX.Element {
  const [note, setNote] = useState<NoteData | null>(null)

  useEffect(() => {
    async function loadLatest (): Promise<void> {
      const result = await getLastUpdated()
      setNote(result)
      const all = await getAllNotes()
      console.log(all)
    }
    void loadLatest()
  }, [])

  const content =
    note !== null ? <Editor note={note} /> : <div>Loading...</div>
  const appWindow = getCurrentWindow()

  const [isFullScreen, setIsFullScreen] = useState(false)

  const checkIsFullScreen = async (): Promise<void> => {
    const state = await appWindow.isMaximized()
    setIsFullScreen(state)
  }

  useEffect(() => {
    void checkIsFullScreen()
  }, [appWindow])

  const rounded = isFullScreen ? '' : 'rounded-md'

  return (
    <div
      className={`bg-crust text-3xl w-screen h-screen flex flex-col ${rounded}`}
    >
      <div className='flex text-text p-1 gap-3'>
        <div data-tauri-drag-region className='grow' />
        <Minimize2 onClick={() => { void appWindow.minimize() }} size={16} />
        <Maximize
          onClick={() => {
            setIsFullScreen(!isFullScreen)
            void appWindow.toggleMaximize()
            void checkIsFullScreen()
          }}
          size={16}
        />
        <X onClick={() => { void appWindow.close() }} size={16} />
      </div>
      <div className='flex flex-row justify-between flex-1 min-h-0'>
        <div className='flex flex-col'>
          <SideButton icon={Search} tooltip='Search' />
          <SideButton onClick={() => {void createNote('Unnamed Note')}} icon={FilePlus2} tooltip='New Note' />
          <div className='grow' />
          <SideButton icon={Settings} tooltip='Settings' />
        </div>
        <div className='bg-base w-full flex flex-row rounded-tl-md min-h-0'>
          {content}
        </div>
      </div>
    </div>
  )
}

export default App
