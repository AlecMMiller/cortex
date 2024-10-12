import { invoke } from '@tauri-apps/api/core'
import { NoteData, NoteTitle } from '../types'
import { buildQueryMethods } from './common'

type NoteSelect = {
  uuid: string
}

export function makeNoteQueryKey(uuid: string) {
  return ['note', uuid]
}

export const { useType: useNote, buildPrefetchType: buildPrefetchNote } =
  buildQueryMethods<NoteSelect, NoteData>({
    command: 'get_note',
    makeKey: (data: NoteSelect) => {
      return makeNoteQueryKey(data.uuid)
    },
  })

export const {
  useType: useAllNotes,
  buildPrefetchType: buildPrefetchAllNotes,
} = buildQueryMethods<{}, NoteTitle[]>({
  command: 'get_all_notes',
  makeKey: (_data: {}) => {
    return ['note_titles']
  },
})

type TitleSearch = {
  title: string
  maxResults: number
}

export const {
  useType: useSearchNotesByTitle,
  buildPrefetchType: buildPretchNotesByTitle,
} = buildQueryMethods<TitleSearch, NoteTitle[]>({
  command: 'get_notes_by_title',
  makeKey: (data: TitleSearch) => {
    return ['notes', 'by_title', data.title]
  },
})

type ContentSearch = {
  content: string
  maxResults: number
}

export const {
  useType: useSearchNotesByContent,
  buildPrefetchType: buildPretchNotesByContent,
} = buildQueryMethods<ContentSearch, NoteTitle[]>({
  command: 'get_notes_by_content',
  makeKey: (data: ContentSearch) => {
    return ['notes', 'by_content', data.content]
  },
})

export async function createNote(name: string): Promise<NoteData> {
  const result = invoke('create_note', { title: name })
  return (await result) as NoteData
}

export async function renameNote(uuid: string, title: string): Promise<void> {
  await invoke('rename_note', { uuid, title })
}
