import { FieldValue, FieldValues, Control, useForm } from 'react-hook-form'
import { TFunction } from 'i18next'
import { FormField, FormItem, FormLabel, FormControl, Form } from '../ui/form'
import { Input } from '../ui/input'
import { z, ZodType } from 'zod'
import { DialogFunctionProps } from '../ui/nav-button'
import { zodResolver } from '@hookform/resolvers/zod'
import { useTranslation } from 'react-i18next'
import { useNavigate } from '@tanstack/react-router'
import { DialogContent, DialogHeader, DialogTitle } from '../ui/dialog'
import { Button } from '../ui/button'

interface GenericCreateDialogProps<
  CreatedEntity extends Entity,
  SchemaType extends ZodType<any, any, any>,
> extends DialogFunctionProps {
  noun: string
  baseNavigate: string
  schema: SchemaType
  createCb: CreationFunction<CreatedEntity, z.infer<SchemaType>>
  fields: FormFieldInfo<SchemaType>[]
}

export interface FormFieldInfo<FormValues extends FieldValues> {
  name: FieldValue<FormValues>
  label: string
}

export function GenericCreateDialog<
  CreatedEntity extends Entity,
  SchemaType extends ZodType<any, any, any>,
>(props: GenericCreateDialogProps<CreatedEntity, SchemaType>): JSX.Element {
  const form = useForm({
    resolver: zodResolver(props.schema),
  })

  const { t } = useTranslation()
  const navigate = useNavigate()

  function onSubmit(values: z.infer<SchemaType>) {
    const doSubmit = async () => {
      const created = await props.createCb(values)
      navigate({ to: `${props.baseNavigate}/${created.uuid}` })
      props.setOpen(false)
    }

    doSubmit()
  }

  const noun = t(props.noun, { count: 1 })

  const fields = props.fields.map((field) => {
    return (
      <CreateFormField
        key={field.label}
        name={field.name}
        label={field.label}
        noun={noun}
        control={form.control}
        t={t}
      />
    )
  })

  return (
    <DialogContent>
      <DialogHeader>
        <DialogTitle>{t('new_noun', { noun })}</DialogTitle>
      </DialogHeader>
      <Form {...form}>
        <form className="space-y-8 pt-4" onSubmit={form.handleSubmit(onSubmit)}>
          {fields}
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

export interface Entity {
  uuid: string
}

export type CreationFunction<CreatedEntity extends Entity, ValueType> = (
  values: ValueType,
) => Promise<CreatedEntity>

export interface CreateFormFieldProps<FormValues extends FieldValues> {
  name: FieldValue<FormValues>
  label: string
  noun: string
  control: Control<FormValues>
  t: TFunction
}

export function CreateFormField<FormValues extends FieldValues>(
  props: CreateFormFieldProps<FormValues>,
) {
  const { t } = props
  return (
    <FormField
      control={props.control}
      name={props.name}
      render={({ field }) => (
        <FormItem>
          <FormLabel>{t(props.label)}</FormLabel>
          <FormControl>
            <Input
              placeholder={t('noun_property', {
                noun: props.noun,
                property: t(props.label),
              })}
              {...field}
            />
          </FormControl>
        </FormItem>
      )}
    />
  )
}
