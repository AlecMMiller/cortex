import { useEffect, useState } from "react";
import { getLastUpdated } from "./commands/note";
import Editor from './editor';
import { NoteData } from "./types";
import { Maximize, Minimize2, Search, Settings, X } from "lucide-react";
import { getCurrentWindow } from '@tauri-apps/api/window';

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
          <Search className="m-2 text-subtext1" size={24} />
          <div className="grow" />
          <Settings className="m-2 text-subtext1" size={24} />
        </div>
        <div className="bg-base w-full flex flex-row rounded-tl-md min-h-0">
          {content}
        </div>
      </div>
    </div>
  );
}

export default App;
