import { invoke, InvokeArgs } from '@tauri-apps/api/core'
import { NoteData, NoteTitle } from '../types'
import { useQuery, QueryClient, UseQueryResult } from '@tanstack/react-query'

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

interface QueryMethods<InputType, ReturnType> {
  useType: UseTypeFunction<InputType, ReturnType>
  buildPrefetchType: BuildPrefetchTypeFunction<InputType>
}

interface BuilderArgs<InputType> {
  command: string
  makeKey: (data: InputType) => string[]
}

type UseTypeFunction<InputType, ReturnType> = (
  data: InputType,
  options: QueryOptions,
) => UseQueryResult<ReturnType>

type PrefetchTypeFunction = () => void

type BuildPrefetchTypeFunction<InputType> = (
  client: QueryClient,
  data: InputType,
) => PrefetchTypeFunction

function buildQueryMethods<InputType extends InvokeArgs, ReturnType>(
  args: BuilderArgs<InputType>,
): QueryMethods<InputType, ReturnType> {
  const buildGetType = (data: InputType): (() => Promise<ReturnType>) => {
    return async (): Promise<ReturnType> => {
      console.debug(`${args.command} ${JSON.stringify(data)}`)
      const result = await invoke(args.command, data)
      return JSON.parse(result as string) as ReturnType
    }
  }

  const useType: UseTypeFunction<InputType, ReturnType> = (
    data: InputType,
    options: QueryOptions,
  ): UseQueryResult<ReturnType> => {
    return useQuery({
      queryKey: args.makeKey(data),
      queryFn: buildGetType(data),
      ...options,
    })
  }

  const buildPrefetchType: BuildPrefetchTypeFunction<InputType> = (
    client: QueryClient,
    data: InputType,
  ): (() => void) => {
    return () => {
      client.prefetchQuery({
        queryKey: args.makeKey(data),
        queryFn: buildGetType(data),
        staleTime: 10000,
      })
    }
  }

  return { useType, buildPrefetchType }
}

type NoteSelect = {
  uuid: string
}

export const { useType: useNote, buildPrefetchType: buildPrefetchNote } =
  buildQueryMethods<NoteSelect, NoteData>({
    command: 'get_note',
    makeKey: (data: NoteSelect) => {
      return ['note', data.uuid]
    },
  })

export const {
  useType: useAllNotes,
  buildPrefetchType: buildPrefetchAllNotes,
} = buildQueryMethods<{}, NoteTitle[]>({
  command: 'get_notes',
  makeKey: (_data: {}) => {
    return ['note_titles']
  },
})

export async function createNote(name: string): Promise<string> {
  const result = invoke('create_note', { title: name })
  return await (result as Promise<string>)
}

export async function renameNote(uuid: string, title: string): Promise<void> {
  await invoke('rename_note', { uuid, title })
}
