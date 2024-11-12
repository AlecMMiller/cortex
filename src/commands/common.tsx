import { Error, Result } from '@/bindings'
import { UseQueryResult, QueryClient, useQuery } from '@tanstack/react-query'

interface QueryOptions {
  readonly staleTime?: number
}

type PrefetchTypeFunction = () => void

type FunctionType<A extends Array<any>, T> = (
  ...args: A
) => Promise<Result<T, Error>>

type UseTypeFunction<Args extends Array<any>, R> = (
  options: QueryOptions,
  ...args: Args
) => UseQueryResult<R>

type BuildPrefetchTypeFunction<Args extends Array<any>> = (
  client: QueryClient,
  ...args: Args
) => PrefetchTypeFunction

interface QueryMethods<Args extends Array<any>, R> {
  useType: UseTypeFunction<Args, R>
  buildPrefetchType: BuildPrefetchTypeFunction<Args>
}

type KeyFunction<Args extends Array<any>> = (...args: Args) => string[]

export function buildQueryMethods<Args extends Array<any>, R>(
  baseFunction: FunctionType<Args, R>,
  makeKey: KeyFunction<Args>,
): QueryMethods<Args, R> {
  const buildGetType = (...args: Args): (() => Promise<R>) => {
    return async (): Promise<R> => {
      const result = await baseFunction(...args)
      if (result.status === 'ok') {
        return result.data
      } else {
        throw result.error
      }
    }
  }

  const useType: UseTypeFunction<Args, R> = (
    options: QueryOptions,
    ...args: Args
  ): UseQueryResult<R> => {
    return useQuery({
      queryKey: makeKey(...args),
      queryFn: buildGetType(...args),
      ...options,
    })
  }

  const buildPrefetchType: BuildPrefetchTypeFunction<Args> = (
    client: QueryClient,
    ...args: Args
  ): (() => void) => {
    return () => {
      client.prefetchQuery({
        queryKey: makeKey(...args),
        queryFn: buildGetType(...args),
        staleTime: 10000,
      })
    }
  }

  return { useType, buildPrefetchType }
}
