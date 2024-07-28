import { useEffect, useState } from "react";
import { getLastUpdated } from "./commands/note";
import Editor from './editor';
import { NoteData } from "./types";

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

  return (
    <div className="bg-background-base-default text-3xl w-screen h-screen flex flex-row justify-between">
      {content}
    </div>
  );
}

export default App;
