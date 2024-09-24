import { invoke } from '@tauri-apps/api/core'
import { NoteData, NoteTitle } from '../types'

export async function getLastUpdated(): Promise<NoteData | null> {
  try {
    const result = await invoke('get_last_updated')
    return JSON.parse(result as string) as NoteData
  } catch {
    return null
  }
}

export async function getNote(uuid: string): Promise<NoteData> {
  const result = await invoke('get_note', { uuid })
  return JSON.parse(result as string) as NoteData
}

export async function getAllNotes(): Promise<NoteTitle[]> {
  const result = await invoke('get_notes')
  return JSON.parse(result as string) as NoteTitle[]
}

export async function createNote(name: string): Promise<string> {
  const result = invoke('create_note', { title: name })
  return await (result as Promise<string>)
}

export async function renameNote(uuid: string, title: string): Promise<void> {
  await invoke('rename_note', { uuid, title })
}
