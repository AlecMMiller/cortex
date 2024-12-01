import { createFileRoute, Link } from '@tanstack/react-router'
import { useAvailableSchemas } from '@/commands/objects'
import { useQueryClient } from '@tanstack/react-query'
import { Plus } from 'lucide-react'
import { NavButton } from '@/components/ui/nav-button'
import { CreateSchemaDialog } from '@/components/dialogs/CreateSchema'
import { useTranslation } from 'react-i18next'

export const Route = createFileRoute('/schemas/')({
  component: RouteComponent,
})

interface SchemaLinkProps {
  uuid: string
  name: string
}

function SchemaLink(props: SchemaLinkProps) {
  return (
    <Link
      to={`/schemas/$uuid`}
      params={{ uuid: props.uuid }}
      className="p-2 text-blue bg-surface1 rounded-lg"
      key={props.uuid}
    >
      {props.name}
    </Link>
  )
}

function RouteComponent() {
  const { data: schemas, status } = useAvailableSchemas({})
  const { t } = useTranslation()
  const queryClient = useQueryClient()

  if (status !== 'success') {
    console.log(`Cache miss for schema list`)
    return <div>Loading</div>
  }

  const schemaLinks = schemas.map((schema) => (
    <SchemaLink key={schema.uuid} uuid={schema.uuid} name={schema.name} />
  ))

  return (
    <div className="m-4 flex h-fit gap-4">
      {schemaLinks}
      <NavButton
        queryClient={queryClient}
        icon={Plus}
        tooltip={t('new_noun', { noun: t('Schema', { count: 1 }) })}
        DialogContent={CreateSchemaDialog}
      />
    </div>
  )
}
