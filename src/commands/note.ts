import { invoke } from "@tauri-apps/api/core"
import { NoteData } from "../types"

export async function getLastUpdated(): Promise<NoteData | null> {
    try {
        const result = await invoke('get_last_updated')
        return JSON.parse(result as string) as NoteData
    }
    catch {
        return null
    }
}

export async function createNote(name: string): Promise<string> {
    const result = invoke('create_note', { name })
    return result as Promise<string>
}

export async function renameNote(uuid: string, title: string): Promise<void> {
    invoke('rename_note', { uuid, title })
}
