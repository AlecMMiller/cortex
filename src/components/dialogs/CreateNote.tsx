import { createNote } from '@/commands/note'
import { z } from 'zod'
import { DialogFunctionProps } from '../ui/nav-button'
import { FormFieldInfo, GenericCreateDialog } from './GenericCreate'

const title = z.string().min(2).max(50)

const noteCreateSchema = z.object({
  title,
})

export function CreateNoteDialog(props: DialogFunctionProps): JSX.Element {
  type SchemaType = typeof noteCreateSchema

  const doCreate = async (values: z.infer<SchemaType>) =>
    createNote(values.title)

  const fields: FormFieldInfo<SchemaType>[] = [
    {
      name: 'title',
      label: 'Title',
    },
  ]

  return (
    <GenericCreateDialog
      noun="Note"
      schema={noteCreateSchema}
      baseNavigate="/notes"
      createCb={doCreate}
      fields={fields}
      {...props}
    />
  )
}
