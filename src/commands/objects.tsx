import { commands, Schema } from '@/bindings'
import { buildQueryMethods } from './common'
import { QueryClient } from '@tanstack/react-query'

export const {
  useType: useAvailableSchemas,
  prefetchType: pretchAvailableSchemas,
} = buildQueryMethods(commands.getAllSchemas, () => ['schemas'])

export const {
  useType: useSchema,
  prefetchType: pretchSchema,
  useTypeSuspense: useSchemaSuspense,
  ensureTypeData: ensureSchema,
} = buildQueryMethods(commands.getSchema, (uuid: string) => ['schema', uuid])

export async function createSchema(
  queryClient: QueryClient,
  name: string,
): Promise<Schema> {
  const result = await commands.createSchema(name)

  if (result.status !== 'ok') throw new Error(result.error.type)

  const new_schema = result.data

  queryClient.invalidateQueries({ queryKey: ['schemas'] })

  return new_schema
}

export async function renameSchema(
  queryClient: QueryClient,
  uuid: string,
  name: string,
): Promise<void> {
  const result = await commands.renameSchema(uuid, name)

  if (result.status !== 'ok') throw new Error(result.error.type)

  queryClient.invalidateQueries({ queryKey: ['schemas'] })
  queryClient.invalidateQueries({ queryKey: ['schema', uuid] })
}
