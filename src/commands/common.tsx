import { UseQueryResult, QueryClient, useQuery } from '@tanstack/react-query'
import { invoke, InvokeArgs } from '@tauri-apps/api/core'

interface QueryOptions {
  readonly staleTime?: number
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

export function buildQueryMethods<InputType extends InvokeArgs, ReturnType>(
  args: BuilderArgs<InputType>,
): QueryMethods<InputType, ReturnType> {
  const buildGetType = (data: InputType): (() => Promise<ReturnType>) => {
    return async (): Promise<ReturnType> => {
      console.debug(`${args.command} ${JSON.stringify(data)}`)
      const result = await invoke(args.command, data)
      return result as ReturnType
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
