import { z } from 'zod'
import { DialogFunctionProps } from '../ui/nav-button'
import { createSchema } from '@/commands/objects'
import { useQueryClient } from '@tanstack/react-query'
import { FormFieldInfo, GenericCreateDialog } from './GenericCreate'

const schemaCreateSchema = z.object({
  name: z
    .string()
    .min(2, { message: 'Name must be at least 2 characters' })
    .max(50),
})

export function CreateSchemaDialog(props: DialogFunctionProps): JSX.Element {
  const client = useQueryClient()
  type SchemaType = typeof schemaCreateSchema

  const doCreate = async (values: z.infer<SchemaType>) =>
    createSchema(client, values.name)

  const fields: FormFieldInfo<SchemaType>[] = [
    {
      name: 'name',
      label: 'Name',
    },
  ]

  return (
    <GenericCreateDialog
      noun="Schema"
      schema={schemaCreateSchema}
      baseNavigate="/schemas"
      createCb={doCreate}
      fields={fields}
      {...props}
    />
  )
}
