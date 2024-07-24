import { invoke } from "@tauri-apps/api/core"

export async function getLastUpdated(): Promise<string | null> {
    try {
        const result = await invoke('get_last_updated')
        return result as string
    }
    catch {
        return null
    }
}

export async function createNote(name: string): Promise<string> {
    const result = invoke('create_note', { name })
    return result as Promise<string>
}