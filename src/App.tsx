import { useEffect, useState } from "react";
import { getLastUpdated } from "./commands/note";
import Editor from './editor';
import { NoteData } from "./types";
import { Maximize, Minimize2, Search, Settings, X, FilePlus2, Icon } from "lucide-react";
import { getCurrentWindow } from '@tauri-apps/api/window';

import {
  Tooltip, TooltipContent, TooltipTrigger, TooltipProvider
} from "@/components/ui/tooltip"

interface SideButtonProps {
  icon: Icon
  tooltip: string
};

function SideButton(props: SideButtonProps): JSX.Element {
  let Actual = props.icon;
  return (
    <TooltipProvider>
      <Tooltip>
        <TooltipTrigger>
          <Actual className="m-2 text-subtext1 hover:text-text" size={24}/>
        </TooltipTrigger>
        <TooltipContent side="right">
          {props.tooltip}
        </TooltipContent>
      </Tooltip>
    </TooltipProvider>
  )
};

function App() {
  const [note, setNote] = useState<NoteData | null>(null)

  useEffect(() => {
    async function loadLatest() {
      const result = await getLastUpdated()
      setNote(result)
    }
    loadLatest()
  }, [])

  let content = note !== null ? <Editor note={note} /> : <div>Loading...</div>
  const appWindow = getCurrentWindow();

  const [isFullScreen, setIsFullScreen] = useState(false);

  const checkIsFullScreen = async () => {
    const state = await appWindow.isMaximized();
    setIsFullScreen(state);
  }

  useEffect(() => {
    checkIsFullScreen();
  }, [appWindow]);

  let rounded = isFullScreen ? '' : 'rounded-md'

  return (
    <div className={`bg-crust text-3xl w-screen h-screen flex flex-col ${rounded}`}>
    <div className="flex text-text p-1 gap-3">
    <div data-tauri-drag-region className="grow" />
    <Minimize2 onClick={() => appWindow.minimize()} size={16} />
    <Maximize onClick={() => {
      setIsFullScreen(!isFullScreen);
      appWindow.toggleMaximize()
      void checkIsFullScreen()
    }} size={16} />
    <X onClick={() => appWindow.close()} size={16} />
    </div>
    <div className="flex flex-row justify-between flex-1 min-h-0">
    <div className="flex flex-col">
    <SideButton icon={Search} tooltip="Search" />
    <SideButton icon={FilePlus2} tooltip="New Note"/>
    <div className="grow" />
    <SideButton icon={Settings} tooltip="Settings"/>
    </div>
    <div className="bg-base w-full flex flex-row rounded-tl-md min-h-0">
    {content}
    </div>
    </div>
    </div>
  );
}

export default App;
