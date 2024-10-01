import { NavButton } from '@/components/ui/nav-button'
import { Search, FilePlus2, Settings, House } from 'lucide-react'
import { createNote } from '@/commands/note'
import { Dialog, DialogContent, DialogHeader, DialogTitle } from '../ui/dialog'
import { Input } from '../ui/input'
import { useForm } from 'react-hook-form'
import { Button } from '../ui/button'
import { Form, FormControl, FormField, FormItem, FormLabel } from '../ui/form'
import { z } from 'zod'
import { zodResolver } from '@hookform/resolvers/zod'
import { useTranslation } from 'react-i18next'
import { useState } from 'react'
import { useNavigate } from '@tanstack/react-router'
const noteCreateSchema = z.object({
  title: z
    .string()
    .min(2, { message: 'Title must be at least 2 characters' })
    .max(50),
})

interface CreateNoteDialogProps {
  setOpen: (open: boolean) => void
}

function CreateNoteDialog(props: CreateNoteDialogProps) {
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

export function Sidebar(): JSX.Element {
  const [open, setOpen] = useState(false)

  return (
    <div className="flex flex-col">
      <NavButton icon={House} tooltip="Home" to="/" />
      <NavButton icon={Search} tooltip="Search" />
      <Dialog open={open} onOpenChange={setOpen}>
        <NavButton isDialog icon={FilePlus2} tooltip="New Note" />
        <CreateNoteDialog setOpen={setOpen} />
      </Dialog>
      <div className="grow" />
      <NavButton icon={Settings} tooltip="Settings" />
    </div>
  )
}
