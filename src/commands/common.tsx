import { Error, Result } from '@/bindings'
import { UseQueryResult, QueryClient, useQuery } from '@tanstack/react-query'

interface QueryOptions {
  readonly staleTime?: number
}

type PrefetchTypeFunction = () => void

type FunctionType<A extends Array<any>, T> = (
  ...args: A
) => Promise<Result<T, Error>>

type NewUseTypeFunction<Args extends Array<any>, R> = (
  options: QueryOptions,
  ...args: Args
) => UseQueryResult<R>

type NewBuildPrefetchTypeFunction<Args extends Array<any>> = (
  client: QueryClient,
  ...args: Args
) => PrefetchTypeFunction

interface NewQueryMethods<Args extends Array<any>, R> {
  useType: NewUseTypeFunction<Args, R>
  buildPrefetchType: NewBuildPrefetchTypeFunction<Args>
}

type KeyFunction<Args extends Array<any>> = (...args: Args) => string[]

export function newBuildQueryMethods<Args extends Array<any>, R>(
  baseFunction: FunctionType<Args, R>,
  makeKey: KeyFunction<Args>,
): NewQueryMethods<Args, R> {
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

  const useType: NewUseTypeFunction<Args, R> = (
    options: QueryOptions,
    ...args: Args
  ): UseQueryResult<R> => {
    return useQuery({
      queryKey: makeKey(...args),
      queryFn: buildGetType(...args),
      ...options,
    })
  }

  const buildPrefetchType: NewBuildPrefetchTypeFunction<Args> = (
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
