import { commands, Schema } from '@/bindings'
import { buildQueryMethods } from './common'
import { QueryClient } from '@tanstack/react-query'

export const {
  useType: useAvailableSchemas,
  buildPrefetchType: buildPretchAvailableSchemas,
} = buildQueryMethods(commands.getAllSchemas, () => ['schemas'])

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
