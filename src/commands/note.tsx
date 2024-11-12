import { invoke } from '@tauri-apps/api/core'
import { buildQueryMethods } from './common'
import { QueryClient } from '@tanstack/react-query'
import { commands, Note, NoteTitle } from '@/bindings'

export function makeNoteQueryKey(uuid: string) {
  return ['note', uuid]
}

export const { useType: useNote, buildPrefetchType: buildPrefetchNote } =
  buildQueryMethods(commands.getNote, makeNoteQueryKey)

export const {
  useType: useAllNotes,
  buildPrefetchType: buildPrefetchAllNotes,
} = buildQueryMethods(commands.getAllNotes, () => ['note_titles'])

export const {
  useType: useNoteDirectTags,
  buildPrefetchType: buildPrefetchDirectTags,
} = buildQueryMethods(commands.getDirectTags, (uuid: string) => {
  return ['notes', 'tags', uuid, 'direct']
})

export const {
  useType: useSearchNotesByTitle,
  buildPrefetchType: buildPretchNotesByTitle,
} = buildQueryMethods(commands.getNotesByTitle, (title: string, ..._rest) => {
  return ['notes', 'by_title', title]
})

export interface TitleWithContext {
  title: NoteTitle
  context: string
}

export const {
  useType: useSearchNotesByContent,
  buildPrefetchType: buildPretchNotesByContent,
} = buildQueryMethods(
  commands.getNotesByContent,
  (content: string, ..._rest) => ['notes', 'by_content', content],
)

export async function createNote(name: string): Promise<Note> {
  const result = invoke('create_note', { title: name })
  return (await result) as Note
}

export async function renameNote(uuid: string, title: string): Promise<void> {
  await invoke('rename_note', { uuid, title })
}

export async function addNewTag(
  uuid: string,
  tagText: string,
  queryClient: QueryClient,
): Promise<void> {
  await invoke('add_new_tag', { uuid, tagText })
  queryClient.invalidateQueries({ queryKey: ['tags'] })
  queryClient.invalidateQueries({ queryKey: ['notes', 'tags', uuid] })
}

export async function addTag(
  noteUuid: string,
  tagUuid: string,
  queryClient: QueryClient,
): Promise<void> {
  await invoke('add_tag', { noteUuid, tagUuid })
  queryClient.invalidateQueries({ queryKey: ['notes', 'tags', noteUuid] })
}
