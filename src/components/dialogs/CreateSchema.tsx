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
import { createSchema } from '@/commands/objects'
import { QueryClient } from '@tanstack/react-query'

const schemaCreateSchema = z.object({
  name: z
    .string()
    .min(2, { message: 'Name must be at least 2 characters' })
    .max(50),
})

export function CreateSchemaDialog(props: DialogFunctionProps): JSX.Element {
  const form = useForm<z.infer<typeof schemaCreateSchema>>({
    resolver: zodResolver(schemaCreateSchema),
    defaultValues: {
      name: '',
    },
  })

  const { t } = useTranslation()
  const navigate = useNavigate()

  function onSubmit(values: z.infer<typeof schemaCreateSchema>) {
    const doSubmit = async (client: QueryClient, name: string) => {
      const newSchema = await createSchema(client, name)
      navigate({ to: `/objects/${newSchema.uuid}` })
      props.setOpen(false)
    }

    doSubmit(props.queryClient, values.name)
  }

  return (
    <DialogContent>
      <DialogHeader>
        <DialogTitle>{t('New Schema')}</DialogTitle>
      </DialogHeader>
      <Form {...form}>
        <form className="space-y-8 pt-4" onSubmit={form.handleSubmit(onSubmit)}>
          <FormField
            control={form.control}
            name="name"
            render={({ field }) => (
              <FormItem>
                <FormLabel>{t('Name')}</FormLabel>
                <FormControl>
                  <Input placeholder={t('Schema Name')} {...field} />
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
