import { buildQueryMethods } from './common'
import { QueryClient } from '@tanstack/react-query'
import { commands, Note, NoteTitle } from '@/bindings'

export function makeNoteQueryKey(uuid: string) {
  return ['note', uuid]
}

export const { useType: useNote, prefetchType: prefetchNote } =
  buildQueryMethods(commands.getNote, makeNoteQueryKey)

export const { useType: useAllNotes, prefetchType: prefetchAllNotes } =
  buildQueryMethods(commands.getAllNotes, () => ['note_titles'])

export const { useType: useNoteDirectTags, prefetchType: prefetchDirectTags } =
  buildQueryMethods(commands.getDirectTags, (uuid: string) => {
    return ['notes', 'tags', uuid, 'direct']
  })

export const {
  useType: useSearchNotesByTitle,
  prefetchType: pretchNotesByTitle,
} = buildQueryMethods(commands.getNotesByTitle, (title: string, ..._rest) => {
  return ['notes', 'by_title', title]
})

export interface TitleWithContext {
  title: NoteTitle
  context: string
}

export const {
  useType: useSearchNotesByContent,
  prefetchType: pretchNotesByContent,
} = buildQueryMethods(
  commands.getNotesByContent,
  (content: string, ..._rest) => ['notes', 'by_content', content],
)

export async function createNote(name: string): Promise<Note> {
  const result = await commands.createNote(name)

  if (result.status === 'ok') {
    return result.data
  } else {
    throw new Error(result.error.type)
  }
}

export async function renameNote(uuid: string, title: string): Promise<void> {
  await commands.renameNote(uuid, title)
}

export async function addNewTag(
  uuid: string,
  tagText: string,
  queryClient: QueryClient,
): Promise<void> {
  const result = await commands.addNewTag(uuid, tagText)

  if (result.status === 'error') {
    throw new Error(result.error.type)
  }

  queryClient.invalidateQueries({ queryKey: ['tags'] })
  queryClient.invalidateQueries({ queryKey: ['notes', 'tags', uuid] })
}

export async function addTag(
  noteUuid: string,
  tagUuid: string,
  queryClient: QueryClient,
): Promise<void> {
  const result = await commands.addTag(noteUuid, tagUuid)

  if (result.status === 'error') {
    throw new Error(result.error.type)
  }

  queryClient.invalidateQueries({ queryKey: ['notes', 'tags', noteUuid] })
}
