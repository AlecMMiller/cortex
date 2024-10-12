import { createNote } from '@/commands/note'
import { DialogContent, DialogHeader, DialogTitle } from '../ui/dialog'
import { Input } from '../ui/input'
import { useForm } from 'react-hook-form'
import { Button } from '../ui/button'
import { Form, FormControl, FormField, FormItem, FormLabel } from '../ui/form'
import { z } from 'zod'
import { zodResolver } from '@hookform/resolvers/zod'
import { useTranslation } from 'react-i18next'
import { useNavigate } from '@tanstack/react-router'
import { DialogFunctionProps } from '../ui/nav-button'

const noteCreateSchema = z.object({
  title: z
    .string()
    .min(2, { message: 'Title must be at least 2 characters' })
    .max(50),
})

export function CreateNoteDialog(props: DialogFunctionProps): JSX.Element {
  const form = useForm<z.infer<typeof noteCreateSchema>>({
    resolver: zodResolver(noteCreateSchema),
    defaultValues: {
      title: '',
    },
  })

  const { t } = useTranslation()
  const navigate = useNavigate()

  function onSubmit(values: z.infer<typeof noteCreateSchema>) {
    const doSubmit = async (title: string) => {
      const newNote = await createNote(title)
      navigate({ to: `/notes/${newNote.uuid}` })
      props.setOpen(false)
    }

    doSubmit(values.title)
  }

  return (
    <DialogContent>
      <DialogHeader>
        <DialogTitle>{t('New Note')}</DialogTitle>
      </DialogHeader>
      <Form {...form}>
        <form className="space-y-8 pt-4" onSubmit={form.handleSubmit(onSubmit)}>
          <FormField
            control={form.control}
            name="title"
            render={({ field }) => (
              <FormItem>
                <FormLabel>{t('Title')}</FormLabel>
                <FormControl>
                  <Input placeholder={t('Note Title')} {...field} />
                </FormControl>
              </FormItem>
            )}
          />

          <div className="w-full flex flex-row-reverse pt-2">
            <Button className="w-26" type="submit">
              {t('Create')}
            </Button>
          </div>
        </form>
      </Form>
    </DialogContent>
  )
}
