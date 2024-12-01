import { Result } from '@/bindings'
import {
  UseQueryResult,
  QueryClient,
  useQuery,
  useSuspenseQuery,
  UseSuspenseQueryResult,
} from '@tanstack/react-query'

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

type UseTypeSuspenseFunction<Args extends Array<any>, R> = (
  options: QueryOptions,
  ...args: Args
) => UseSuspenseQueryResult<R>

type EnsureTypeData<Args extends Array<any>, R> = (
  client: QueryClient,
  options: QueryOptions,
  ...args: Args
) => Promise<R>

interface QueryMethods<Args extends Array<any>, R> {
  useType: UseTypeFunction<Args, R>
  useTypeSuspense: UseTypeSuspenseFunction<Args, R>
  prefetchType: PrefetchTypeFunction<Args>
  ensureTypeData: EnsureTypeData<Args, R>
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

  const useTypeSuspense: UseTypeSuspenseFunction<Args, R> = (
    options: QueryOptions,
    ...args: Args
  ): UseSuspenseQueryResult<R> => {
    return useSuspenseQuery({
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

  const ensureTypeData: EnsureTypeData<Args, R> = (
    client: QueryClient,
    options: QueryOptions,
    ...args: Args
  ): Promise<R> => {
    return client.ensureQueryData({
      queryKey: makeKey(...args),
      queryFn: buildGetType(...args),
      ...options,
    })
  }

  return { useType, prefetchType, ensureTypeData, useTypeSuspense }
}
