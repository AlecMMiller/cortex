import { Result } from '@/bindings'
import { UseQueryResult, QueryClient, useQuery } from '@tanstack/react-query'

interface QueryOptions {
  readonly staleTime?: number
}

export type PrefetchTypeFunction<Args extends Array<any>> = (
  client: QueryClient,
  ...args: Args
) => void

type FunctionType<A extends Array<any>, R, E> = (
  ...args: A
) => Promise<Result<R, E>>

type UseTypeFunction<Args extends Array<any>, R> = (
  options: QueryOptions,
  ...args: Args
) => UseQueryResult<R>

interface QueryMethods<Args extends Array<any>, R> {
  useType: UseTypeFunction<Args, R>
  prefetchType: PrefetchTypeFunction<Args>
}

type KeyFunction<Args extends Array<any>> = (...args: Args) => string[]

export function buildQueryMethods<Args extends Array<any>, R, E>(
  baseFunction: FunctionType<Args, R, E>,
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

  const prefetchType: PrefetchTypeFunction<Args> = (
    client: QueryClient,
    ...args: Args
  ) => {
    client.prefetchQuery({
      queryKey: makeKey(...args),
      queryFn: buildGetType(...args),
      staleTime: 10000,
    })
  }

  return { useType, prefetchType }
}
