import { invoke } from '@tauri-apps/api/core'
import { NoteData, NoteTitle } from '../types'
import { useQuery, UseQueryResult } from '@tanstack/react-query'

interface QueryOptions {
  readonly staleTime?: number
}

export async function getLastUpdated(): Promise<NoteData | null> {
  try {
    const result = await invoke('get_last_updated')
    return JSON.parse(result as string) as NoteData
  } catch {
    return null
  }
}

export function useNote(
  uuid: string,
  options: QueryOptions,
): UseQueryResult<NoteData> {
  return useQuery({
    queryKey: ['note', uuid],
    queryFn: async () => {
      const result = await invoke('get_note', { uuid })
      return JSON.parse(result as string) as NoteData
    },
    ...options,
  })
}

export function useAllNotes(
  options: QueryOptions,
): UseQueryResult<NoteTitle[]> {
  return useQuery({
    queryKey: ['noteTitles'],
    queryFn: async () => {
      const result = await invoke('get_notes')
      return JSON.parse(result as string) as NoteTitle[]
    },
    ...options,
  })
}

export async function createNote(name: string): Promise<string> {
  const result = invoke('create_note', { title: name })
  return await (result as Promise<string>)
}

export async function renameNote(uuid: string, title: string): Promise<void> {
  await invoke('rename_note', { uuid, title })
}
